param(
    [switch]$SkipBuild,
    [string[]]$Modes = @('inputs', 'parameters', 'parameters-weights', 'parameters-results', 'parameters-ab', 'parameters-mt', 'parameters-compact', 'parameters-scaled', 'results', 'pockets', 'heads', 'detail', 'detail-compact', 'detail-scaled', 'detail-bottom', 'detail-carry', 'file-menu', 'edit-menu', 'filters', 'reopened', 'unsaved', 'file-error', 'no-candidates', 'no-common', 'compact', 'scaled', 'many')
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$cargoPath = Join-Path $env:USERPROFILE '.cargo/bin/cargo.exe'
$previewDirectory = Join-Path $repoRoot 'work/ui-refinements'
$binaryPath = Join-Path $repoRoot 'target/debug/stick-optimizer-desktop.exe'

Push-Location $repoRoot
$previousMode = $env:STICK_OPTIMIZER_PREVIEW
$previousScreenshot = $env:STICK_OPTIMIZER_SCREENSHOT_TO
try {
    if (-not $SkipBuild) {
        & $cargoPath build -p stick-optimizer-desktop --features ui-preview --locked
        if ($LASTEXITCODE -ne 0) { throw 'Build della GUI di prova non riuscita.' }
    }
    New-Item -ItemType Directory -Path $previewDirectory -Force | Out-Null
    foreach ($mode in $Modes) {
        $screenshotPath = Join-Path $previewDirectory "$mode.png"
        $env:STICK_OPTIMIZER_PREVIEW = $mode
        $env:STICK_OPTIMIZER_SCREENSHOT_TO = $screenshotPath
        $startedAt = Get-Date
        $process = Start-Process -FilePath $binaryPath -WorkingDirectory $repoRoot -WindowStyle Hidden -PassThru
        if (-not $process.WaitForExit(60000)) {
            $process.Kill()
            throw "La prova $mode non è terminata entro 60 secondi."
        }
        # Refresh the exit code before checking the freshly generated image.
        $process.Refresh()
        if ($process.ExitCode -ne 0) { throw "La prova $mode ha restituito un errore: $($process.ExitCode)." }
        $file = Get-Item -LiteralPath $screenshotPath
        if ($file.LastWriteTime -lt $startedAt -or $file.Length -lt 1000) {
            throw "Screenshot mancante o non aggiornato per $mode."
        }
        Write-Output "Screenshot ${mode}: $screenshotPath"
    }
} finally {
    $env:STICK_OPTIMIZER_PREVIEW = $previousMode
    $env:STICK_OPTIMIZER_SCREENSHOT_TO = $previousScreenshot
    Pop-Location
}
