//! Arabic error display. Machine/control errors and diagnostic values remain unchanged.
use figcraft_engine::EngineError;

include!("diagnostic_arabic.rs");

/// Error-only boundary for callbacks that return an error type other than EngineError.
pub(crate) fn message(message: &str) -> String {
    if crate::i18n::current().is_none_or(|l| l.code != "ar") {
        return message.into();
    }
    reason_at(message, 3)
}

pub fn display(error: &EngineError) -> String {
    if crate::i18n::current().is_none_or(|l| l.code != "ar") {
        return error.to_string();
    }
    match error {
        EngineError::UnknownCommand(command) => format!("أمر غير معروف: `{command}`"),
        EngineError::Disabled(command, reason) => format!("الأمر `{command}` غير متاح حالياً: {}", reason_at(reason, 3)),
        EngineError::BadParams { cmd, msg } => format!("معاملات غير صالحة للأمر `{cmd}`: {}", reason_at(msg, 3)),
        EngineError::NoDocument => "لا يوجد مستند مفتوح".into(),
        EngineError::NoNode(id) => format!("لا توجد عقدة بالمعرّف {id}"),
        EngineError::Other(reason) => reason_at(reason, 3),
        EngineError::Waiting => "بانتظار الخادم".into(),
        EngineError::Internal { cmd, msg } => {
            format!("خطأ داخلي في `{cmd}`: {} (أُعيد المستند إلى حالته السابقة؛ يرجى الإبلاغ عن هذا الخلل)", reason_at(msg, 3))
        }
    }
}

fn reason_at(message: &str, depth: usize) -> String {
    if let Some((_, value)) = AR_DIAGNOSTICS.iter().find(|(source, _)| *source == message) {
        return (*value).into();
    }
    if depth == 0 || message.len() > 16_384 {
        return message.into();
    }
    for (source, translated) in AR_DIAGNOSTICS {
        if !has_prose_template(source) {
            continue;
        }
        if let Some(values) = capture(source, message) {
            return fill(translated, &values, depth);
        }
    }
    message.into()
}

/// Template tokens own decoded literal braces; captured values are never tokenized again.
#[derive(Debug)]
enum Token<'a> {
    Literal(String),
    Field(&'a str),
}

fn valid_field(field: &str) -> bool {
    let (name, spec) = field.split_once(':').unwrap_or((field, ""));
    if !name.is_empty() && !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    // A JSON member such as {id: value} is prose, not a Rust format field.
    // Accept the actual formatting grammar used by diagnostics, including named widths.
    let mut rest = spec;
    let mut chars = rest.char_indices();
    let first = chars.next();
    let second = chars.next();
    if let Some((_, c)) = first {
        if matches!(c, '<' | '>' | '^') {
            rest = &rest[c.len_utf8()..];
        } else if let Some((offset, c)) = second
            && matches!(c, '<' | '>' | '^')
        {
            rest = &rest[offset + c.len_utf8()..];
        }
    }
    if rest.starts_with(['+', '-']) {
        rest = &rest[1..];
    }
    if let Some(tail) = rest.strip_prefix('#') {
        rest = tail;
    }
    if let Some(tail) = rest.strip_prefix('0') {
        rest = tail;
    }
    let width = rest.bytes().take_while(u8::is_ascii_digit).count();
    rest = &rest[width..];
    if width == 0
        && let Some(dollar) = rest.find('$')
    {
        let parameter = &rest[..dollar];
        if !parameter.is_empty() && parameter.chars().all(|c| c.is_alphanumeric() || c == '_') {
            rest = &rest[dollar + 1..];
        }
    }
    if let Some(precision) = rest.strip_prefix('.') {
        rest = precision;
        if let Some(tail) = rest.strip_prefix('*') {
            rest = tail;
        } else {
            let digits = rest.bytes().take_while(u8::is_ascii_digit).count();
            if digits > 0 {
                rest = &rest[digits..];
                if let Some(tail) = rest.strip_prefix('$') {
                    rest = tail;
                }
            } else if let Some(dollar) = rest.find('$') {
                let parameter = &rest[..dollar];
                if parameter.is_empty() || !parameter.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    return false;
                }
                rest = &rest[dollar + 1..];
            } else {
                return false;
            }
        }
    }
    matches!(rest, "" | "?" | "x?" | "X?" | "x" | "X" | "o" | "b" | "p" | "e" | "E")
}

