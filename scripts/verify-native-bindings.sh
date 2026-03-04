#!/usr/bin/env bash
set -euo pipefail

VENV_PATH="${VENV_PATH:-.venv}"
BINDINGS_ROOT="${BINDINGS_ROOT:-generated/bindings/python}"

VENV_PYTHON="$VENV_PATH/bin/python"
if [[ ! -x "$VENV_PYTHON" ]]; then
  echo "Virtual environment python not found at $VENV_PYTHON" >&2
  exit 1
fi

BINDINGS_ABS="$(cd "$BINDINGS_ROOT" && pwd)"
"$VENV_PYTHON" - <<PY
import sys
sys.path.insert(0, r"$BINDINGS_ABS")
import engpy_native
import engpy
import engpy._runtime as rt
print("runtime_mode:", rt.runtime_mode())
print("worker_stats:", rt.worker_stats())
print("runtime_info:", engpy_native.runtime_info())
if rt.runtime_mode() != "native":
    raise SystemExit("Native runtime is not active.")
PY
