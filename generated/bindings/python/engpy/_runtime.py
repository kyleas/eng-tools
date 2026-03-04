import json
import os
import subprocess
import threading
import uuid
import atexit
import builtins

PROTOCOL_VERSION = "eng-invoke.v1"


class EngBindingError(RuntimeError):
    def __init__(self, code, message, op=None, field=None, request_id=None, detail=None):
        super().__init__(f"[{code}] {message}")
        self.code = code
        self.message = message
        self.op = op
        self.field = field
        self.request_id = request_id
        self.detail = detail


def _load_native_runtime():
    try:
        import engpy_native  # type: ignore
        return engpy_native
    except Exception:
        return None


_NATIVE_RUNTIME = _load_native_runtime()


class _NativeClient:
    def __init__(self, native_mod):
        self._native = native_mod
        self._request_count = 0
        self._last_failure = None
        self._last_request_id = None

    def _stop(self):
        # No-op for in-process runtime.
        return None

    def worker_pid(self):
        return None

    def stats(self):
        return {
            "runtime_mode": "native",
            "worker_pid": None,
            "startup_count": 0,
            "restart_count": 0,
            "request_count": self._request_count,
            "last_reused": True,
            "last_request_id": self._last_request_id,
            "last_failure": self._last_failure,
        }

    def invoke(self, op: str, args: dict, request_id=None):
        req_id = request_id or str(uuid.uuid4())
        req = {"protocol_version": PROTOCOL_VERSION, "op": op, "request_id": req_id, "args": args}
        self._request_count += 1
        self._last_request_id = req_id
        try:
            raw = self._native.invoke_json(json.dumps(req))
            data = json.loads(raw)
            self._last_failure = None
            return _validate_response(op, req_id, data)
        except EngBindingError:
            raise
        except Exception as exc:
            self._last_failure = str(exc)
            raise EngBindingError(
                "invoke_native_failed",
                f"native invoke failed: {exc}",
                op=op,
                request_id=req_id,
            )


class _WorkerClient:
    def __init__(self):
        self._lock = threading.RLock()
        self._process = None
        self._startup_count = 0
        self._restart_count = 0
        self._request_count = 0
        self._last_failure = None
        self._last_reused = False
        self._last_request_id = None

    def _resolve_worker_bin(self):
        return os.environ.get("ENG_WORKER_BIN") or os.environ.get("ENG_BIN") or "eng"

    def _popen_kwargs(self):
        kwargs = {
            "stdin": subprocess.PIPE,
            "stdout": subprocess.PIPE,
            # keep stderr drained to avoid blocking on pipe saturation
            "stderr": subprocess.DEVNULL,
            "text": True,
            "bufsize": 1,
            "encoding": "utf-8",
            "errors": "replace",
        }
        if os.name == "nt":
            # Prevent flashing/visible command windows in Excel/xlOil usage.
            creationflags = getattr(subprocess, "CREATE_NO_WINDOW", 0)
            kwargs["creationflags"] = creationflags
            si = subprocess.STARTUPINFO()
            si.dwFlags |= subprocess.STARTF_USESHOWWINDOW
            kwargs["startupinfo"] = si
        return kwargs

    def _spawn(self):
        worker_bin = self._resolve_worker_bin()
        self._process = subprocess.Popen(
            [worker_bin, "worker"],
            **self._popen_kwargs(),
        )
        self._startup_count += 1

    def _ensure_running(self, count_restart=False):
        if self._process is None:
            self._spawn()
            return False
        if self._process.poll() is not None:
            if count_restart:
                self._restart_count += 1
            self._spawn()
            return False
        return True

    def _stop(self):
        with self._lock:
            if self._process is None:
                return
            try:
                if self._process.stdin:
                    self._process.stdin.close()
            except Exception:
                pass
            try:
                self._process.terminate()
            except Exception:
                pass
            self._process = None

    def stats(self):
        with self._lock:
            return {
                "worker_pid": self.worker_pid(),
                "startup_count": self._startup_count,
                "restart_count": self._restart_count,
                "request_count": self._request_count,
                "last_reused": self._last_reused,
                "last_request_id": self._last_request_id,
                "last_failure": self._last_failure,
            }

    def worker_pid(self):
        with self._lock:
            if self._process is None or self._process.poll() is not None:
                return None
            return self._process.pid

    def invoke(self, op: str, args: dict, request_id=None):
        with self._lock:
            req_id = request_id or str(uuid.uuid4())
            req = {"protocol_version": PROTOCOL_VERSION, "op": op, "request_id": req_id, "args": args}
            self._request_count += 1
            self._last_request_id = req_id

            last_exc = None
            for attempt in range(2):
                reused = self._ensure_running(count_restart=(attempt > 0))
                self._last_reused = reused
                try:
                    if self._process.stdin is None or self._process.stdout is None:
                        raise RuntimeError("worker stdio not available")
                    self._process.stdin.write(json.dumps(req) + "\n")
                    self._process.stdin.flush()
                    raw = self._process.stdout.readline()
                    if not raw:
                        raise RuntimeError("worker returned no response")
                    data = json.loads(raw)
                    self._last_failure = None
                    return _validate_response(op, req_id, data)
                except Exception as exc:
                    last_exc = exc
                    self._last_failure = str(exc)
                    self._stop()
            raise EngBindingError(
                "invoke_worker_failed",
                f"worker invocation failed after restart: {last_exc}",
                op=op,
                request_id=req_id,
            )


