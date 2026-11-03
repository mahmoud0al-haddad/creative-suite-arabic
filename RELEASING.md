# Release workflow

Planned first public tag: **v2026.10-ar.1**. Expected Windows artifact: **Creative-Suite-Arabic-Windows-x64-Setup.exe**. Publish only from the maintainer account/project after review; none of these scripts publish or message other people.

1. Preserve the exported source tree, source manifest, checksum list, component notices and font manifest. Confirm no private/control configuration, system font, build cache or excluded brand graphics are included.
2. Build pinned sources with Cargo lockfiles and the recorded Rust/MSVC/SDK versions. Run the affected native/wasm/translation/RTL/document gates. The manual Windows matrix workflow helps verify alternative runner environments; it does not replace clean-machine installer review.
3. Prepare a **new public payload directory** from the reviewed bundle using `scripts/prepare-public-payload.py`. Supply a freshly audited public source ZIP, not the historical personal source ZIP. The assembler copies the exact reviewed GUI/support files, omits the ten CRT names everywhere, and creates a path-free review summary. Original stage and desktop Setup remain untouched.
4. Compile the public installer. Its independent AppId/install directory permits coexistence with the personal edition; it uses the same reviewed app binaries. Microsoft's runtime is a checked prerequisite, not a redistributed payload.

```powershell
python scripts/prepare-public-payload.py --reviewed-stage <reviewed-bundle> --destination <new-public-payload> --public-source-zip <audited-public-source.zip> --tag v2026.10-ar.1 --revision 1
& <ISCC.exe> /DPublicPayload=<new-public-payload> /DPublicOutput=<new-output> /DPublicVersion=2026.10-ar.1 packaging/public-setup.iss
Get-FileHash <new-output>/Creative-Suite-Arabic-Windows-x64-Setup.exe -Algorithm SHA256
```

5. Test missing/old/current runtime paths without editing system registry keys; use the isolated prerequisite harness or disposable VMs. Silent Setup with missing runtime must fail without a browser. Normal Setup offers Microsoft's page, remains blocked, and succeeds after the user independently installs a sufficient x64 runtime.
6. On clean Windows, install in Arabic and English; inspect fresh settings, preserved existing settings, relative manifest paths, installed hashes, all thirteen real Open buttons in both languages (26 launches), actual child language and representative Arabic-named document save/reopen. Confirm no packaged CRT DLLs and that executables use the installed Microsoft runtime. Recheck uninstallation only for the test instance.
7. Produce SHA256SUMS.txt for actual public artifacts, public source archive and an honest verification summary with tool versions and skipped checks. Update README/release notes around the shipped artifact; do not publish personal runtime claims as public runtime evidence.
8. Create the reviewed GitHub release and attach Setup, checksums, the matching source archive and the generated `arabic-suite-release.json` feed. Generate the feed from the final Setup's actual SHA-256/size and the installed component metadata; never edit hashes to make a failing check pass. The weekly watcher does not automatically perform any step of a release.
9. Exercise a real download from that published release using an isolated installed prior revision. Verify notification/manual discovery, size/SHA validation, visible Setup, preserved settings, installed component hashes, restart and successful result. Keep private test metadata separate from public release assets. Do not count a mocked process or local mirror as this end-to-end test.

Required runtime minimum must be raised if a newer MSVC build is used. Keep old releases tied to their original source pins and known limitations. Windows code signing, if later enabled, requires independent credentials and separate handling; no signing key belongs in source or logs.