fn tokens(template: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut literal = String::new();
    let mut rest = template;
    while !rest.is_empty() {
        if let Some(tail) = rest.strip_prefix("{{") {
            literal.push('{');
            rest = tail;
        } else if let Some(tail) = rest.strip_prefix("}}") {
            literal.push('}');
            rest = tail;
        } else if rest.starts_with('{') {
            if let Some(close) = rest.find('}') {
                let field = &rest[1..close];
                if valid_field(field) {
                    tokens.push(Token::Literal(std::mem::take(&mut literal)));
                    tokens.push(Token::Field(field));
                    rest = &rest[close + 1..];
                    continue;
                }
            }
            literal.push('{');
            rest = &rest[1..];
        } else if let Some(c) = rest.chars().next() {
            literal.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    tokens.push(Token::Literal(literal));
    tokens
}

fn has_prose_template(template: &str) -> bool {
    let parts = tokens(template);
    parts.iter().any(|part| matches!(part, Token::Field(_)))
        && parts.iter().any(|part| matches!(part, Token::Literal(text) if text.chars().any(char::is_alphabetic)))
}

/// Capture already formatted values without parsing numbers, identifiers or file names.
/// Match a complete template in one unambiguous way, within fixed work and depth limits.
/// Captured text remains opaque; repeated named fields must contain the same bytes.
fn unique_capture<'m>(literals: &[&str], names: &[&str], message: &'m str) -> Option<Vec<&'m str>> {
    if names.is_empty() || names.len() > 32 || literals.len() != names.len() + 1 || message.len() > 65_536 {
        return None;
    }
    let remaining = message.strip_prefix(literals.first().copied()?)?;
    struct Search<'p, 'm> {
        literals: &'p [&'p str],
        names: &'p [&'p str],
        budget: usize,
        exhausted: bool,
        ambiguous: bool,
        answer: Option<Vec<&'m str>>,
    }
    impl<'m> Search<'_, 'm> {
        fn spend(&mut self) -> bool {
            if self.budget == 0 {
                self.exhausted = true;
                return false;
            }
            self.budget -= 1;
            !self.ambiguous
        }

        fn candidate(&mut self, index: usize, rest: &'m str, values: &mut Vec<&'m str>, end: usize, separator: &str) {
            if !self.spend() {
                return;
            }
            let (Some(&name), Some(value), Some(tail)) = (self.names.get(index), rest.get(..end), rest.get(end..)) else {
                return;
            };
            let Some(tail) = tail.strip_prefix(separator) else {
                return;
            };
            if !name.split(':').next().unwrap_or(name).is_empty()
                && self.names.iter().take(index).zip(values.iter()).any(|(prior, captured)| *prior == name && *captured != value)
            {
                return;
            }
            values.push(value);
            self.visit(index + 1, tail, values);
            values.pop();
        }

        fn visit(&mut self, index: usize, rest: &'m str, values: &mut Vec<&'m str>) {
            if !self.spend() {
                return;
            }
            if index == self.names.len() {
                if rest.is_empty() {
                    if let Some(answer) = &self.answer {
                        self.ambiguous = answer != values;
                    } else {
                        self.answer = Some(values.clone());
                    }
                }
                return;
            }
            let (Some(&name), Some(&separator)) = (self.names.get(index), self.literals.get(index + 1)) else {
                return;
            };
            let prior = if name.split(':').next().unwrap_or(name).is_empty() {
                None
            } else {
                self.names.iter().take(index).zip(values.iter()).find(|(prior, _)| **prior == name).map(|(_, value)| *value)
            };
            if let Some(prior) = prior {
                if rest.starts_with(prior) {
                    self.candidate(index, rest, values, prior.len(), separator);
                }
            } else if index + 1 == self.names.len() {
                if let Some(value) = rest.strip_suffix(separator) {
                    self.candidate(index, rest, values, value.len(), separator);
                }
            } else if separator.is_empty() {
                for end in rest.char_indices().map(|(offset, _)| offset).chain(std::iter::once(rest.len())) {
                    self.candidate(index, rest, values, end, separator);
                    if self.exhausted || self.ambiguous {
                        break;
                    }
                }
            } else {
                let mut offset = 0;
                while let Some(tail) = rest.get(offset..) {
                    let Some(at) = tail.find(separator) else {
                        break;
                    };
                    let end = offset + at;
                    self.candidate(index, rest, values, end, separator);
                    if self.exhausted || self.ambiguous {
                        break;
                    }
                    let Some(character) = rest.get(end..).and_then(|text| text.chars().next()) else {
                        break;
                    };
                    // Advance one character, including overlapping literal occurrences.
                    offset = end + character.len_utf8();
                }
            }
        }
    }
    let mut search = Search { literals, names, budget: 4096, exhausted: false, ambiguous: false, answer: None };
    search.visit(0, remaining, &mut Vec::with_capacity(names.len()));
    if search.exhausted || search.ambiguous { None } else { search.answer }
}

