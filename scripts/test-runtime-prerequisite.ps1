param(
    [Parameter(Mandatory)][string]$OutputDirectory,
    [string]$Compiler='C:\Program Files (x86)\Inno Setup 6\ISCC.exe'
)
$ErrorActionPreference='Stop'
$taskRoot=Split-Path -Parent $PSScriptRoot
$taskRunId=[Guid]::NewGuid().ToString('N')
$taskOutput=[IO.Path]::GetFullPath((Join-Path $OutputDirectory $taskRunId))
[void][IO.Directory]::CreateDirectory($taskOutput)
$taskKey='Software\CreativeSuiteArabicQA\'+$taskRunId
$taskReport=Join-Path $taskOutput 'runtime-result.txt'
$taskHarness=Join-Path $taskRoot 'tests/runtime-prerequisite-harness.iss'
$taskHelper=Join-Path $taskRoot 'packaging/runtime-prerequisite.iss'
$taskPascalReport=$taskReport.Replace("'","''")
& $Compiler "/DHarnessOutput=$taskOutput" "/DHarnessReport=$taskPascalReport" "/DHarnessKey=$taskKey" $taskHarness *> (Join-Path $taskOutput 'compile.log')
if($LASTEXITCODE -ne 0){throw 'Runtime prerequisite harness compilation failed'}
$taskExe=Join-Path $taskOutput 'runtime-prerequisite-qa.exe'
$taskResults=@()
foreach($taskLanguage in @('ar','en')){
    if(Test-Path -LiteralPath $taskReport){throw 'A stale harness report would invalidate the next language test'}
    $taskRun=Start-Process -FilePath $taskExe -ArgumentList @('/VERYSILENT','/SUPPRESSMSGBOXES','/NORESTART',"/LANG=$taskLanguage") -WindowStyle Hidden -Wait -PassThru
    # InitializeSetup deliberately returns False after writing the report; no install starts.
    # Inno exit 1 means initialization stopped, whereas exit 2 is a user's Cancel action.
    # https://jrsoftware.org/ishelp/topic_setupexitcodes.htm
    if($taskRun.ExitCode -ne 1){throw "Unexpected harness exit code: $($taskRun.ExitCode)"}
    $taskText=Get-Content -LiteralPath $taskReport -Raw -Encoding UTF8
    if($taskText -notmatch 'CHECKS=(\d+)' -or [int]$Matches[1] -ne 43 -or $taskText -match 'FAIL'){throw 'Runtime prerequisite checks did not all pass'}
    if(Test-Path -LiteralPath ('HKCU:\'+$taskKey)){throw 'Isolated registry fixtures were not removed'}
    $taskLanguageReport=Join-Path $taskOutput "runtime-result-$taskLanguage.txt"
    Move-Item -LiteralPath $taskReport -Destination $taskLanguageReport
    $taskResults+=@{language=$taskLanguage;checks=43;noInstallationStarted=$true;isolatedHkcuFixturesRemoved=$true;reportSha256=(Get-FileHash -LiteralPath $taskLanguageReport -Algorithm SHA256).Hash.ToLowerInvariant()}
    Write-Output "Runtime prerequisite: 43 evaluator checks passed ($taskLanguage); no installation started."
}
@{runtimeHelperSha256=(Get-FileHash -LiteralPath $taskHelper -Algorithm SHA256).Hash.ToLowerInvariant();harnessSha256=(Get-FileHash -LiteralPath $taskHarness -Algorithm SHA256).Hash.ToLowerInvariant();machineRuntimeRegistryModified=$false;scope='Production Pascal version/registry evaluator with isolated HKCU fixtures and actual-machine silent PrepareToInstall; not a clean-machine missing-runtime installation';results=$taskResults} | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath (Join-Path $taskOutput 'verified.json') -Encoding utf8NoBOM
Write-Output "Evidence: $taskOutput"
