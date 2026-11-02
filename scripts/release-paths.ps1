# Remove build-machine paths from the independent edition's published executables.
# Dot-source within the build process after setting portable Cargo/Rustup homes.
function Set-CreativeSuiteReleasePaths {
    param([Parameter(Mandatory)][string]$WorkspaceRoot)
    $taskProfile=[Environment]::GetFolderPath('UserProfile')
    $taskCargo=if ($env:CARGO_HOME) {$env:CARGO_HOME} else {Join-Path $taskProfile '.cargo'}
    $taskRustup=if ($env:RUSTUP_HOME) {$env:RUSTUP_HOME} else {Join-Path $taskProfile '.rustup'}
    $taskMappings=@(
        @{From=$taskProfile;To='/user-home'},
        @{From=[IO.Path]::GetFullPath($WorkspaceRoot);To='/creative-suite'},
        @{From=[IO.Path]::GetFullPath($taskCargo);To='/cargo-sources'},
        @{From=[IO.Path]::GetFullPath($taskRustup);To='/rust-toolchains'}
    )
    $taskFlags=[Collections.Generic.List[string]]::new()
    foreach ($taskMapping in $taskMappings) {
        foreach ($taskPrefix in (@($taskMapping.From.Replace('/','\'),$taskMapping.From.Replace('\','/')) | Select-Object -Unique)) {
            $taskFlags.Add("--remap-path-prefix=$taskPrefix=$($taskMapping.To)")
            if ($taskPrefix.Length -gt 1 -and $taskPrefix[1] -eq ':') {
                $taskFlags.Add("--remap-path-prefix=$($taskPrefix.Substring(0,1).ToLower())$($taskPrefix.Substring(1))=$($taskMapping.To)")
            }
        }
    }
    # Encoded arguments preserve paths containing spaces and Arabic characters.
    $env:CARGO_ENCODED_RUSTFLAGS=$taskFlags -join [char]0x1f
    $env:CARGO_PROFILE_RELEASE_STRIP='symbols'
}

function Set-CreativeSuiteAwsLcReleasePaths {
    param([Parameter(Mandatory)][string]$WorkspaceRoot)
    $env:CC_SHELL_ESCAPED_FLAGS='1'
    $taskNativeFlags=[Collections.Generic.List[string]]::new()
    $taskNativeFlags.Add('/experimental:deterministic')
    foreach ($taskNativeMapping in @(
        @{From=[IO.Path]::GetFullPath($WorkspaceRoot);To='/creative-suite'},
        @{From=[Environment]::GetFolderPath('UserProfile');To='/user-home'}
    )) {
        foreach($taskNativePrefix in (@($taskNativeMapping.From.Replace('/','\'),$taskNativeMapping.From.Replace('\','/'))|Select-Object -Unique)) {
            $taskNativeFlags.Add('"/pathmap:'+$taskNativePrefix+'='+$taskNativeMapping.To+'"')
        }
    }
    $env:AWS_LC_SYS_CFLAGS=(@($env:AWS_LC_SYS_CFLAGS)+@($taskNativeFlags)) -join ' '
}
