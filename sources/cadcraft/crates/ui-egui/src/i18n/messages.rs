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
                        .filter(|((context, source), _)| *context == "@msg" && source.contains('{'))
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

fn capture<'a>(source: &'a str, text: &'a str) -> Option<Vec<(&'a str, &'a str)>> {
    let mut source = source;
    let mut text = text;
    let mut values = Vec::new();
    while let Some((literal, after)) = source.split_once('{') {
        text = text.strip_prefix(literal)?;
        let (name, tail) = after.split_once('}')?;
        if values.len() >= 12 {
            return None;
        }
        let delimiter = tail.split('{').next().unwrap_or_default();
        if delimiter.is_empty() && !tail.is_empty() {
            return None;
        }
        if tail.is_empty() {
            values.push((name, text));
            text = "";
        } else {
            let end = text.find(delimiter)?;
            values.push((name, text.get(..end)?));
            text = text.get(end..)?;
        }
        source = tail;
    }
    (source == text).then_some(values)
}

fn render(template: &str, values: &[(&str, String)]) -> String {
    let mut result = String::new();
    let mut rest = template;
    while let Some((literal, after)) = rest.split_once('{') {
        result.push_str(literal);
        let Some((name, tail)) = after.split_once('}') else {
            result.push('{');
            rest = after;
            break;
        };
        if let Some((_, value)) = values.iter().find(|(key, _)| *key == name) {
            result.push_str(value);
        } else {
            result.push('{');
            result.push_str(name);
            result.push('}');
        }
        rest = tail;
    }
    result.push_str(rest);
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
mod tests {
    use super::*;
    #[test]
    fn captures_unicode_values_and_preserves_paths() {
        assert_eq!(capture("Couldn't open {path}: {e}", "Couldn't open ملف.dxf: broken"), Some(vec![("path", "ملف.dxf"), ("e", "broken")]));
        assert_eq!(capture("{a}{b}", "anything"), None);
        assert_eq!(capture("Error: {e}", "Ok: no"), None);
    }
}
