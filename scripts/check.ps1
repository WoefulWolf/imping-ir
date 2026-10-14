# Runs the checks from the definition of done: formatting, clippy, tests and docs.
# Usage, from the repo root: powershell -ExecutionPolicy Bypass -File scripts/check.ps1
#
# This crate is platform-independent and has no unsafe code, so there is no
# per-target loop; `--all-features` exercises the optional schema path.

$ErrorActionPreference = 'Stop'
Set-Location (Split-Path -Parent $PSScriptRoot)

function Invoke-Step([string]$name, [scriptblock]$command) {
    Write-Host "==> $name" -ForegroundColor Cyan
    & $command
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAILED: $name" -ForegroundColor Red
        exit $LASTEXITCODE
    }
}

Invoke-Step 'cargo fmt --check' { cargo fmt --all --check }
Invoke-Step 'clippy' { cargo clippy --all-targets --all-features -- -D warnings }
Invoke-Step 'test' { cargo test --all-features }

$env:RUSTDOCFLAGS = '-D warnings'
Invoke-Step 'cargo doc' { cargo doc --no-deps --all-features }

Write-Host 'All checks passed.' -ForegroundColor Green
