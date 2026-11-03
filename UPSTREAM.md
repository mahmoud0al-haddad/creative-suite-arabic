# Upstream versions and credits

Independent edition maintained by **mahmoud0al-haddad**. Original applications and their authors retain their rights. Names below are factual attribution, not a claim of official endorsement. Exact modified-export checksums are in source-manifest.json and source-checksums.sha256.

| Workspace | Base version | Original source | Exact upstream commit |
| --- | --- | --- | --- |
| cadcraft | 0.8.0 | [cadcraft](https://github.com/storytold/cadcraft) | `714111d528acc86dc6b41e03f9a82cabc1066dca` |
| craft-launcher | 0.5.1 | [craft-launcher](https://github.com/storytold/craft-launcher) | `ed6b32ebf365736ba50bb1ccdc24f183415095f5` |
| deckcraft | 0.8.0 | [deckcraft](https://github.com/storytold/deckcraft) | `22db9261f9d1a214c296fd390173130c45d76d66` |
| designcraft | 0.9.0 | [designcraft](https://github.com/storytold/designcraft) | `7cb8b12507def48fb9346e5004340b5e8e4cfcb7` |
| effectcraft | 0.11.1 | [effectcraft](https://github.com/storytold/effectcraft) | `2a01226894c6bc58ae4fadcc24a10198d693acaa` |
| figcraft | 0.7.0 | [figcraft](https://github.com/storytold/figcraft) | `47438dd4a2e8b49a7e6bb3dfd480262dadc37f12` |
| filmcraft | 0.9.0 | [filmcraft](https://github.com/storytold/filmcraft) | `4a69a038d32761f604b3c7e4845b21fb40b3cbe4` |
| gridcraft | 0.8.0 | [gridcraft](https://github.com/storytold/gridcraft) | `823c56582d00be91e31b4f7f3680496cf89e84fb` |
| lightcraft | 0.9.1 | [lightcraft](https://github.com/storytold/lightcraft) | `4f0b110bcbd2e350c6d97b47f1d824c2222295c4` |
| pdfcraft | 0.9.0 | [pdfcraft](https://github.com/storytold/pdfcraft) | `645016addcfb9078c557f0189559ab47cdcee057` |
| photocraft | 0.10.0 | [photocraft](https://github.com/storytold/photocraft) | `8407d9ccb125af52944fca9d228383459f94aaef` |
| soundcraft | 0.8.0 | [soundcraft](https://github.com/storytold/soundcraft) | `2578ea69a388395ddadb6394d7b4ab40f21aa670` |
| vectorcraft | 0.13.1 | [vectorcraft](https://github.com/storytold/vectorcraft) | `904f0fe5227bd987d55319e89a38b4b8f046998c` |
| wordcraft | 0.8.0 | [wordcraft](https://github.com/storytold/wordcraft) | `f2e15cf8f899a79899a2d9595ca4ce7df1e5ea22` |

## Shared build inputs

- **craft-fonts**: [https://github.com/storytold/craft-fonts.git](https://github.com/storytold/craft-fonts.git), revision `3d9ab3a132ed0eff457e424226a435a901da30d3`; directory `build-inputs/craft-fonts`.
- **aws-lc-sys-0.45.0**: [https://crates.io/crates/aws-lc-sys/0.45.0](https://crates.io/crates/aws-lc-sys/0.45.0), revision `7943223c99d909bc399bdf1b856821bb04f1f3c5`; directory `build-inputs/aws-lc-sys-0.45.0`.

craft-fonts includes Noto Sans Arabic under SIL OFL; the pinned Arabic font SHA-256 is `63111b5b2e074dd48cc67692e0a2726d86ee94c1c37fe8598257b7b4e87e869e`. Retain each font license and source record. The aws-lc-sys source export omits upstream prebuilt NASM fallback objects; public builds require NASM. The source manifest records those exclusions.

## Arabic draft contributions

- **azizr12**: [PhotoCraft PR #2505](https://github.com/storytold/photocraft/pull/2505), revision `777e1774b018984a358092a747ae3964876033f1`. Selected Arabic draft phrases were reviewed/adapted for other workspaces.
- **kirolos-esmat**: [WordCraft PR #398](https://github.com/storytold/wordcraft/pull/398), revision `252a79b16c926852fbe0c35edf2e68c02df276af`. Arabic draft phrases were reviewed/adapted in this edition.
- **ViolinMai**: [EffectCraft PR #836](https://github.com/storytold/effectcraft/pull/836), revision `f629b5c50529805004eabd4a14e2f69f92d96220`. Draft reversed word ordering was restored to logical Unicode; specialized captions/options were reviewed and expanded.

These acknowledgments describe inputs used, not a claim that those contributors authored or approved the complete independent edition. Existing application NOTICE/ATTRIBUTION records remain authoritative.

## Renderer and local modifications

Patched **egui/epaint 0.36.2** keep their MIT/Apache licenses and provenance under each workspace vendor directory. This edition modifies shaping/bidi line layout, cursor/selection handling, ordinary Arabic font selection, language startup behavior, catalogs/diagnostics and neutral independent branding. Other dependencies keep their own licenses. No proprietary Windows font or Microsoft runtime binary is included in the public source export.

## Additional open interface fallbacks

DeckCraft reads the separate optional `craft-fonts/fonts/community-ui-manifest.txt`: Nanum Gothic Regular (NHN Corporation / Sandoll Communications, Google Fonts commit `133ccbee9a8b408eb71f31a36ccb9116f5c695ad`) and ZCOOL XiaoWei Regular (ZCOOL XiaoWei Project Authors / Li Dawei, commit `d427686494a89f0b7ecb9833273b6aabb83a6350`). Both retain their original SIL OFL-1.1 text, metadata and exact file SHA-256 under their input directories. Other applications keep their existing font manifests.

The same DeckCraft-only optional manifest also supplies its default Inter Variable family by Rasmus Andersson / The Inter Project Authors, Google Fonts commit `0b58fb370093f9a9f4ff785d94405710b79de67c`, TTF SHA-256 `29160a80ff49ddcab2c97711247e08b1fab27a484a329ce8b813d820dc559031`. Its original OFL and metadata are retained in `fonts/community-inter/`.
