# Licenses, notices and independent branding

The thirteen upstream workspaces use **MIT OR Apache-2.0** for their own code. Their original copyright notices and full `LICENSE-MIT`/`LICENSE-APACHE` files remain in each workspace. Every third-party crate, vendored renderer, font and non-code asset retains its own terms. A top-level project license cannot replace these component notices.

New independent tooling and documentation in this repository are licensed **MIT OR Apache-2.0**, at your option, copyright © 2026 mahmoud0al-haddad as independent Arabic edition maintainer. The complete root [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE) texts apply to that new work. Preserve original authors' copyrights instead of relabeling their work as wholly authored by the maintainer. Existing component license files shipped with `sources/` and `build-inputs/` remain authoritative for their own code/assets; the root license does not relicense those components.

## Required release contents

- Preserve upstream license files, NOTICE, ATTRIBUTION/ASSETS documentation and attribution sidecars where supplied. MIT recipients need the copyright/permission notice; Apache distributions retain license/notice obligations and indicate modifications.
- Preserve complete egui/epaint licenses and fork provenance. Changed bidi, text layout, caret and selection code remains under its original licenses, with modification notes.
- Ship every used open font's OFL or other applicable license and attribution. `build-inputs/craft-fonts/fonts/manifest.txt` records the source and file hashes. Noto Sans Arabic has SIL OFL-1.1. No Windows Tahoma file is distributed, copied into the repo or embedded from the user's machine.
- Preserve aws-lc-sys and its nested AWS-LC/BoringSSL third-party licenses and source provenance. The short-path workaround changes build placement, not cryptographic behavior.
- Keep dependency and asset license reports current for the exact release. Optional skipped audits must remain visible; do not treat an unrun security audit as passed.
- Retain license texts governing upstream artwork even when excluded. The exported `BRAND-EXCLUSIONS.md` records removed marks. Independent packages use original neutral graphics/plain application names and do not claim an official ArtCraft build. App-specific animal artwork is retained only where its own attribution permits redistribution.

## Arabic translation credit

See [UPSTREAM.md](UPSTREAM.md) for original application authors and the Arabic draft contributions used as input. Those drafts were adapted and reviewed; EffectCraft's early word-order reversal was corrected to logical Unicode. Credit the input author and revision, keep existing notices, and identify later translation/renderer changes as modifications.

## Microsoft components

The public source/package route does not include Microsoft CRT DLLs, MSVC, the Windows SDK, proprietary fonts, or the official redistributable installer. Users/developers obtain prerequisites under Microsoft's own terms. Public Setup checks for an independently installed runtime as described in [WINDOWS-RUNTIME.md](WINDOWS-RUNTIME.md). The personal installer's existing app-local Microsoft files are not licensed by MIT/Apache and must not be copied into a public payload without established applicable rights.

## Before publishing

Compare source checksums, license sidecars and exported exclusions; verify no private documents/settings/control keys, compiled cache, unrelated marks or proprietary fonts enter the public repository. Keep the matching modified source and source manifest available with each binary release. Record the source/tool revisions and actual installer SHA-256. License notices explain rights and provenance; they do not certify every upstream asset or future dependency without review.
