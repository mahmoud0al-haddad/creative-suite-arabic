//! Translate rendered diagnostics while preserving the English engine/control data.
use super::{CURRENT, LANGUAGES, catalog};
use std::collections::HashMap;
use std::sync::OnceLock;

type Templates = HashMap<&'static str, Vec<(&'static str, &'static str)>>;
fn templates(language: &str) -> &'static [(&'static str, &'static str)] {
    static TEMPLATES: OnceLock<Templates> = OnceLock::new();
    TEMPLATES
        .get_or_init(|| {
            let mut all = HashMap::new();
            for language in &LANGUAGES {
                if let Some(catalog) = catalog(language.code) {
                    let mut entries: Vec<_> = catalog
                        .iter()
                        .filter(|((context, source), _)| *context == "@msg" && has_prose_template(source))
                        .map(|((_, source), translation)| (*source, *translation))
                        .collect();
                    entries.sort_by_key(|(source, _)| {
                        std::cmp::Reverse(
                            source.split('{').map(|part| part.split_once('}').map_or(part.len(), |(_, after)| after.len())).sum::<usize>(),
                        )
                    });
                    all.insert(language.code, entries);
                }
            }
            all
        })
        .get(language)
        .map(Vec::as_slice)
        .unwrap_or_default()
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
    if names.len() > 12 {
        return None;
    }
    let values = unique_capture(&literals, &names, message)?;
    Some(names.into_iter().zip(values).collect())
}

/// Values are already translated only where the caller recognizes nested diagnostics.
/// Their braces and quotes remain opaque during this single substitution pass.
fn render(template: &str, values: &[(&str, String)]) -> String {
    let mut result = String::with_capacity(template.len());
    for part in tokens(template) {
        match part {
            Token::Literal(literal) => result.push_str(&literal),
            Token::Field(name) => {
                if let Some((_, value)) = values.iter().find(|(key, _)| *key == name) {
                    result.push_str(value);
                } else {
                    result.push('{');
                    result.push_str(name);
                    result.push('}');
                }
            }
        }
    }
    result
}

fn translate(language: &str, text: &str, depth: usize) -> String {
    let Some(catalog) = catalog(language) else {
        return text.to_owned();
    };
    if let Some(value) = catalog.get(&("@msg", text)) {
        return (*value).to_owned();
    }
    // Prompts and static status labels also use the ordinary catalog, in every language.
    if let Some(value) = catalog.get(&("", text)) {
        return (*value).to_owned();
    }
    if text.len() > 2000 || depth == 0 {
        return text.to_owned();
    }
    // Specific templates are sorted once, rather than reallocated for each history row/frame.
    for (source, template) in templates(language) {
        if let Some(values) = capture(source, text) {
            let values: Vec<_> = values
                .into_iter()
                .map(|(name, value)| {
                    let plural_suffix =
                        value == "s" && ["reference", "attribute", "row"].iter().any(|noun| source.contains(&format!("{noun}{{{name}}}")));
                    (
                        name,
                        if plural_suffix {
                            String::new()
                        } else if matches!(name, "e" | "err" | "error" | "reason" | "message" | "msg" | "status")
                            || (name == "1" && source.starts_with("command `") && source.contains("` is not available right now:"))
                            || (name == "0" && source.starts_with("syntax error:"))
                            || (name == "_1" && matches!(*source, "{path} can't be attached: {_1}" | "{path}: {_1}"))
                        {
                            translate(language, value, depth - 1)
                        } else {
                            value.to_owned()
                        },
                    )
                })
                .collect();
            return render(template, &values);
        }
    }
    text.to_owned()
}

pub fn message(language: &str, text: &str) -> String {
    translate(language, text, 3)
}
pub fn msg(text: &str) -> String {
    CURRENT.with(|language| message(language.get(), text))
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
    fn literal_json_escaped_braces_and_user_names_are_preserved() {
        let source = "bad config {\"force\":true}: {e}";
        let value = r#"ملف C:/Folder/{name} "quoted".dxf"#;
        let message = format!("bad config {{\"force\":true}}: {value}");
        assert_eq!(capture(source, &message), Some(vec![("e", value)]));
        let escaped = "bad config {{\"force\":true}}: {e}";
        assert_eq!(capture(escaped, &message), Some(vec![("e", value)]));
        let rendered = render("إعداد غير صالح {{\"force\":true}}: {e}", &[("e", value.to_owned())]);
        assert_eq!(rendered, format!("إعداد غير صالح {{\"force\":true}}: {value}"));
        assert_eq!(capture("object {id: value}: {e}", "object {id: value}: broken"), Some(vec![("e", "broken")]));
        assert!(!has_prose_template("{path}: {e}"), "opaque values are not a prose diagnostic");
        assert!(has_prose_template("Couldn't open {path}: {e}"));
        assert_eq!(
            capture("Couldn't open {path}: {e}", "Couldn't open C:/Folder/File123.dxf: broken"),
            Some(vec![("path", "C:/Folder/File123.dxf"), ("e", "broken")])
        );
        assert_eq!(
            capture("Couldn't open {path}: {e}", "Couldn't open File123.dxf: broken: twice"),
            None,
            "ambiguous delimiters preserve the original text"
        );
    }

    #[test]
    fn captures_unicode_values_and_preserves_paths() {
        assert_eq!(capture("Couldn't open {path}: {e}", "Couldn't open ملف.dxf: broken"), Some(vec![("path", "ملف.dxf"), ("e", "broken")]));
        assert_eq!(capture("{a}{b}", "anything"), None);
        assert_eq!(capture("Error: {e}", "Ok: no"), None);
    }
}
