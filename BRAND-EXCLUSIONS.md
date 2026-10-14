# Trademark exclusions and source publication review

This independently maintained edition is based on the13 upstream Craft app repositories pinned in `source-manifest.json`. Arabic edition preparation and maintenance: **mahmoud0al-haddad**. Original authors retain all original copyrights. This is not an official or endorsed ArtCraft release.

The original ArtCraft name/wordmark/A-mark graphics and mark-derived launcher artwork are not distributed. Their unchanged `LICENSE-brand.txt` terms and factual plain-text upstream credits remain. The app-specific animal icons retain their independently attributed MIT/Apache licences. Neutral geometric launcher replacements are original edition artwork under MIT OR Apache-2.0.

No original code licence is changed. Full amended egui/epaint sources and their MIT/Apache licence notices are included. Optional open font inputs retain their original per-font licences, pinned sources and checksums. No Tahoma or other proprietary installed system font is bundled.

Git metadata, targets, downloaded executables/installers, user configs, control credentials, private snapshots and raw native-control reports are excluded. Standard upstream synthetic cryptographic test fixtures are licensed source fixtures, not user credentials.

The public AWS-LC input intentionally omits the upstream prebuilt NASM object fallback. NASM is required to build the retained assembly source; the input is not claimed to be an unmodified byte-for-byte crate archive. The standard plaintext Wavefront `.obj` mesh fixture is retained.

Public source metadata checks passed for all13 workspaces. All supported per-app asset/layer gates passed; Photo has no `xtask assets` command and is covered by the explicit resource/font/licence audit. Photo strict UI/native clippy and10 theme/font tests passed after its open Arabic fallback fix. The source audit verifies all literal include resources, original licence notices, pinned open-font checksums and absence of private user paths or credential patterns. These checks do not promise perfect upstream feature parity or freedom from future bugs.

Publication adaptations and per-file original/exported checksums are recorded in `source-manifest.json`. `source-checksums.sha256` covers only `sources/` and `build-inputs/`; `.git`, build outputs, website content and root documentation are deliberately outside that source-input index.

## Excluded originals

| Original resource | Reason |
|---|---|
| `sources/cadcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/cadcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/deckcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/deckcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/designcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/designcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/effectcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo-white.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo-white.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-logo.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark-black.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark-black.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/docs/brand/artcraft-mark.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/effectcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/avcc.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/baseline_lowqp_slices.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/baseline_speed.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/bframes.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/cbr.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/high_crf_b2.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/high_quality_b3_slices.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/high_speed_odd_size.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/main_b2.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/mandelbrot.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/psnr1080.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/testsrc2.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/twopass.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/crates/h264enc/target/agent-h264enc/fixtures/h264enc/oracle/vbr1pass.h264` | private metadata, build output or local corpus |
| `sources/filmcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo-white.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo-white.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-logo.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark-black.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark-black.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark.png.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/docs/brand/artcraft-mark.svg.attribution` | ArtCraft trademark artwork or artwork sidecar |
| `sources/filmcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/gridcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/gridcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/lightcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/lightcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/pdfcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/pdfcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/pdfcraft/vendor/hayro-interpret/.cargo-ok` | local/cache state |
| `sources/pdfcraft/vendor/hayro/.cargo-ok` | local/cache state |
| `sources/pdfcraft/vendor/winit/.cargo-ok` | local/cache state |
| `sources/photocraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/photocraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/soundcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/soundcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/vectorcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/vectorcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/wordcraft/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/wordcraft/vendor/egui/.cargo-ok` | local/cache state |
| `sources/craft-launcher/assets/app-icon/artcraft-launcher-1024.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/artcraft-launcher.icns` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/artcraft-launcher.ico` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/artcraft-launcher.svg` | byte-identical copy of ArtCraft trademark artwork; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/128x128/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/16x16/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/24x24/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/256x256/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/32x32/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/48x48/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/512x512/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/64x64/apps/ai.storyteller.artcraft-launcher.png` | unchanged upstream launcher mark or mark-derived asset; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/assets/app-icon/hicolor/scalable/apps/ai.storyteller.artcraft-launcher.svg` | byte-identical copy of ArtCraft trademark artwork; an independently licensed neutral replacement is present at this resource path |
| `sources/craft-launcher/docs/brand/artcraft-logo-white.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-logo-white.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-logo.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-logo.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark-64.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark-black-44.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark-black.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark-black.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark.png` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/brand/artcraft-mark.svg` | ArtCraft trademark artwork or artwork sidecar |
| `sources/craft-launcher/docs/screenshots/artcraft-launcher-grid.png` | upstream screenshot visibly contains ArtCraft wordmark |
| `sources/craft-launcher/docs/screenshots/artcraft-launcher-table.png` | upstream screenshot visibly contains ArtCraft wordmark |
| `sources/craft-launcher/vendor/egui/.cargo-ok` | local/cache state |
| `build-inputs/aws-lc-sys-0.45.0/.cargo-ok` | local/cache state |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aes128gcmsiv-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-gcm-avx512.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-gcm-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-sha1-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-sha256-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/aesni-xts-avx512.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/chacha-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/chacha20_poly1305_x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/ghash-ssse3-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/ghash-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/md5-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/p256-x86_64-asm.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/p256_beeu-x86_64-asm.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/rdrand-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/rsaz-2k-avx512.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/rsaz-3k-avx512.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/rsaz-4k-avx512.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/rsaz-avx2.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/sha1-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/sha256-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/sha512-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/trampoline-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/vpaes-x86_64.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/x86_64-mont.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
| `build-inputs/aws-lc-sys-0.45.0/builder/prebuilt-nasm/x86_64-mont5.obj` | Licensed upstream prebuilt NASM fallback object omitted from source-only publication; NASM is required to rebuild the retained assembly source |
