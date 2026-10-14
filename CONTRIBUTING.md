# المساهمة / Contributing

المشرف على النسخة المستقلة: [mahmoud0al-haddad](https://github.com/mahmoud0al-haddad). نرحب بإصلاحات العربية والاتجاه والإدخال والأداء، وبمراجعة الترجمة حسب تخصص التطبيق. تظل تعليمات AGENTS/CLAUDE لكل workspace سارية على تغييرات مصدره، مع حفظ التراخيص وإسناد العمل.

## Report a problem

Use the Arabic translation or performance issue form. Include the app, release tag, Windows version, selected language, minimal steps, expected/actual behavior, and a harmless synthetic file if needed. Screenshots should show only your own non-sensitive example. Never attach access tokens, control-channel keys, private documents, full settings folders or unsanitized logs.

## Translation changes

Keep Arabic in **logical Unicode order**. Never reverse words in the catalog to imitate RTL. Use ordinary readable Arabic typography. Preserve anonymous/named placeholders, plural arguments, punctuation semantics, units, identifiers, file formats and keyboard shortcuts where meaningful. Do not translate command IDs, project data, user filenames, custom layer names or plugin-owned data as fixed interface captions.

Translate full phrases in context; the same English key can mean different things in audio, images, video and typography. Review the displayed sentence and parameter values, not just catalog row counts. Prefer existing precise terminology, then add contextual keys where needed. Changes to canonical effect/parameter names must preserve renamed user labels and enum values.

## Required verification

1. Run `scripts/check-translations.py` for the supported exported catalog structures, then the affected workspace's real catalog/placeholder/UI tests. The static checker is an early check, not proof of total translation coverage.
2. Run formatting, clippy, relevant tests and the workspace CI/layers/assets/wasm gates. Use bounded test parallelism and distinct xtask targets. Record optional skipped gates honestly.
3. For renderer/editing changes, test Arabic/Latin/digits, cursor click/arrows/backspace, graphemes/ligatures, selection highlighting, wrap/elision, blank paragraphs and rich text section colors.
4. Review Arabic/English screenshots at narrow/wide sizes and different UI scaling. For lag reports, measure the same synthetic project before/after and separate startup, rendering and input delay.
5. Exercise startup language override and in-app selection persistence, then create/save/reopen an Arabic-named example. Check that document content is unchanged and the installed app opens from the launcher.

Keep fixes focused and include a regression that exposes a meaningful bug. Avoid tests that merely repeat the implementation. No automatic merge of upstream releases, unreviewed machine translation, silent replacement of the user's modified apps, or promise that future updates will always remain fully translated.

## Source updates

Review the weekly upstream report, choose a pinned revision deliberately, merge/resolve source changes, refresh translation references, review assets/licenses, and run the release gates before updating the source pins and publishing a rebuilt Setup. The read-only watcher never creates an issue or merges code. Maintainers decide priorities; sustained community maintenance is a goal, not a lifetime service guarantee.
