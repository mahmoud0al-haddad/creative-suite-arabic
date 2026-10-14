param([string]$SourceRoot=(Split-Path -Parent $PSScriptRoot),[switch]$Ci,[switch]$Plan,[string]$CargoHome,[string]$RustupHome)
$ErrorActionPreference='Stop'
$taskScript=Join-Path $PSScriptRoot 'build.ps1'
$taskRepos=@('craft-launcher','cadcraft','deckcraft','designcraft','effectcraft','filmcraft','gridcraft','lightcraft','pdfcraft','photocraft','soundcraft','vectorcraft','wordcraft')
foreach ($taskRepo in $taskRepos) {
    # Fresh child shell: build helper's process environment stays scoped to one invocation.
    $taskCmd=@('-NoProfile','-File',$taskScript,'-Repo',$taskRepo,'-SourceRoot',$SourceRoot)
    if ($Plan) {$taskCmd+='-Plan'};if ($Ci) {$taskCmd+=@('-Action','Ci')}
    if ($CargoHome) {$taskCmd+=@('-CargoHome',$CargoHome)};if ($RustupHome) {$taskCmd+=@('-RustupHome',$RustupHome)}
    & (Join-Path $PSHOME 'pwsh.exe') @taskCmd
    if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
}
