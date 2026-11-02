//! Adapted from ArtCraft Launcher (MIT OR Apache-2.0); original notices retained.
//! TSV catalog parsing for UI localisation.
//!
//! Format (one entry per line):
//!   context<TAB>source<TAB>translation
//! - Empty context = plain UI string (`tl!`, `t`).
//! - `@id` context = keyed by command id (`tr_id`).
//! - `@plural` context = plural forms: the English `one|other` key, and the language's forms
//!   separated by `|` (one form for a language whose nouns don't inflect for number, up to
//!   three: `one|few|many` for Russian).
//! - `@msg` context = a status/error message, or a template whose `{placeholders}` match any text
//!   (`message`, `msg`).
//! - Any other context = disambiguation for `tr_ctx`.
//!
//! Escapes in source/translation: `\t` `\n` `\\` (parsed once at load time).
//! Placeholders `{name}` must appear in both source and translation.
//! A trailing `…` (U+2026) in source must be preserved in translation.

use std::collections::HashMap;

#[path = "message_tokens.rs"]
mod message_tokens;

/// A parsed translation catalog for one language.
#[derive(Debug, Default)]
pub struct Catalog {
    /// Plain strings: English -> translation.
    plain: HashMap<String, String>,
    /// Contextual strings: context -> (English -> translation).
    contextual: HashMap<String, HashMap<String, String>>,
    /// Command-id keyed strings: id -> translation.
    by_id: HashMap<String, String>,
    /// Plural forms: English "one|other" -> the language's forms.
    plural: HashMap<String, Vec<String>>,
    /// Messages: English message or template -> translation.
    messages: HashMap<String, String>,
    /// The `@msg` rows with placeholders, matched against messages that have no exact row.
    templates: Vec<Template>,
}

/// An `@msg` template, split into its literal text and placeholders.
#[derive(Debug)]
struct Template {
    /// Literal text around the placeholders: always one more than `names`.
    literals: Vec<String>,
    names: Vec<String>,
    translation: String,
}

impl Template {
    fn new(source: &str, translation: String) -> Option<Self> {
        let mut literals = Vec::new();
        let mut names = Vec::new();
        for token in message_tokens::tokens(source) {
            match token {
                message_tokens::Token::Literal(text) => literals.push(text),
                message_tokens::Token::Field(name) => names.push(name.to_string()),
            }
        }
        // A filename followed by a typed reason is a diagnostic boundary, not a generic wrapper.
        let typed_reason = names.len() == 2
            && names.first().is_some_and(|name| matches!(name.as_str(), "name" | "path" | "file"))
            && names.last().is_some_and(|name| matches!(name.as_str(), "e" | "error" | "reason"))
            && literals.first().is_some_and(String::is_empty)
            && literals.get(1).is_some_and(|literal| literal == ": ")
            && literals.last().is_some_and(String::is_empty);
        // A wrapper such as {msg} cannot distinguish an error from arbitrary user data.
        if names.is_empty() || (!typed_reason && !literals.iter().any(|text| text.chars().any(char::is_alphabetic))) {
            return None;
        }
        Some(Self { literals, names, translation })
    }

    /// The length of the template's literal text: the more of a message a template spells out,
    /// the better it fits.
    fn specificity(&self) -> usize {
        self.literals.iter().map(String::len).sum()
    }

    /// The translation with this message's placeholder values filled in, if `s` fits the template.
    /// Only a unique complete decomposition is accepted, within fixed work and field limits.
    fn apply(&self, s: &str, catalog: &Catalog, depth: usize) -> Option<String> {
        let literals: Vec<&str> = self.literals.iter().map(String::as_str).collect();
        let names: Vec<&str> = self.names.iter().map(String::as_str).collect();
        let values = message_tokens::unique_capture(&literals, &names, s).filter(|values| values.iter().all(|value| !value.is_empty()))?;
        let values: Vec<String> = self
            .names
            .iter()
            .zip(values)
            .map(|(name, value)| {
                if depth < 4 && matches!(name.as_str(), "e" | "error" | "reason" | "f" | "msg" | "message" | "1") {
                    catalog.message_depth(value, depth + 1).unwrap_or_else(|| value.to_owned())
                } else {
                    value.to_owned()
                }
            })
            .collect();
        let args: Vec<(&str, &str)> = self.names.iter().map(String::as_str).zip(values.iter().map(String::as_str)).collect();
        Some(message_tokens::render(&self.translation, &args))
    }
}

