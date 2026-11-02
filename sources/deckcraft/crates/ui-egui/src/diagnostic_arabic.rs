// Independently reviewed display-only diagnostic templates. Values stay verbatim.
const AR_DIAGNOSTICS: &[(&str, &str)] = &[
    ("No {} dictionary is installed, so only common misspellings are found. Put a Hunspell .aff/.dic pair in {}.", "لم يُثبّت قاموس {}، لذلك تُكتشف الأخطاء الإملائية الشائعة فقط. ضع ملفَي Hunspell بالامتدادين .aff و.dic في {}."),
    ("{name} was not applied: the presentation it was chosen for is closed", "لم يُطبّق {name}: أُغلق العرض التقديمي الذي اختير له"),
    ("Missing fonts: {names}. Similar fonts are used for their text.", "الخطوط غير المتاحة: {names}. تُستخدم خطوط مشابهة لعرض نصوصها."),
    ("Do you want to save the changes you made to “{name}”?", "هل تريد حفظ التغييرات التي أجريتها على «{name}»؟"),
    ("Do you want to save the changes you made to {what}?", "هل تريد حفظ التغييرات التي أجريتها على {what}؟"),
    ("{}: not a settings file; using the defaults", "{}: ليس ملف إعدادات؛ تُستخدم الإعدادات الافتراضية"),
    ("{}: reading the file stopped unexpectedly", "{}: توقفت قراءة الملف على نحو غير متوقع"),
    ("Rotation {rot:.0}°, brightness {brightness:.1}, contrast {contrast:.1}", "الدوران {rot:.0}°، السطوع {brightness:.1}، التباين {contrast:.1}"),
    ("No recent presentations match “{}”.", "لا توجد عروض تقديمية حديثة تطابق «{}»."),
    ("No themes or templates match “{}”.", "لا توجد نُسق أو قوالب تطابق «{}»."),
    ("Align text to the {l} of the cell", "محاذاة النص إلى {l} الخلية"),
    ("Command `{command}` is not available: {message}", "الأمر `{command}` غير متاح: {message}"),
    ("{} needs a colour like #RRGGBB.", "يتطلب {} لونًا بصيغة #RRGGBB."),
    ("Invalid parameters for `{command}`: {message}", "معلمات غير صالحة للأمر `{command}`: {message}"),
    ("{path} — this file can't be found", "{path} — تعذّر العثور على هذا الملف"),
    ("DeckCraft can't export “{other}”", "لا يستطيع DeckCraft التصدير بالتنسيق «{other}»"),
    ("“{from}” isn't used explicitly.", "لا يُستخدم «{from}» صراحةً."),
    ("“{dialog}” is not available yet.", "«{dialog}» غير متاح بعد."),
    ("DeckCraft can't find “{text}”.", "تعذّر على DeckCraft العثور على «{text}»."),
    ("`{key}` must be true or false", "يجب أن تكون قيمة `{key}` إما true أو false"),
    ("{} contributors · {} commits", "{} مساهم · {} إيداع"),
    ("Replaced {} occurrence(s).", "عدد المواضع المستبدلة: {}."),
    ("Inserted {} slides from {path}", "عدد الشرائح المدرجة من {path}: {}"),
    ("{}: {e}; settings not saved", "{}: {e}؛ لم تُحفظ الإعدادات"),
    ("{}: {e}; using the defaults", "{}: {e}؛ تُستخدم الإعدادات الافتراضية"),
    ("Match {} of {} on slide {}{where_}.", "التطابق {} من {} في الشريحة {}{where_}."),
    ("command `{id}` panicked: {msg}", "توقف الأمر `{id}` بسبب خطأ داخلي: {msg}"),
    ("Slide inserted after {slide}", "شريحة مُدرجة بعد {slide}"),
    ("Reading the file: {secs:.0} s", "جارٍ قراءة الملف: {secs:.0} ث"),
    ("Replaced “{from}” with “{}”.", "استُبدل «{from}» بـ «{}»."),
    ("No themes match “{}”.", "لا توجد سمات تطابق «{}»."),
    ("{kind_label} (click to change)", "{kind_label} (انقر للتغيير)"),
    ("Unknown command `{command}`", "أمر غير معروف `{command}`"),
    ("{name} was not applied: {e}", "لم يُطبّق {name}: {e}"),
    ("Clear the tab at {pos:.2}\"", "مسح علامة الجدولة عند {pos:.2} بوصة"),
    ("Slide {:02}:{:04.1}   Total {:02}:{:02}", "الشريحة {:02}:{:04.1}   المجموع {:02}:{:02}"),
    ("{label} must be a number", "يجب أن تكون قيمة {label} رقمًا"),
    ("Slides Per\nPage: {}", "الشرائح في كل\nصفحة: {}"),
    ("Start: {:.2}s, End: {:.2}s", "البداية: {:.2} ث، النهاية: {:.2} ث"),
    ("Bad parameters: {message}", "معلمات غير صالحة: {message}"),
    ("Heading sample  {major}", "نموذج عنوان  {major}"),
    ("{n} slides per page", "عدد الشرائح في الصفحة: {n}"),
    ("Duration handle {}", "مقبض المدة {}"),
    ("Stopped opening {}", "أُوقف فتح {}"),
    ("{} slides per page", "عدد الشرائح في الصفحة: {}"),
    ("Not available: {message}", "غير متاح: {message}"),
    ("Action Button: {k}", "زر إجراء: {k}"),
    ("Still opening {name}", "ما زال فتح {name} جاريًا"),
    ("{name}  (dictionary)", "{name}  (قاموس)"),
    ("Compared with {}", "تمت المقارنة مع {}"),
    ("Slide {} changed", "الشريحة {} معدّلة"),
    ("Slide {} deleted", "الشريحة {} محذوفة"),
    ("WordArt style {}", "نمط نص زخرفي {}"),
    ("{} presentations", "{} عروض تقديمية"),
    ("First {GRID_CAP} slides", "الشرائح الأولى وعددها {GRID_CAP}"),
    ("Unsupported: {message}", "غير مدعوم: {message}"),
    ("Body sample  {minor}", "نموذج نص أساسي  {minor}"),
    ("Accept All ({})", "قبول الكل ({})"),
    ("{} Slide Master", "{} شريحة رئيسية"),
    ("On Click of {name}", "عند النقر على {name}"),
    ("Brightness {:+}%", "السطوع {:+}%"),
    ("Dictionary: {}", "القاموس: {}"),
    ("Go to slide {}", "الانتقال إلى الشريحة {}"),
    ("Saturation {}%", "التشبع {}%"),
    ("{when} · Resolved", "{when} · تم الحل"),
    ("{f} (Headings)", "{f} (العناوين)"),
    ("Timing bar {}", "شريط التوقيت {}"),
    ("← Back to {title}", "← العودة إلى {title}"),
    ("No sound: {err}", "لا يوجد صوت: {err}"),
    ("Slide {} of {n}", "الشريحة {} من {n}"),
    ("Slide {}: “{w}”", "الشريحة {}: «{w}»"),
    ("Contrast {:+}%", "التباين {:+}%"),
    ("Slide {} of {}", "الشريحة {} من {}"),
    ("{} ({} slides)", "{} ({} شرائح)"),
    ("Exported {path}", "تم تصدير {path}"),
    ("Opening {name}…", "جاري فتح {name}…"),
    ("Slide {}: {}{}", "الشريحة {}: {}{}"),
    ("{ch}  Bullet", "{ch}  تعداد نقطي"),
    ("{r} pt glow", "توهج {r} نقطة"),
    ("Version {}", "الإصدار {}"),
    ("{}  Hidden", "{}  مخفية"),
    ("panic{at}: {}", "خطأ داخلي{at}: {}"),
    ("{n} Column{}", "{n} أعمدة{}"),
    ("{}x{} Table", "جدول {}×{}"),
    ("{f} (Body)", "{f} (المتن)"),
    ("Series {}", "السلسلة {}"),
    ("Saved {path}", "تم حفظ {path}"),
    ("Text: {text}", "النص: {text}"),
    ("Style {i}", "النمط {i}"),
    ("Group {}", "المجموعة {}"),
    ("Page: {}", "صفحة: {}"),
    ("Shape {}", "شكل {}"),
    ("Slide {}", "الشريحة {}"),
    ("{a} and {b}", "{a} و{b}"),
    ("{} of {}", "{} من {}"),
    ("{} hex", "{} بالنظام الست عشري"),
    ("{:.0} pt{side}", "{:.0} نقطة{side}"),
    ("{r} pt", "{r} نقطة"),
    ("{w} pt", "{w} نقطة"),

    ("Nothing to print: every slide in the range is hidden.", "لا يوجد ما يُطبع: جميع الشرائح في النطاق مخفية."),
    ("the presentation has {count} slides; “{part}” is outside them", "عدد شرائح العرض التقديمي {count}؛ والنطاق «{part}» يتجاوزها"),
    ("saving to a path is not available in the browser", "الحفظ إلى مسار غير متاح في المتصفح"),
    ("nothing to print: no presentation is open", "لا يوجد ما يُطبع: لا يوجد عرض تقديمي مفتوح"),
    ("Couldn't save AutoRecover information: {e}", "تعذر حفظ معلومات الاسترداد التلقائي: {e}"),
    ("command `{0}` is not available right now: {1}", "الأمر `{0}` غير متاح الآن: {1}"),
    ("render-transition needs an output path", "يتطلب render-transition مسار إخراج"),
    ("--connect needs a port or host:port", "يتطلب --connect منفذًا أو host:port"),
    ("File › {} needs an open presentation", "يتطلب الأمر «ملف › {}» عرضًا تقديميًا مفتوحًا"),
    ("use file.openBytes in the browser", "استخدم file.openBytes في المتصفح"),
    ("render-transition needs --slide N", "يتطلب render-transition تحديد --slide N"),
    ("recovery save after `{id}` failed: {e}", "فشل حفظ الاسترداد بعد `{id}`: {e}"),
    ("no frame could be decoded there", "تعذر فك ترميز إطار في ذلك الموضع"),
    ("render-transition needs a FILE", "يتطلب render-transition تحديد FILE"),
    ("couldn't read dropped file {name}: {e}", "تعذّرت قراءة الملف المُسقط {name}: {e}"),
    ("invalid parameters for `{cmd}`: {msg}", "معلمات غير صالحة للأمر `{cmd}`: {msg}"),
    ("render-transition needs --t", "يتطلب render-transition تحديد --t"),
    ("missing or invalid `color`", "قيمة `color` مفقودة أو غير صالحة"),
    ("no recovery folder is set", "لم يُحدد مجلد للاسترداد"),
    ("the media data is missing", "بيانات الوسائط مفقودة"),
    ("need at least two points", "نقطتان على الأقل مطلوبتان"),
    ("no text insertion point", "لا توجد نقطة لإدراج النص"),
    ("select text or a shape", "حدد نصًا أو شكلًا"),
    ("unknown mcp option `{other}`", "خيار mcp غير معروف `{other}`"),
    ("download of {name} failed: {e}", "فشل التنزيل من {name}: {e}"),
    ("the object is locked", "العنصر مقفل"),
    ("unknown File page `{id}`", "صفحة ملف غير معروفة `{id}`"),
    ("--width needs pixels", "يتطلب --width عددًا بالبكسل"),
    ("unknown command `{0}`", "أمر غير معروف `{0}`"),
    ("unknown command `{id}`", "أمر غير معروف `{id}`"),
    ("--t needs a value", "يتطلب --t قيمة"),
    ("unknown effect `{effect}`", "تأثير غير معروف `{effect}`"),
    ("unknown option `{other}`", "الخيار `{other}` غير معروف"),
    ("nothing to group", "لا يوجد شيء لتجميعه"),
    ("bad --t value `{x}`", "قيمة --t غير صالحة `{x}`"),
    ("not editing text", "لا يجري تحرير نص"),
    ("nothing to undo", "لا شيء للتراجع عنه"),
    ("nothing to redo", "لا شيء لإعادته"),
    ("no such comment", "لا يوجد تعليق بهذا المعرف"),
    ("no such layout", "لا يوجد هذا التخطيط"),
    ("insert failed", "تعذّر الإدراج"),
    ("no document", "لا يوجد مستند"),
    ("no slides", "لا توجد شرائح"),
    ("no window", "لا توجد نافذة"),
    ("no writer", "لا تتوفر أداة كتابة"),
    ("no body", "لا يوجد محتوى أساسي"),
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
];
