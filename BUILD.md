# Build the pinned Windows x64 edition

## Layout and inputs

Run from the public repository root:

```text
sources/craft-launcher/
sources/<application>/
build-inputs/craft-fonts/
build-inputs/aws-lc-sys-0.45.0/
scripts/build.ps1
source-manifest.json
source-checksums.sha256
source-pins.json
```

The exported sources already contain modifications and vendored egui/epaint. Do not apply the historical patches a second time. `source-manifest.json` records upstream revisions and export changes; `source-checksums.sha256` records the actual exported files. Preserve every `Cargo.lock`. Exact upstream and font revisions are also in `source-pins.json`.

## Prerequisites

- Windows x64 with PowerShell 7, Git, and Python 3.11 or newer for verification/packaging scripts.
- Rust **1.99.0**, the reviewed compiler (`b940084d7`, 2026-09-28), MSVC target.
- Visual Studio C++ tools **14.44.35207**, Windows SDK **10.0.26100.0** for the closest reviewed build. Install and use those tools under their own Microsoft terms. CMake and **NASM** are required for the public PdfCraft source build: upstream licensed prebuilt NASM object fallbacks are intentionally excluded from the source-only export. The personal build used those upstream fallback objects when NASM was unavailable.
- Inno Setup 6 for a public Setup. Record its exact version in the release's build record.
- Network access to obtain pinned Cargo dependencies on the first build. Keep the font input pinned and verify its manifest hashes; never substitute a proprietary font.

```powershell
rustup toolchain install 1.99.0-x86_64-pc-windows-msvc --profile minimal --component rustfmt --component clippy
rustup target add wasm32-unknown-unknown --toolchain 1.99.0-x86_64-pc-windows-msvc
python scripts/check-source.py
pwsh -NoProfile -File scripts/build.ps1 -Repo effectcraft -Plan
pwsh -NoProfile -File scripts/build.ps1 -Repo effectcraft -Action Ci
pwsh -NoProfile -File scripts/build.ps1 -Repo effectcraft
```

`build.ps1` discovers Visual Studio with `vswhere` and the Windows SDK from Installed Roots. It requires the pins unless `-AllowDifferentTools` is explicitly selected. Overrides `-VsPath`, `-SdkRoot`, `-MsvcVersion`, `-SdkVersion`, `-FontsDir`, `-TargetDir`, `-CargoHome`, and `-RustupHome` avoid any dependency on a particular user's drive or profile. Alternate tools are recorded and require their own validation; they are not automatically the reviewed binary build.

For portable Cargo/Rustup caches, pass your own paths with `-CargoHome` and `-RustupHome`. For a fully populated offline cache add `-Offline`; that flag does not download a missing compiler or crate. `-Plan` validates source/fonts/tools and prints the planned command without running Cargo or writing a build record. Production release builds use `cargo build --locked --release --target x86_64-pc-windows-msvc`.

Outputs are `.build/<repo>/x86_64-pc-windows-msvc/release/<application>.exe`; the launcher's executable retains the machine filename `artcraft-launcher.exe`. Build metadata is in `.build/records/`, which is local-only and must not be uploaded without path sanitization.

```powershell
pwsh -NoProfile -File scripts/build-all.ps1 -Ci
pwsh -NoProfile -File scripts/build-all.ps1
```

Run repositories sequentially when using a shared target directory. CI uses a separate `.build/xtask/<repo>` for every `xtask.exe` to prevent stale executable collisions. Default jobs and Rust test threads are both two. Building all workspaces requires substantial disk/cache space; no universal time or disk-size guarantee is made.

LightCraft's complete native UI test configuration additionally exercises product model features:

```powershell
pwsh -NoProfile -File scripts/build.ps1 -Repo lightcraft -Action Cargo -CargoArgs @('test','--locked','-p','lightcraft-ui-egui','--features','lightcraft-engine/denoise,lightcraft-engine/sam','--lib','--','--test-threads=2')
```

## PdfCraft short-path input

The exported PdfCraft `Cargo.toml` points at `../../build-inputs/aws-lc-sys-0.45.0`. Its retained source comes from the Cargo-verified crate with its Apache/ISC notices; prebuilt `.obj` fallback files are intentionally excluded and recorded in the export manifest. This is not a byte-for-byte complete crate copy. NASM builds the retained assembly sources instead. The short-path placement works around a Windows compiler long-include-path failure; cryptographic source is unchanged. Keep the checkout near a drive root if Windows path length becomes an issue. Verify retained files against recorded hashes. Do not bundle MSVC or the Windows SDK themselves.

## Release verification

Run each workspace's CI plus its Arabic catalog/placeholder, bidi, editing and native workflow tests. Compare public source hashes before builds. Inspect Arabic/English screenshots at narrow sizes and mixed-text inputs. Exercise real launcher Open buttons for all twelve installed applications, verify independent installed paths, test Arabic filenames and document save/reopen, and verify required runtime modules on a clean Windows machine. Optional gates must be explicitly recorded as passed, failed or skipped.

The scripts pin input/tool versions and use Cargo lockfiles; they do not claim byte-for-byte deterministic output across different machines, SDKs, signatures or build paths. Publish SHA-256 for the actual released Setup and matching source archive.

## Public Windows packaging

Use [packaging/public-setup.iss](packaging/public-setup.iss) and its prerequisite include with a **separate public payload copy**. Never re-use the personal Setup unchanged. See [WINDOWS-RUNTIME.md](WINDOWS-RUNTIME.md) and [RELEASING.md](RELEASING.md). These scripts do not publish, sign, upload or install anything automatically.

## Verify source before building

With Python 3.11 or newer, run `python scripts/verify-source.py` from the repository root. This verifies every exported source/input byte hash and all thirteen upstream pins. After intentional source edits, keep the original release manifest as provenance and prepare a new reviewed manifest for the next release.