fn capture<'a>(template: &'a str, message: &'a str) -> Option<Vec<(&'a str, &'a str)>> {
    let parts = tokens(template);
    let literals: Vec<&str> = parts
        .iter()
        .filter_map(|part| match part {
            Token::Literal(value) => Some(value.as_str()),
            _ => None,
        })
        .collect();
    let names: Vec<&str> = parts
        .iter()
        .filter_map(|part| match part {
            Token::Field(name) => Some(*name),
            _ => None,
        })
        .collect();
    let values = unique_capture(&literals, &names, message)?;
    Some(names.into_iter().zip(values).collect())
}

/// Substitute one pass: a value containing braces is never interpreted as another placeholder.
fn fill(template: &str, values: &[(&str, &str)], depth: usize) -> String {
    let mut output = String::with_capacity(template.len());
    let mut unnamed = 0;
    for part in tokens(template) {
        let Token::Field(field) = part else {
            if let Token::Literal(text) = part {
                output.push_str(&text);
            }
            continue;
        };
        let name = field.split(':').next().unwrap_or(field);
        let found = if name.is_empty() {
            let next = values.iter().filter(|(key, _)| key.split(':').next() == Some("")).nth(unnamed);
            unnamed += 1;
            next
        } else {
            values.iter().find(|(key, _)| *key == field)
        };
        if let Some((_, value)) = found {
            if depth > 1 && matches!(name, "e" | "err" | "error" | "reason" | "message" | "msg" | "why") {
                output.push_str(&reason_at(value, depth - 1));
            } else {
                output.push_str(value);
            }
        } else {
            output.push('{');
            output.push_str(field);
            output.push('}');
        }
    }
    output
}

#[cfg(test)]
mod unique_capture_tests {
    use super::unique_capture;

    #[test]
    fn later_literals_disambiguate_repeated_separator_text() {
        assert_eq!(
            unique_capture(&["Move ", " to ", "; then copy to ", ""], &["from", "to", "to"], "Move A to B; then copy to B"),
            Some(vec!["A", "B", "B"])
        );
    }

    #[test]
    fn repeated_fields_can_resolve_adjacent_fields_without_parsing_user_data() {
        assert_eq!(unique_capture(&["Reference ", " is ", "", ""], &["letter", "letter", "len"], "Reference A is A12"), Some(vec!["A", "A", "12"]));
        assert_eq!(unique_capture(&["Value ", "", ""], &["one", "two"], "Value ABC"), None);
    }

    #[test]
    fn ambiguous_values_and_inconsistent_repetitions_are_rejected() {
        assert_eq!(unique_capture(&["Cannot write ", ": ", ""], &["path", "reason"], "Cannot write name: inside: message"), None);
        assert_eq!(unique_capture(&["Compare ", " with ", ""], &["name", "name"], "Compare one with two"), None);
    }

    #[test]
    fn utf8_braces_and_apostrophes_remain_opaque() {
        assert_eq!(
            unique_capture(&["Open ", ": ", ""], &["path", "reason"], "Open ملف {reason} O'Brien.pdf: خطأ"),
            Some(vec!["ملف {reason} O'Brien.pdf", "خطأ"])
        );
    }

