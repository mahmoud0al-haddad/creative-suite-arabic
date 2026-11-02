param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string]$Compiler='C:\Program Files (x86)\Inno Setup 6\ISCC.exe'
)
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskOutput=[IO.Path]::GetFullPath((Join-Path $OutputDirectory ([Guid]::NewGuid().ToString('N'))))
[void][IO.Directory]::CreateDirectory($taskOutput)
$taskReport=Join-Path $taskOutput 'result.txt'
$taskHarness=Join-Path $taskRoot 'tests/suite-update-mode-harness.iss'
$taskHelper=Join-Path $taskRoot 'packaging/suite-update-mode.iss'
& $Compiler "/DHarnessOutput=$taskOutput" ("/DHarnessReport="+$taskReport.Replace("'","''")) $taskHarness *> (Join-Path $taskOutput 'compile.log')
if($LASTEXITCODE -ne 0){throw 'Update-mode harness compilation failed'}
$taskResults=@()
foreach($taskCase in @(
    @{name='manual';arguments=@();offer=1},
    @{name='update';arguments=@('/SUITEUPDATE=1');offer=0},
    @{name='explicit-manual';arguments=@('/SUITEUPDATE=0');offer=1},
    @{name='empty';arguments=@('/SUITEUPDATE=');offer=1},
    @{name='leading-zero';arguments=@('/SUITEUPDATE=01');offer=1},
    @{name='unknown';arguments=@('/SUITEUPDATE=unknown');offer=1}
)){
    if(Test-Path -LiteralPath $taskReport){throw 'Stale update-mode report'}
    $taskArguments=@('/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART',('/EXPECTOFFER='+$taskCase.offer))+$taskCase.arguments
    $taskRun=Start-Process -FilePath (Join-Path $taskOutput 'suite-update-mode-qa.exe') -ArgumentList $taskArguments -WindowStyle Hidden -Wait -PassThru
    # InitializeSetup stops initialization after the production predicate has run.
    if($taskRun.ExitCode -ne 1){throw "Unexpected update-mode exit: $($taskRun.ExitCode)"}
    if((Get-Content -LiteralPath $taskReport -Raw).Trim() -ne 'PASS launch ownership'){throw 'Update-mode check failed'}
    Move-Item -LiteralPath $taskReport -Destination (Join-Path $taskOutput ($taskCase.name+'.txt'))
    $taskResults+=@{case=$taskCase.name;finishPageOffersLauncher=([bool]$taskCase.offer);passed=$true}
}
@{helperSha256=(Get-FileHash -LiteralPath $taskHelper -Algorithm SHA256).Hash.ToLowerInvariant();scope='Actual Inno production launch predicate; no installation starts. Final Setup and real GitHub update still require integration review.';results=$taskResults} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $taskOutput 'verified.json') -Encoding utf8NoBOM
Write-Output "Update mode: six actual Inno predicate cases passed. Evidence: $taskOutput"
