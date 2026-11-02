param(
    [Parameter(Mandatory)][ValidateSet('craft-launcher','vectorcraft','lightcraft','photocraft','wordcraft','designcraft','effectcraft','gridcraft','deckcraft','cadcraft','soundcraft','filmcraft','figcraft','pdfcraft')][string]$Repo,
    [ValidateSet('Release','Ci','Cargo')][string]$Action='Release',
    [string]$SourceRoot=(Split-Path -Parent $PSScriptRoot),
    [string]$FontsDir,
    [string]$TargetDir,
    [string]$RustupHome,
    [string]$CargoHome,
    [string]$VsPath,
    [string]$SdkRoot,
    [string]$MsvcVersion='14.44.35207',
    [string]$SdkVersion='10.0.26100.0',
    [string]$Toolchain='1.99.0-x86_64-pc-windows-msvc',
    [int]$Jobs=2,
    [switch]$AllowDifferentTools,
    [switch]$Offline,
    [switch]$Plan,
    [string[]]$CargoArgs=@()
)
$ErrorActionPreference='Stop'
Set-StrictMode -Version Latest
if ($Jobs -lt 1) { throw 'Jobs must be at least 1.' }
$taskRoot=[IO.Path]::GetFullPath($SourceRoot)
$taskRepo=Join-Path $taskRoot "sources/$Repo"
if (!(Test-Path -LiteralPath (Join-Path $taskRepo 'Cargo.toml'))) { throw "Missing source workspace: $taskRepo" }
if (!$FontsDir) { $FontsDir=Join-Path $taskRoot 'build-inputs/craft-fonts' }
$taskFonts=[IO.Path]::GetFullPath($FontsDir)
if (!(Test-Path -LiteralPath (Join-Path $taskFonts 'fonts/manifest.txt'))) { throw 'Pinned craft-fonts input is required; see BUILD.md.' }
foreach ($taskLine in (Get-Content -LiteralPath (Join-Path $taskFonts 'fonts/manifest.txt'))) {
    if (!$taskLine.Trim() -or $taskLine.StartsWith('#')) { continue }
    $taskParts=$taskLine.Split('|') | ForEach-Object {$_.Trim()}
    if ($taskParts.Count -ne 8) { throw 'Invalid font manifest row.' }
    $taskFontFile=[IO.Path]::GetFullPath((Join-Path $taskFonts $taskParts[2]))
    if (!$taskFontFile.StartsWith($taskFonts+[IO.Path]::DirectorySeparatorChar,[StringComparison]::OrdinalIgnoreCase)) { throw 'Font path escapes input directory.' }
    if ((Get-FileHash -LiteralPath $taskFontFile -Algorithm SHA256).Hash -ne $taskParts[6]) { throw "Font checksum mismatch: $($taskParts[2])" }
    if (!(Test-Path -LiteralPath (Join-Path $taskFonts $taskParts[5]))) { throw "Missing font license: $($taskParts[5])" }
}
if ($Repo -eq 'pdfcraft') {
    $taskAws=Join-Path $taskRoot 'build-inputs/aws-lc-sys-0.45.0'
    if (!(Test-Path -LiteralPath (Join-Path $taskAws 'Cargo.toml'))) { throw 'Missing source input aws-lc-sys 0.45.0; see BUILD.md.' }
    if (!$Plan) { Get-Command nasm,cmake -ErrorAction Stop | Out-Null }
}
# Optional portable Rust homes; no fixed user name or drive is assumed.
if ($RustupHome) { $env:RUSTUP_HOME=[IO.Path]::GetFullPath($RustupHome) }
if ($CargoHome) { $env:CARGO_HOME=[IO.Path]::GetFullPath($CargoHome); $env:PATH="$env:CARGO_HOME/bin;$env:PATH" }
if (!$VsPath) {
    $taskVswhere=Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
    if (!(Test-Path -LiteralPath $taskVswhere)) { throw 'Install Visual Studio C++ Build Tools, or pass -VsPath.' }
    $VsPath=(& $taskVswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath | Select-Object -First 1)
    if (!$VsPath) { throw 'No installation with x64 MSVC tools was found.' }
}
$taskMsvc=Join-Path $VsPath "VC/Tools/MSVC/$MsvcVersion"
if (!(Test-Path -LiteralPath (Join-Path $taskMsvc 'bin/Hostx64/x64/cl.exe'))) {
    if (!$AllowDifferentTools) { throw "Pinned MSVC $MsvcVersion unavailable. Pass -AllowDifferentTools to record an alternate toolset." }
    $taskMsvc=(Get-ChildItem -LiteralPath (Join-Path $VsPath 'VC/Tools/MSVC') -Directory | Sort-Object {[version]$_.Name} -Descending | Select-Object -First 1).FullName
}
if (!$SdkRoot) { $SdkRoot=(Get-ItemProperty -LiteralPath 'HKLM:\SOFTWARE\Microsoft\Windows Kits\Installed Roots').KitsRoot10 }
if (!(Test-Path -LiteralPath (Join-Path $SdkRoot "Lib/$SdkVersion/um/x64/kernel32.lib"))) {
    if (!$AllowDifferentTools) { throw "Pinned Windows SDK $SdkVersion unavailable." }
    $SdkVersion=(Get-ChildItem -LiteralPath (Join-Path $SdkRoot 'Lib') -Directory | Where-Object {Test-Path -LiteralPath (Join-Path $_.FullName 'um/x64/kernel32.lib')} | Sort-Object {[version]$_.Name} -Descending | Select-Object -First 1).Name
}
$env:PATH="$taskMsvc/bin/Hostx64/x64;$SdkRoot/bin/$SdkVersion/x64;$env:PATH"
$env:LIB="$taskMsvc/lib/x64;$SdkRoot/Lib/$SdkVersion/ucrt/x64;$SdkRoot/Lib/$SdkVersion/um/x64"
$env:INCLUDE="$taskMsvc/include;$SdkRoot/Include/$SdkVersion/ucrt;$SdkRoot/Include/$SdkVersion/shared;$SdkRoot/Include/$SdkVersion/um"
$env:CRAFT_FONTS_DIR=$taskFonts
$env:CARGO_BUILD_JOBS="$Jobs"
$env:RUST_TEST_THREADS='2'
$env:CARGO_INCREMENTAL='0'
$env:CARGO_PROFILE_DEV_DEBUG='0'
$env:CARGO_PROFILE_TEST_DEBUG='0'
if (!$TargetDir) { $TargetDir=Join-Path $taskRoot ".build/$Repo" }
$env:CARGO_TARGET_DIR=[IO.Path]::GetFullPath($TargetDir)
if ($Offline) { $env:CARGO_NET_OFFLINE='true' }
if ($Repo -eq 'craft-launcher') { $env:ARTCRAFT_LAUNCHER_REQUIRE_WINRES='1' }
$taskPackage=if ($Repo -eq 'craft-launcher') {'artcraft-launcher'} else {$Repo}
$taskArgs=switch ($Action) {
    'Release' {@('build','--locked','--release','--target','x86_64-pc-windows-msvc','-p',$taskPackage)}
    'Ci' {@('run','--locked','-p','xtask','--target-dir',(Join-Path $taskRoot ".build/xtask/$Repo"),'--','ci')}
    'Cargo' {if (!$CargoArgs.Count) {throw '-Action Cargo requires -CargoArgs.'};$CargoArgs}
}
if ($Action -eq 'Release' -or $taskArgs -contains '--release' -or $taskArgs -contains '-r') {
    . (Join-Path $PSScriptRoot 'release-paths.ps1')
    Set-CreativeSuiteReleasePaths -WorkspaceRoot $taskRoot
    if ($Repo -eq 'pdfcraft') { Set-CreativeSuiteAwsLcReleasePaths -WorkspaceRoot $taskRoot }
}
$taskPlan=[ordered]@{repo=$Repo;source=$taskRepo;toolchain=$Toolchain;msvc=(Split-Path $taskMsvc -Leaf);sdk=$SdkVersion;fonts=$taskFonts;target=$env:CARGO_TARGET_DIR;args=@($taskArgs);offline=[bool]$Offline}
if ($Plan) { $taskPlan | ConvertTo-Json -Depth 4; exit 0 }
Get-Command rustup,cargo,cl,link,rc -ErrorAction Stop | Out-Null
New-Item -ItemType Directory -Path (Join-Path $taskRoot '.build/records') -Force | Out-Null
$taskPlan | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $taskRoot ".build/records/$Repo-build.json") -Encoding utf8
Push-Location $taskRepo
try {
    & rustup run $Toolchain rustc --version
    if ($LASTEXITCODE -ne 0) { throw 'Install the pinned Rust toolchain as described in BUILD.md.' }
    & rustup run $Toolchain cargo @taskArgs
    $taskExit=$LASTEXITCODE
} finally { Pop-Location }
exit $taskExit
