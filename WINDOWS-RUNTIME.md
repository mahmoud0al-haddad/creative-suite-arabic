# Microsoft runtime and public distribution

Reviewed 2026-10-13 against Microsoft's own licensing/deployment pages. This documents the release decision; it does not grant Microsoft redistribution rights.

The personal installer used ten original signed x64 Microsoft CRT DLLs version **14.44.35211.0**, alongside the reviewed GUI binaries built with MSVC **14.44.35207**. Its existing notice correctly says applicable redistribution rights are required. Authenticode verifies the publisher and file integrity; it does not establish the suite distributor's license entitlement.

Microsoft's [latest-supported runtime page](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist) limits redistribution to licensed Visual Studio users and requires a runtime at least as recent as the build tools. The [Visual Studio 2022 distributable list](https://learn.microsoft.com/en-us/visualstudio/releases/2022/redistribution) makes distribution of release files and the complete redistributable subject to the applicable license. The actual [Runtime license](https://visualstudio.microsoft.com/license-terms/vs2022-cruntime/) and [Build Tools license](https://visualstudio.microsoft.com/license-terms/vs2022-ga-diagnosticbuildtools/) must be consulted for the distributor's own edition and rights. No Visual Studio license entitlement was established as part of this project review.

## Chosen public route

The public Setup **omits Microsoft CRT files and the redistributable installer**. The user obtains and installs Microsoft's x64 v14 runtime independently from the [official Microsoft download page](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist). This avoids relying on unverified rights to redistribute either the DLLs or `vc_redist.x64.exe`.

The public installer checks `HKLM\SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64` in both 64-bit and 32-bit machine registry views. It requires `Installed=1` and version **14.44.35207.0 or later within major version 14**, using the Version string or Major/Minor/Bld/Rbld values. An x86 runtime key does not satisfy it. Newer v14 minor/build versions are accepted. There is no registry write or automatic elevated install.

When missing or too old, normal Setup offers an Arabic/English choice to open Microsoft's page. Setup stays blocked while the user installs the prerequisite, then rechecks on Retry. Silent Setup fails with an explanatory message and never opens a browser. Installing the Microsoft prerequisite may require administrator rights even though the suite installer itself is per-user.

Use `scripts/check-runtime.ps1` for a read-only local report. `packaging/runtime-prerequisite.iss` is the complete Inno implementation. `packaging/public-setup.iss` also excludes the ten DLL names anywhere under application folders as an additional guard:

```text
concrt140.dll
msvcp140.dll
msvcp140_1.dll
msvcp140_2.dll
msvcp140_atomic_wait.dll
msvcp140_codecvt_ids.dll
vccorlib140.dll
vcruntime140.dll
vcruntime140_1.dll
vcruntime140_threads.dll
```

This runtime threshold applies to the currently reviewed binaries. A future rebuild with newer MSVC must raise the minimum to the toolset actually used and retest clean-machine startup. A registry check confirms registration/version, not the integrity of every runtime file; a corrupt installation may still require Microsoft's repair process. Do not mutate system runtime keys for testing. Exercise missing/older/sufficient registry cases through the isolated installer test harness or a disposable VM.

Do not publish the personal app-local-runtime Setup under the public release name. Changing compression or adding a notice alone does not resolve unverified redistribution entitlement.
