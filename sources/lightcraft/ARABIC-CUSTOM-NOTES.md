# Private Arabic/English modification

This is a local modified build, not an official ArtCraft release. The graphical ArtCraft A-mark and wordmark were removed from the modified interface. Plain text upstream attribution and independently licensed app animal icons are retained.

Arabic catalogs use logical Unicode order. The vendored egui and epaint (0.36.2, MIT OR Apache-2.0) were modified for Unicode bidi shaping, wrapping, caret movement and selection. Both upstream licenses and renderer provenance are in vendor/.

The ordinary Windows Tahoma face is read from the user's installed Windows fonts at runtime when available. No Tahoma font bytes are redistributed. The open Noto Sans Arabic fallback comes from the separate craft-fonts build input under SIL OFL-1.1; its source/license accompanies the font input/package notices.

Translations combine original local Arabic translation work with reviewed exact matching phrases from the MIT OR Apache-2.0 Craft application sources and these upstream contributions:

- EffectCraft Arabic draft by ViolinMai, https://github.com/storytold/effectcraft/pull/836, revision f629b5c50529805004eabd4a14e2f69f92d96220. The draft's manually reversed Arabic word order was restored to logical Unicode and specialist terms were reviewed.
- PhotoCraft Arabic draft by azizr12, https://github.com/storytold/photocraft/pull/2505, revision 777e1774b018984a358092a747ae3964876033f1.
- WordCraft Arabic draft by kirolos-esmat, https://github.com/storytold/wordcraft/pull/398, revision 252a79b16c926852fbe0c35edf2e68c02df276af.
- Craft suite source catalogs and local reviewed Arabic additions from PdfCraft, DesignCraft and office applications, all under the respective upstream MIT OR Apache-2.0 licenses.

Source command identifiers, serialized values, enum indices and user document/file names are unchanged by interface translation. CRAFT_SUITE_LANGUAGE=ar or en overrides only the initial interface language after preferences load; users can change it again in the application's language settings.
