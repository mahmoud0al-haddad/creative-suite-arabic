param([version]$MinimumVersion='14.44.35207.0')
$ErrorActionPreference='Stop'
$taskKey='SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64'
$taskMatches=@()
foreach ($taskView in @([Microsoft.Win32.RegistryView]::Registry64,[Microsoft.Win32.RegistryView]::Registry32)) {
    $taskBase=[Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine,$taskView)
    $taskRuntime=$null
    try {
        $taskRuntime=$taskBase.OpenSubKey($taskKey)
        if (!$taskRuntime -or $taskRuntime.GetValue('Installed',0) -ne 1) {continue}
        $taskValue=[string]$taskRuntime.GetValue('Version','')
        $taskParsed=$null
        if ([version]::TryParse($taskValue.TrimStart('v','V'),[ref]$taskParsed)) {
            if ($taskParsed.Revision -lt 0 -and $taskParsed.Build -ge 0) { $taskParsed=[version]::new($taskParsed.Major,$taskParsed.Minor,$taskParsed.Build,0) }
            $taskMatches+=@{view="$taskView";version="$taskParsed";sufficient=($taskParsed.Major -eq 14 -and $taskParsed -ge $MinimumVersion)}
        } else {
            $taskMajor=$taskRuntime.GetValue('Major',$null);$taskMinor=$taskRuntime.GetValue('Minor',$null);$taskBuild=$taskRuntime.GetValue('Bld',$null)
            if ($null -ne $taskMajor -and $null -ne $taskMinor -and $null -ne $taskBuild) {
                $taskParsed=[version]::new([int]$taskMajor,[int]$taskMinor,[int]$taskBuild,[int]$taskRuntime.GetValue('Rbld',0))
                $taskMatches+=@{view="$taskView";version="$taskParsed";sufficient=($taskParsed.Major -eq 14 -and $taskParsed -ge $MinimumVersion)}
            }
        }
    } finally {if ($taskRuntime) {$taskRuntime.Dispose()};$taskBase.Dispose()}
}
$taskOkay=([Environment]::Is64BitOperatingSystem -and @($taskMatches | Where-Object {$_.sufficient}).Count -gt 0)
@{architecture='x64';minimum="$MinimumVersion";registered=@($taskMatches);sufficient=$taskOkay;downloadPage='https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist'} | ConvertTo-Json -Depth 4
if (!$taskOkay) {exit 2}
