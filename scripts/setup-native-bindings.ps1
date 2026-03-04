param(
    [string]$VenvPath = ".venv",
    [string]$BindingsRoot = "generated/bindings/python",
    [string]$Python = ""
)

$ErrorActionPreference = "Stop"

function Invoke-Checked {
    param([scriptblock]$Command)
    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "External command failed with exit code $LASTEXITCODE"
    }
}

function Resolve-Python {
    param([string]$Requested)
    if ($Requested -and (Get-Command $Requested -ErrorAction SilentlyContinue)) {
        return $Requested
    }
    if (Get-Command python -ErrorAction SilentlyContinue) {
        return "python"
    }
    if (Get-Command py -ErrorAction SilentlyContinue) {
        return "py -3"
    }
    throw "Python was not found in PATH. Install Python 3.9+ first."
}

$pythonCmd = Resolve-Python -Requested $Python
Write-Host "Using Python launcher: $pythonCmd"

if (-not (Test-Path $VenvPath)) {
    Write-Host "Creating virtual environment at $VenvPath..."
    Invoke-Expression "$pythonCmd -m venv `"$VenvPath`""
}

$venvPython = Join-Path $VenvPath "Scripts/python.exe"
if (-not (Test-Path $venvPython)) {
    throw "Virtual environment Python not found at $venvPython"
}
$venvPython = (Resolve-Path $venvPython).Path

Write-Host "Installing/Updating build tools in venv..."
& "$venvPython" -m pip install --upgrade pip setuptools wheel maturin

$bindingsAbs = (Resolve-Path $BindingsRoot).Path
Push-Location $bindingsAbs
try {
    Write-Host "Building and installing engpy_native via maturin..."
    Invoke-Checked { & "$venvPython" -m maturin develop --manifest-path "../../../crates/eng-pyext/Cargo.toml" }
} finally {
    Pop-Location
}

Write-Host "Verifying native runtime activation..."
$verifyScript = Join-Path ([System.IO.Path]::GetTempPath()) "engpy_native_verify.py"
@"
import sys
sys.path.insert(0, r'''$bindingsAbs''')
import engpy_native
import engpy
import engpy._runtime as rt
print("engpy_native runtime_info:", engpy_native.runtime_info())
print("engpy runtime_mode:", rt.runtime_mode())
if rt.runtime_mode() != "native":
    raise SystemExit("Expected native runtime mode after setup.")
print("OK: native runtime is active.")
"@ | Set-Content $verifyScript
try {
    Invoke-Checked { & "$venvPython" $verifyScript }
} finally {
    Remove-Item $verifyScript -ErrorAction SilentlyContinue
}

Write-Host "Native bindings setup complete."
