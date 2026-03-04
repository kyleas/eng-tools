#!/usr/bin/env bash
set -euo pipefail

VENV_PATH="${VENV_PATH:-.venv}"
BINDINGS_ROOT="${BINDINGS_ROOT:-generated/bindings/python}"
PYTHON_BIN="${PYTHON_BIN:-python3}"

if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  if command -v python >/dev/null 2>&1; then
    PYTHON_BIN="python"
  else
    echo "Python 3.9+ is required but was not found in PATH." >&2
    exit 1
  fi
fi

echo "Using Python launcher: $PYTHON_BIN"

if [[ ! -d "$VENV_PATH" ]]; then
  echo "Creating virtual environment at $VENV_PATH..."
  "$PYTHON_BIN" -m venv "$VENV_PATH"
fi

VENV_PYTHON="$VENV_PATH/bin/python"
if [[ ! -x "$VENV_PYTHON" ]]; then
  echo "Virtual environment python not found at $VENV_PYTHON" >&2
  exit 1
fi

echo "Installing/Updating build tools in venv..."
"$VENV_PYTHON" -m pip install --upgrade pip setuptools wheel maturin

BINDINGS_ABS="$(cd "$BINDINGS_ROOT" && pwd)"
pushd "$BINDINGS_ABS" >/dev/null
echo "Building and installing engpy_native via maturin..."
"$VENV_PYTHON" -m maturin develop --manifest-path "../../../crates/eng-pyext/Cargo.toml"
popd >/dev/null

echo "Verifying native runtime activation..."
"$VENV_PYTHON" - <<PY
import sys
sys.path.insert(0, r"$BINDINGS_ABS")
import engpy_native
import engpy
import engpy._runtime as rt
print("engpy_native runtime_info:", engpy_native.runtime_info())
print("engpy runtime_mode:", rt.runtime_mode())
if rt.runtime_mode() != "native":
    raise SystemExit("Expected native runtime mode after setup.")
print("OK: native runtime is active.")
PY

echo "Native bindings setup complete."
