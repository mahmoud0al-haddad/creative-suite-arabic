# خارطة الطريق / Roadmap

المشرف: **mahmoud0al-haddad**. الأولوية لإصلاح خلل قابل لإعادة الإنتاج، والحفاظ على أعمال المستخدم، ثم تحسين التعريب والأداء. البنود التالية اتجاه للعمل وليست مواعيد أو ضمانات دعم دائم.

| Priority | Work | Completion evidence |
| --- | --- | --- |
| P0 | Public Windows package with independent runtime prerequisite | Clean-machine install, missing/old/new runtime paths, twelve launcher opens, source/license/hash checks |
| P1 | Arabic captions, diagnostics and terminology regressions | Minimal issue example, catalog/placeholder test, reviewed Arabic/English screenshot |
| P1 | RTL mixed-text input, caret and selection | Event-level regression for logical positions and visible selection/wrapping |
| P1 | Lag and narrow/scaled UI problems | Before/after timing on a synthetic project and readable 1×/1.5×/2× UI |
| P2 | Review pinned upstream updates | Read-only upstream report, reviewed merge, new catalog references and full affected CI/native workflows |
| P2 | Broader platform/device coverage | Recorded VM/device environment and actual tested behaviors, without extending claims beyond evidence |
| P2 | Dependency/asset audit automation | Installed audit tool, recorded findings, license notices and resolved or documented exceptions |

Known boundaries: upstream applications retain their own incomplete features and platform restrictions; third-party plugin and OS interfaces are outside fixed UI catalogs; optional models and devices need separate testing. The existing review is representative rather than exhaustive. Future release notes must state regressions and skipped gates instead of claiming universal 100% correctness.
