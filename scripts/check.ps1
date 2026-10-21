# Runs fmt, clippy, tests and docs, with all features so the schema code is covered.
# Usage, from the repo root: powershell -ExecutionPolicy Bypass -File scripts/check.ps1
#
# The crate is platform-independent with no unsafe code, so there's no per-target loop.

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
