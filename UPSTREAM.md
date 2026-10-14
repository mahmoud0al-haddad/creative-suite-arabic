# Upstream versions and credits

Independent edition maintained by **mahmoud0al-haddad**. Original applications and their authors retain their rights. Names below are factual attribution, not a claim of official endorsement. Exact modified-export checksums are in source-manifest.json and source-checksums.sha256.

| Workspace | Base version | Original source | Exact upstream commit |
| --- | --- | --- | --- |
| cadcraft | 0.5.0 | [cadcraft](https://github.com/storytold/cadcraft) | `54ea5e743a29acd8f3178cfaa82fb74a38c2de72` |
| deckcraft | 0.5.0 | [deckcraft](https://github.com/storytold/deckcraft) | `be14248577b92059eb2b8c0c9460cceffbc1f01d` |
| designcraft | 0.6.0 | [designcraft](https://github.com/storytold/designcraft) | `ca343e8985dc4d92dd65cf77a8198723b5f8a9eb` |
| effectcraft | 0.8.0 | [effectcraft](https://github.com/storytold/effectcraft) | `67434e90388b6dca4417555346fa6850b6f96033` |
| filmcraft | 0.6.0 | [filmcraft](https://github.com/storytold/filmcraft) | `0800a8b0aa4a703ba42172f415b45a61f8d8a5e1` |
| gridcraft | 0.5.0 | [gridcraft](https://github.com/storytold/gridcraft) | `fb3737aa0308114dc14afd1dc4960a939bc34eed` |
| lightcraft | 0.6.0 | [lightcraft](https://github.com/storytold/lightcraft) | `76b5377868d912db69a777bda253a5b1b96b22bd` |
| pdfcraft | 0.6.0 | [pdfcraft](https://github.com/storytold/pdfcraft) | `881682c49570b8d29d74c95fa54354190fd454a1` |
| photocraft | 0.7.0 | [photocraft](https://github.com/storytold/photocraft) | `2d6ff3f6cc015ff4d13e1dfbdf656317ec3a3451` |
| soundcraft | 0.5.0 | [soundcraft](https://github.com/storytold/soundcraft) | `240aa848a04811ffff75cf193e69d9d4c404ae68` |
| vectorcraft | 0.9.0 | [vectorcraft](https://github.com/storytold/vectorcraft) | `2ac3ba2b866ad9d34b2252dd8024f73e34f4b2b8` |
| wordcraft | 0.5.0 | [wordcraft](https://github.com/storytold/wordcraft) | `30993ffb3f3ec710e0a285865e4de731518687ae` |
| craft-launcher | 0.2.1 | [craft-launcher](https://github.com/storytold/craft-launcher) | `c273a94a9a37c7a2d7a07fd6cc9ca44e3dcc6b9c` |

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