def _validate_response(op: str, request_id: str, data: dict):
    if data.get("protocol_version") != PROTOCOL_VERSION:
        raise EngBindingError(
            "protocol_version_mismatch",
            f"response protocol version '{data.get('protocol_version')}' does not match '{PROTOCOL_VERSION}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if data.get("op") != op:
        raise EngBindingError(
            "operation_mismatch",
            f"response op '{data.get('op')}' does not match request op '{op}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if data.get("request_id") and data.get("request_id") != request_id:
        raise EngBindingError(
            "request_id_mismatch",
            f"response request_id '{data.get('request_id')}' does not match '{request_id}'",
            op=op,
            request_id=data.get("request_id"),
        )
    if bool(data.get("ok", False)):
        return data.get("value")
    err = data.get("error") or {}
    raise EngBindingError(
        err.get("code", "invoke_error"),
        err.get("message", "unknown invoke error"),
        op=data.get("op", op),
        field=err.get("field"),
        request_id=data.get("request_id"),
        detail=err.get("detail"),
    )


def _select_client():
    runtime_pref = os.environ.get("ENGPY_RUNTIME", "").strip().lower()
    if runtime_pref not in {"", "native", "worker"}:
        raise EngBindingError(
            "invalid_runtime_preference",
            f"ENGPY_RUNTIME must be 'native' or 'worker', got '{runtime_pref}'",
        )
    if runtime_pref != "worker" and _NATIVE_RUNTIME is not None:
        return _NativeClient(_NATIVE_RUNTIME), "native"
    if runtime_pref == "native":
        raise EngBindingError(
            "native_runtime_unavailable",
            "ENGPY_RUNTIME=native requested but engpy_native could not be imported",
        )
    return _WorkerClient(), "worker"


if hasattr(builtins, "_ENGPY_CLIENT"):
    _CLIENT = builtins._ENGPY_CLIENT
    _RUNTIME_MODE = getattr(builtins, "_ENGPY_CLIENT_MODE", "worker")
else:
    _CLIENT, _RUNTIME_MODE = _select_client()
    builtins._ENGPY_CLIENT = _CLIENT
    builtins._ENGPY_CLIENT_MODE = _RUNTIME_MODE
atexit.register(_CLIENT._stop)


def runtime_mode():
    return _RUNTIME_MODE


def worker_pid():
    return _CLIENT.worker_pid()


def worker_stats():
    stats = _CLIENT.stats()
    stats["runtime_mode"] = _RUNTIME_MODE
    return stats


def stop_worker():
    _CLIENT._stop()


def invoke(op: str, args: dict, request_id=None):
    return _CLIENT.invoke(op, args, request_id=request_id)