impl Catalog {
    /// Parse a catalog from TSV content. Malformed rows are logged and skipped.
    pub fn parse(source: &str) -> Self {
        let mut cat = Catalog::default();
        for (line_no, line) in source.lines().enumerate() {
            let line_no = line_no + 1;
            // Only the end: a plain row starts with its (empty) context and a TAB.
            let line = line.trim_end();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split('\t').collect();
            let [context, source_text, translation] = parts.as_slice() else {
                log::warn!("i18n catalog: line {line_no}: expected 3 tab-separated fields, got {}", parts.len());
                continue;
            };
            let (context, source_text, translation) = (unescape(context), unescape(source_text), unescape(translation));
            let duplicate = match context.as_str() {
                "" => cat.plain.insert(source_text.clone(), translation).is_some(),
                "@id" => cat.by_id.insert(source_text.clone(), translation).is_some(),
                "@plural" => {
                    let forms: Vec<String> = translation.split('|').map(str::to_string).collect();
                    if forms.len() > 6 {
                        log::warn!("i18n catalog: line {line_no}: @plural expects 1 to 6 forms separated by '|', got {}", forms.len());
                        continue;
                    }
                    cat.plural.insert(source_text.clone(), forms).is_some()
                }
                "@msg" => {
                    if source_text.contains('{') {
                        match Template::new(&source_text, translation.clone()) {
                            Some(t) => cat.templates.push(t),
                            None => log::warn!("i18n catalog: line {line_no}: @msg template can't be matched: {source_text:?}"),
                        }
                    }
                    cat.messages.insert(source_text.clone(), translation).is_some()
                }
                _ => cat.contextual.entry(context.clone()).or_default().insert(source_text.clone(), translation).is_some(),
            };
            if duplicate {
                log::warn!("i18n catalog: line {line_no}: duplicate key {context:?} {source_text:?}");
            }
        }
        // The most specific template first.
        cat.templates.sort_by_key(|t| std::cmp::Reverse(t.specificity()));
        cat
    }

    /// Look up a plain string.
    pub fn plain(&self, s: &str) -> Option<&str> {
        self.plain.get(s).map(String::as_str)
    }

    /// Look up a contextual string.
    pub fn contextual(&self, context: &str, s: &str) -> Option<&str> {
        self.contextual.get(context)?.get(s).map(String::as_str)
    }

    /// Look up a command-id keyed string.
    pub fn id(&self, id: &str) -> Option<&str> {
        self.by_id.get(id).map(String::as_str)
    }

    /// Look up plural forms for an English "one|other" key; the form at `index` (0 = one).
    /// A row with fewer forms than the language's rule uses its last form for the rest.
    pub fn plural(&self, key: &str, index: usize) -> Option<&str> {
        let forms = self.plural.get(key)?;
        forms.get(index).or_else(|| forms.last()).map(String::as_str)
    }

    /// Translate a message: an exact `@msg` row, else the most specific template it fits.
    pub fn message(&self, s: &str) -> Option<String> {
        self.message_depth(s, 0)
    }

    fn message_depth(&self, s: &str, depth: usize) -> Option<String> {
        if let Some(t) = self.messages.get(s) {
            return Some(t.clone());
        }
        self.templates.iter().find_map(|t| t.apply(s, self, depth))
    }
}

/// Unescape `\t`, `\n`, `\\` in a TSV field.
pub(super) fn unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod diagnostic_template_regressions {
    use super::Catalog;

    #[test]
    fn formatted_json_braces_are_literal_and_values_remain_opaque() {
        let catalog =
            Catalog::parse("@msg\tCould not parse {{\"force\":true}} for {path}: {reason}\tتعذر تحليل {{\"force\":true}} للملف {path}: {reason}\n");
        let message = r#"Could not parse {"force":true} for C:\العربية\O'Brien {reason}.pdf: invalid"#;
        assert_eq!(catalog.message(message).as_deref(), Some(r#"تعذر تحليل {"force":true} للملف C:\العربية\O'Brien {reason}.pdf: invalid"#));
        let plain = Catalog::parse("@msg\tCannot use {path}\tتعذر استخدام {path}\n");
        assert_eq!(plain.message("Cannot use ملف {path} {{x}}.pdf").as_deref(), Some("تعذر استخدام ملف {path} {{x}}.pdf"));
    }

    #[test]
    fn pure_wrappers_cannot_translate_arbitrary_user_data() {
        let catalog = Catalog::parse("@msg\t{msg}\tرسالة {msg}\n@msg\t{name}{line}: {msg}\t{name}{line}: {msg}\n");
        assert_eq!(catalog.message("report {name}.pdf"), None);
        assert_eq!(catalog.message("a.txt12: a user title"), None);
    }

    #[test]
    fn ambiguous_or_inconsistent_captures_are_not_guessed() {
        let catalog = Catalog::parse(
            "@msg\tCannot write {name}: {reason}\tتعذر كتابة {name}: {reason}\n@msg\tCompare {name} with {name}\tقارن {name} مع {name}\n",
        );
        assert_eq!(catalog.message("Cannot write name: inside: message"), None);
        assert_eq!(catalog.message("Compare one with two"), None);
        assert_eq!(catalog.message("Compare واحد with واحد").as_deref(), Some("قارن واحد مع واحد"));
    }
}
