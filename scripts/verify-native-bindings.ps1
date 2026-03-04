param(
    [string]$VenvPath = ".venv",
    [string]$BindingsRoot = "generated/bindings/python"
)

$ErrorActionPreference = "Stop"
function Invoke-Checked {
    param([scriptblock]$Command)
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "External command failed with exit code $LASTEXITCODE"
    }
}
$venvPython = Join-Path $VenvPath "Scripts/python.exe"
if (-not (Test-Path $venvPython)) {
    throw "Virtual environment Python not found at $venvPython"
}
$venvPython = (Resolve-Path $venvPython).Path

$bindingsAbs = (Resolve-Path $BindingsRoot).Path
$verifyScript = Join-Path ([System.IO.Path]::GetTempPath()) "engpy_native_verify.py"
@"
import sys
sys.path.insert(0, r'''$bindingsAbs''')
import engpy_native
import engpy
import engpy._runtime as rt
print("runtime_mode:", rt.runtime_mode())
print("worker_stats:", rt.worker_stats())
print("runtime_info:", engpy_native.runtime_info())
if rt.runtime_mode() != "native":
    raise SystemExit("Native runtime is not active.")
"@ | Set-Content $verifyScript
try {
    Invoke-Checked { & "$venvPython" $verifyScript }
} finally {
    Remove-Item $verifyScript -ErrorAction SilentlyContinue
}
