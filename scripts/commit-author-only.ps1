# Create a commit without Cursor co-author trailers.
# Usage:
#   .\scripts\commit-author-only.ps1 -Subject "Short summary" -Body "Optional body"
param(
  [Parameter(Mandatory = $true)][string]$Subject,
  [string]$Body = ""
)

$ErrorActionPreference = "Stop"
Set-Location (Resolve-Path (Join-Path $PSScriptRoot "..")).Path

git add -A

$msgFile = Join-Path ".git" "COMMIT_MSG_AUTHOR_ONLY.txt"
if ($Body) {
  Set-Content -Path $msgFile -Value ($Subject + [Environment]::NewLine + [Environment]::NewLine + $Body)
} else {
  Set-Content -Path $msgFile -Value $Subject
}

$tree = git write-tree
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$parent = git rev-parse "HEAD~1" 2>$null
$git = "C:\Program Files\Git\cmd\git.exe"
if ($LASTEXITCODE -eq 0 -and $parent) {
  $new = & $git commit-tree $tree -p $parent -F $msgFile
} else {
  $new = & $git commit-tree $tree -F $msgFile
}

if (-not $new -or $new -match '\s') {
  Write-Error "commit-tree failed: $new"
}

git reset --hard $new
Remove-Item -Force $msgFile -ErrorAction SilentlyContinue
Write-Output "Created commit $new"
git log -1 --format=fuller