    #[test]
    fn limits_return_no_match_instead_of_recursing_without_bound() {
        assert!(unique_capture(&["", "", ""], &["a", "a"], &"x".repeat(10_000)).is_none());
        assert!(unique_capture(&["Open ", ""], &["path"], &format!("Open {}", "x".repeat(65_536))).is_none());
        let names = vec!["x"; 33];
        let literals = vec![""; 34];
        assert!(unique_capture(&literals, &names, "x").is_none());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn october_diagnostics_translate_new_workflows_without_touching_values() {
        for (original, expected) in [
            ("the open file isn't in a live session", "الملف المفتوح غير مشارك في جلسة مباشرة"),
            ("invalid path coordinates", "إحداثيات مسار غير صالحة"),
            ("properties: {name: value}", "الخصائص: {name: value}"),
        ] {
            assert_eq!(reason_at(original, 3), expected);
        }
        let opaque = "ملف O'Brien {name} 12";
        let message = format!("couldn't remove {opaque}: the server answered something else");
        assert_eq!(reason_at(&message, 3), format!("تعذّر إخراج {opaque}: أعاد الخادم استجابة غير متوقعة"));
        let message = format!("step 2: step 1's result has no `{opaque}`");
        assert_eq!(reason_at(&message, 3), format!("الخطوة 2: لا تتضمن نتيجة الخطوة 1 الحقل `{opaque}`"));
        let message = format!("`position` is an auto layout property, and layer #12 “{opaque}” is a TEXT (only frames, components and instances have auto layout)");
        let translated = reason_at(&message, 3);
        assert_ne!(translated, message);
        for value in ["position", "#12", opaque, "TEXT"] {
            assert!(translated.contains(value));
        }
    }

    #[test]
    fn diagnostic_values_keep_braces_apostrophes_arabic_and_ids() {
        let original = "مشروع PhotoCraft {literal} O'Brien 12";
        let template = "a collection named `{name}` already exists";
        let message = format!("a collection named `{original}` already exists");
        let captures = capture(template, &message).unwrap();
        assert_eq!(captures, [("name", original)]);
        let translated = reason_at(&message, 3);
        assert!(translated.contains(original));
        assert_ne!(translated, message);
        let opaque = "unknown external failure {a} O'Brien";
        assert_eq!(reason_at(opaque, 3), opaque);
    }

    #[test]
    fn format_specs_and_anonymous_values_are_not_reformatted() {
        let source = "`{key}` must be one of {vals:?}";
        let message = "`fontStyle` must be one of [\"Regular\", \"SemiBold\"]";
        let captures = capture(source, message).unwrap();
        let filled = fill("تقبل `{key}` القيم {vals:?}", &captures, 3);
        assert_eq!(filled, "تقبل `fontStyle` القيم [\"Regular\", \"SemiBold\"]");
        let captures = capture("a {0} × {1} page is too large for PDF", "a 12 × 35 page is too large for PDF").unwrap();
        assert_eq!(fill("{0} × {1}", &captures, 3), "12 × 35");
        let captures = capture("\"{}\" publishes no component {v}", "\"{my file}\" publishes no component #123").unwrap();
        assert_eq!(fill("{v} · {}", &captures, 3), "#123 · {my file}");
        let captures = capture("{name} and {name:?}", "A and \"A\"").unwrap();
        assert_eq!(fill("{name:?} · {name}", &captures, 3), "\"A\" · A");
        assert!(capture("{name} and {name}", "A and B").is_none());
    }

    #[test]
    fn escaped_braces_json_and_user_values_are_single_pass() {
        let source = "unknown field `{k}`: expected {{id: value}} (got {actual:?})";
        let name = "ملف O'Brien {literal}";
        let actual = "\"{tag: true}\"";
        let message = format!("unknown field `{name}`: expected {{id: value}} (got {actual})");
        let values = capture(source, &message).unwrap();
        assert_eq!(values, [("k", name), ("actual:?", actual)]);
        assert_eq!(
            fill("الحقل `{k}` غير معروف: المتوقع {{id: value}} (القيمة {actual:?})", &values, 3),
            format!("الحقل `{name}` غير معروف: المتوقع {{id: value}} (القيمة {actual})")
        );
        assert_eq!(fill("استخدم {\"force\":true} ثم {path}", &[("path", "{ملف} O'Brien")], 3), "استخدم {\"force\":true} ثم {ملف} O'Brien");
        assert!(capture("pass {\"force\":true} to {action}", "pass {\"force\":true} to discard").is_some());
        assert!(!has_prose_template("{msg}"));
        assert!(!has_prose_template("{name}{line}: {msg}"));
        assert!(capture("{name} and {other}", "A and B and C").is_none());
        for field in ["name:?", "value:.2", "value:>8", "value:0width$x", "_1", "0"] {
            assert!(valid_field(field), "{field}");
        }
        for field in ["\"force\":true", "id: value", "bad field", "name:garbage"] {
            assert!(!valid_field(field), "{field}");
        }
    }
}
