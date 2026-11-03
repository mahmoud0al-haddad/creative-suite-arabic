# حالة المصادر والتعريب — ٢ نوفمبر ٢٠٢٦

هذه مسودة مصادر كاملة للبرامج الثلاثة عشر والمشغّل، بإعداد وصيانة **mahmoud0al-haddad**. مراجع المصدر الأصلية مجمّدة من فحص ١٩ أكتوبر ٢٠٢٦، وهي ليست ادعاءً بأنها أحدث إصدارات الشركة يوم قراءة هذه الصفحة. لم يُنشر من هذه المسودة سيت أب معتمد.

تتضمن المصادر التعريبات، وتعديلات عرض العربية وإدخالها، والخطوط المفتوحة وتراخيصها، والمشغّل المتكامل، وأدوات بناء الحزمة وتحديثها من GitHub. الإنجليزية تبقى متاحة. محتوى الملفات وأسماء الطبقات والمعرّفات تظل بيانات المستخدم.

| المكوّن | حالة التحقق الحالية |
| --- | --- |
| FilmCraft 0.9، SoundCraft 0.8، CADCraft 0.8 | بوابات المصدر والبناء والخصوصية، وتشغيل عربي/إنجليزي وحفظ/إعادة فتح ومراجعة الصور اجتازت. فحص Sound استعمل وضعًا بلا عتاد صوت. |
| Launcher 0.5.1 | بوابات المصدر الأربع اجتازت؛ يلزم بناء ملف سطح المكتب النهائي والتحقق من التثبيت والتحديث الفعلي. |
| PdfCraft 0.9 | اختبارات الواجهة اجتازت؛ جُدد سجل الأصول المشتق، وبقية التحقق والتشغيل النهائي معلقة. |
| DesignCraft 0.9 | عولجت افتراضات مسارات Windows وتحميل خط الواجهة؛ يلزم إكمال إعادة الاختبارات والبناء. |
| WordCraft 0.8 | فشل اختباران لذاكرة الخطوط التايلاندية المؤقتة؛ المراجعة جارية. |
| PhotoCraft 0.10 | توقف اختبار GPU الأصلي marquee_autoscroll بعطل عملية؛ يحتاج فحصًا مركّزًا وإكمال البوابات. |
| EffectCraft 0.11.1 | كشف اختبار السجل 32 تسمية جديدة تحتاج ترجمة ثم إعادة الاختبار. |
| LightCraft 0.9.1، FigCraft 0.7، GridCraft 0.8، DeckCraft 0.8، VectorCraft 0.13.1 | دمج المصدر والتعريب موجود؛ البوابات والتشغيل النهائي لهذه الإصدارات لم تكتمل. |

الصور المنشورة في مسودة الموقع تخص النسخ الثلاث المراجعة فقط. أي بطاقة أخرى بلا صورة ما زالت قيد المراجعة. هذه الفحوص لا تعتمد كل سير عمل أو جهاز، ولا تثبت نجاح تنزيل سيت أب وتثبيته من GitHub؛ ذلك اختبار مستقل قبل الإصدار النهائي.

## الملفات والبناء

- `sources/`: المصادر المعدلة للبرامج الـ١٣ وللانشر.
- `build-inputs/`: الخطوط المفتوحة ومصدر AWS-LC مع الحقوق والتراخيص الأصلية.
- `source-pins.json`: الإصدارات والمراجع الأصلية الدقيقة.
- `source-manifest.json` و`source-checksums.sha256`: جرد الملفات وبصماتها والتعديلات اللازمة لتخطيط البناء العام.
- `scripts/` و`packaging/` و`tests/`: أدوات البناء والتعريب والتحقق والتثبيت والتحديث.
- `docs/`: مسودة الصفحة ثنائية اللغة والصور الحالية.

راجع [BUILD.md](BUILD.md) للبناء، و[UPDATES.md](UPDATES.md) لآلية التحديث. لا تتضمن المصادر ملفات تشغيل Microsoft، أو خط Tahoma، أو ذاكرة بناء المطور، أو بيانات تحكمه المحلية. بيانات الاختبار الاصطناعية في مصدر التشفير مطابقة حرفيًا لأرشيف الحزمة الأصلي العام؛ تسجل بصماتها ومصدرها في الجرد.

## English

This is a complete source-only draft checkpoint for thirteen Arabic/English applications and the launcher, maintained by **mahmoud0al-haddad**. Upstream commits are frozen from the October 19 snapshot; they are not a claim of latest upstream versions as of November 2. No tested installer is released from this checkpoint. Film, Sound and CAD have reviewed native evidence; the table above records the remaining validation work. Public source/placeholder checks do not replace native, installer, hardware or real GitHub update testing.

Latest follow-up: [official releases observed November 3](validation/observed-upstreams-20261103.json) are newer than this preserved source snapshot. Integration of those releases is ongoing; this checkpoint is not a claim that they have already been translated and validated.
