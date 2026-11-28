# MyLocalApp — double-click via start.bat, or run this directly.
$ErrorActionPreference = "Stop"
Set-Location -LiteralPath $PSScriptRoot
$exe = ".\target\debug\mylocalapp.exe"
if (-not (Test-Path $exe)) {
    Write-Host "First run: building (one time, a few minutes)..."
    cargo build
}
& $exe
