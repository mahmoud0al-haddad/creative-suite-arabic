//! Display-only Arabic diagnostics; no canonical error, command or document data changes.
use effectcraft_engine::EngineError;
include!("diagnostic_arabic.rs");
fn arabic() -> bool {
    crate::i18n::current() == "ar"
}
pub(crate) fn display(error: &EngineError) -> String {
    if !arabic() {
        return error.to_string();
    }
    match error {
        EngineError::UnknownCommand(command) => format!("أمر غير معروف: `{command}`"),
        EngineError::Disabled(command, reason) => format!("الأمر `{command}` غير متاح حاليًا: {}", reason_at(reason, 3)),
        EngineError::BadParams { cmd, msg } => format!("معاملات غير صالحة للأمر `{cmd}`: {}", reason_at(msg, 3)),
        EngineError::NoComp => "لا يوجد تركيب نشط".into(),
        EngineError::NoSuchComp(name) => format!("لا يوجد تركيب {name}"),
        EngineError::Project(error) => reason_at(&error.to_string(), 3),
        EngineError::Other(reason) => reason_at(reason, 3),
    }
}

pub(crate) fn message(message: &str) -> String {
    if !arabic() {
        return message.into();
    }
    reason_at(message, 3)
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
        if let Some(mut values) = capture_with_translation(source, translated, message) {
            // This one field is a known English plural suffix, not a document value.
            // Arabic uses a quantity-neutral plural here. Keep every other captured value.
            if *source == "{_1} already in {folder}: nothing was moved. Rename the photo{s_} first or choose another folder" {
                for (key, value) in &mut values {
                    if *key == "s_" && matches!(*value, "" | "s") {
                        *value = "";
                    }
                }
            }
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
            if depth > 1 && matches!(name, "e" | "err" | "error" | "reason" | "message" | "msg" | "why" | "how_to_merge") {
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
fn set_test_language(arabic: bool) {
    crate::i18n::set_current(if arabic { "ar" } else { "en" });
}

/// Adjacent fields may be kept as one opaque value only when the translation preserves
/// their exact order and neither source nor translation reuses any of those fields.
fn capture_with_translation<'a>(source: &'a str, translated: &str, message: &'a str) -> Option<Vec<(&'a str, &'a str)>> {
    if let Some(values) = capture(source, message) {
        return Some(values);
    }
    let parts = tokens(source);
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
    if names.is_empty() || names.len() > 32 || literals.len() != names.len() + 1 {
        return None;
    }
    let translated_parts = tokens(translated);
    let mut merged_names = Vec::new();
    let mut merged_literals = vec![*literals.first()?];
    let mut groups = Vec::new();
    let mut index = 0;
    while index < names.len() {
        let mut end = index;
        while end + 1 < names.len() && literals.get(end + 1).is_some_and(|literal| literal.is_empty()) {
            end += 1;
        }
        if end > index {
            let group = names.get(index..=end)?;
            let mut spelling = String::new();
            for name in group {
                if name.is_empty()
                    || names.iter().filter(|item| *item == name).count() != 1
                    || translated_parts.iter().filter(|part| matches!(part, Token::Field(field) if field == name)).count() != 1
                {
                    return None;
                }
                spelling.push('{');
                spelling.push_str(name);
                spelling.push('}');
            }
            if !translated.contains(&spelling) {
                return None;
            }
        }
        merged_names.push(*names.get(index)?);
        merged_literals.push(*literals.get(end + 1)?);
        groups.push((index, end));
        index = end + 1;
    }
    if merged_names.len() == names.len() {
        return None;
    }
    let captured = unique_capture(&merged_literals, &merged_names, message)?;
    let mut values = Vec::with_capacity(names.len());
    for ((start, end), value) in groups.into_iter().zip(captured) {
        for at in start..=end {
            values.push((*names.get(at)?, if at == start { value } else { "" }));
        }
    }
    Some(values)
}

#[cfg(test)]
mod unique_capture_tests {
    use super::unique_capture;

    #[test]
    fn later_literals_disambiguate_repeated_separator_text() {
        assert_eq!(unique_capture(&["Move ", " to ", "; then copy to ", ""], &["from", "to", "to"], "Move A to B; then copy to B"), Some(vec!["A", "B", "B"]));
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
    fn typed_errors_preserve_machine_results_and_opaque_values() {
        set_test_language(false);
        let name = "ملف PhotoCraft {literal} O'Brien 12";
        let error = EngineError::UnknownCommand(name.into());
        let original = error.to_string();
        assert_eq!(display(&error), original);
        set_test_language(true);
        assert_eq!(display(&error), format!("أمر غير معروف: `{name}`"));
        assert_eq!(error.to_string(), original);
        let error = EngineError::BadParams { cmd: "file.open".into(), msg: "no active document".into() };
        assert!(display(&error).contains("file.open"));
        assert!(display(&error).starts_with("معاملات غير صالحة"));
        let unknown = "external diagnostic {user} O'Brien";
        assert_eq!(message(unknown), unknown);
        set_test_language(false);
    }
    #[test]
    fn templates_keep_json_braces_specs_and_user_data() {
        let source = "unknown field `{k}`: expected {{id: value}} (got {actual:?})";
        let values = capture(source, "unknown field `اسم {user}`: expected {id: value} (got \"A\")").unwrap();
        assert_eq!(fill("الحقل `{k}`: {{id: value}} ({actual:?})", &values, 3), "الحقل `اسم {user}`: {id: value} (\"A\")");
        assert!(!has_prose_template("{msg}"));
        assert!(!has_prose_template("{name}{line}: {msg}"));
        assert!(capture("{name} and {name}", "A and B").is_none());
        assert_eq!(fill("مثال {\"force\": true} {path}", &[("path", "{ملف} O'Brien")], 3), "مثال {\"force\": true} {ملف} O'Brien");
    }
    #[test]
    fn reviewed_catalog_is_arabic_and_keeps_every_field() {
        for (source, translated) in AR_DIAGNOSTICS {
            assert!(translated.chars().any(|c| ('\u{0600}'..='\u{06ff}').contains(&c)), "{source}");
            let fields = |text: &str| {
                let mut names = tokens(text)
                    .into_iter()
                    .filter_map(|part| match part {
                        Token::Field(name) => Some(name.to_owned()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                names.sort();
                names
            };
            assert_eq!(fields(source), fields(translated), "{source}");
        }
    }
    #[test]
    fn every_reviewed_template_matches_formatted_samples() {
        for (source, translated) in AR_DIAGNOSTICS {
            if !has_prose_template(source) {
                continue;
            }
            let mut sample = String::new();
            let mut names = Vec::new();
            for part in tokens(source) {
                match part {
                    Token::Literal(text) => sample.push_str(&text),
                    Token::Field(name) => {
                        let index = names.iter().position(|previous| *previous == name).unwrap_or(names.len());
                        if index == names.len() {
                            names.push(name);
                        }
                        sample.push_str(&format!("D{index}Z"));
                    }
                }
            }
            let values = capture_with_translation(source, translated, &sample).unwrap_or_else(|| panic!("template cannot match: {source}"));
            let shown = fill(translated, &values, 1);
            assert!(shown.chars().any(|c| ('\u{0600}'..='\u{06ff}').contains(&c)), "{source}");
            for (_, value) in values {
                assert!(shown.contains(value), "missing {value}: {source}");
            }
        }
    }
}

#[cfg(test)]
mod adjacent_display_tests {
    use super::*;
    #[test]
    fn preserved_adjacent_groups_keep_the_entire_opaque_value() {
        let values = capture_with_translation("there is no track {letter}{n}", "لا يوجد المسار {letter}{n}", "there is no track A123").unwrap();
        assert_eq!(values, vec![("letter", "A123"), ("n", "")]);
        assert_eq!(fill("لا يوجد المسار {letter}{n}", &values, 1), "لا يوجد المسار A123");
        assert!(capture_with_translation("there is no track {letter}{n}", "لا يوجد المسار {n}{letter}", "there is no track A123").is_none());
        assert!(capture_with_translation("Cannot write {path}: {reason}", "تعذر كتابة {path}: {reason}", "Cannot write a: b: c").is_none());
    }
}
