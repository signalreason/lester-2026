use std::collections::HashSet;

use crate::models::{TagSource, TagSuggestion};

pub struct TaggingRules {
    stopwords: HashSet<String>,
}

impl TaggingRules {
    pub fn new() -> Self {
        let words = [
            "the", "and", "for", "with", "that", "this", "from", "into", "your", "you", "are",
            "was", "were", "have", "has", "about", "http", "https",
        ];
        let stopwords = words.iter().map(|word| word.to_string()).collect();
        Self { stopwords }
    }

    pub fn suggest(&self, url: &str, title: &str) -> Vec<TagSuggestion> {
        let mut suggestions = Vec::new();

        if let Some(domain) = extract_domain(url) {
            suggestions.push(TagSuggestion {
                name: domain,
                confidence: 0.72,
                source: TagSource::Rules,
            });
        }

        for keyword in extract_keywords(title, &self.stopwords) {
            suggestions.push(TagSuggestion {
                name: keyword,
                confidence: 0.6,
                source: TagSource::Rules,
            });
        }

        suggestions
    }
}

fn extract_domain(url: &str) -> Option<String> {
    let trimmed = url.trim();
    let without_scheme = trimmed
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let domain = without_scheme.split('/').next()?;
    let domain = domain.trim_start_matches("www.");
    if domain.is_empty() {
        None
    } else {
        Some(domain.to_string())
    }
}

fn extract_keywords(text: &str, stopwords: &HashSet<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split(|c: char| !c.is_alphanumeric())
        .filter_map(|raw| {
            let candidate = raw.trim().to_lowercase();
            if candidate.len() < 4 {
                return None;
            }
            if stopwords.contains(&candidate) {
                return None;
            }
            if seen.insert(candidate.clone()) {
                Some(candidate)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suggests_domain_and_keywords() {
        let rules = TaggingRules::new();
        let suggestions = rules.suggest(
            "https://news.ycombinator.com/item?id=1",
            "OpenAI releases new research model",
        );

        let names: Vec<_> = suggestions.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"news.ycombinator.com"));
        assert!(names.contains(&"openai"));
        assert!(names.contains(&"research"));
        assert!(names.contains(&"model"));
    }
}
