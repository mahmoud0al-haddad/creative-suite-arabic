# تحديث الحزمة العربية / Updating the Arabic edition

الحزمة المتكاملة تستقبل تحديثاتها من إصدارات **mahmoud0al-haddad/creative-suite-arabic**. يتضمن التحديث المشغّل والتطبيقات المعربة معًا، مع بصمات الملفات وإصدارات المصادر التي بُنيت منها.

## للمستخدم

1. افتح المشغّل. في الإعداد الافتراضي يفحص تحديث الحزمة مرة يوميًا أثناء تشغيله. عند توفر إصدار أحدث يظهر إشعار، ويظهر زر **«تحديث الحزمة العربية»** داخل المشغّل.
2. للفحص فورًا اضغط **«التحقق من وجود تحديثات»**. عبارة **«لا يوجد إصدار عربي منشور بعد»** تعني أن المستودع لم ينشر إصدارًا صالحًا للحزمة؛ لا تعني أن تطبيقات الشركة الأصلية توقفت عن التحديث.
3. احفظ مشاريعك وأغلق تطبيقات الحزمة، ثم اختر التحديث واضغط **«تنزيل برنامج التثبيت وتشغيله»**. يعرض المشغّل تقدم التنزيل ويتحقق من حجم الملف وبصمة SHA-256 قبل تشغيله.
4. أكمل معالج التثبيت الذي يظهر لك. يتحقق مساعد التحديث من إصدارات وبصمات التطبيقات المثبتة، ثم يعيد فتح المشغّل ويعرض نتيجة العملية. إعداداتك الحالية تُحفظ؛ اختيار لغة أول تثبيت لا يفرض نفسه على إعدادات موجودة.

يمكن تنزيل Setup يدويًا من [صفحة الإصدارات](https://github.com/mahmoud0al-haddad/creative-suite-arabic/releases). استخدم الملف والبصمة من الإصدار نفسه. إذا تعذّر الاتصال، أعد الفحص عند عودة الشبكة. إذا لم يظهر إشعار النظام، افتح المشغّل وافحص يدويًا، وتحقق من إعداد الإشعارات فيه وفي Windows.

التثبيت يبدأ باختيارك. تعطيل الفحص التلقائي لا يمنع الفحص اليدوي. تحديثات المصدر الأصلي تُراجع قبل إصدار حزمة عربية جديدة؛ صدور نسخة أصلية اليوم لا يعني وجود نسخة عربية معتمدة منها في اللحظة نفسها.

## للمشرف والمساهمين

فحص GitHub Actions الأسبوعي يقارن الإصدارات الأصلية بمراجع `source-pins.json` ويحفظ تقريرًا في **Actions → Read-only upstream versions**. يلزم أن تكون Actions مفعّلة في المستودع. التقرير لا يترجم النصوص أو يدمج تغييرات أو ينشر ملفات تنفيذية تلقائيًا.

بعد ظهور تحديث أصلي: راجع التغييرات، ادمج التعريب، ترجم النصوص الجديدة، شغّل الفحوص الخاصة بالتطبيق واختبارات العربية والإنجليزية والحفظ، وراجع الصور. بعدها جهّز حزمة كاملة وبيانات تحديثها من الملفات النهائية، وفق [RELEASING.md](RELEASING.md). لا تعدّل بيانات إصدار قديم ليشير إلى ملفات جديدة؛ أنشئ إصدارًا جديدًا بمراجعة وبصمات جديدة.

## English

The integrated launcher checks this repository for reviewed complete-bundle releases once daily by default while it is running. Use **Check for updates** for an immediate check. When a newer edition is available, save your work, close the suite apps, choose **Update Arabic suite**, then **Download and open setup**. Download size and SHA-256 are verified before the visible installer starts. After installation, the helper checks installed component versions/hashes and restarts the launcher. Existing preferences are retained.

Manual downloads are available on [Releases](https://github.com/mahmoud0al-haddad/creative-suite-arabic/releases). A missing desktop notification does not prevent a manual check. Upstream releases need translation review, tests and a new bundle release before becoming Arabic-edition updates. The scheduled upstream report only reads versions; it does not automatically merge, build or publish an update.
