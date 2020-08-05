use once_cell::sync::Lazy;
use regex::Regex;
/// The Card is the main unit of analysis. We use it to represent a
/// zettel file.
use std::path::{Path, PathBuf};

type Tag = String;

static TAG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[[:alnum:]:-]+").unwrap());

#[derive(Debug, Clone)]
pub struct Card {
    /// The zettel on the filesystem.
    // I'm not decided on whether I should immediately read the zettel
    // into memory, considering if that would impact tracking updates
    // to the zettel.
    path: PathBuf,
}

impl Card {
    /// Looks through a card and attempts to extract anything that
    /// resembles a tag. A tag looks something like this:
    /// #abc123-def456:ghi-789 (something alphanumeric with dashes
    /// and/or colons, that starts with a pound)
    pub fn extract_tags(&self) -> Vec<Tag> {
        TAG_REGEX
            .find_iter(std::fs::read_to_string(&self.path).unwrap().as_str())
            .map(|m| String::from(m.as_str()))
            .collect::<Vec<Tag>>()
    }
}

impl std::convert::From<PathBuf> for Card {
    fn from(path: PathBuf) -> Self {
        Self { path: path }
    }
}

impl std::convert::From<&Path> for Card {
    fn from(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_path() {
        let path = PathBuf::from("some-file");
        let card = Card::from(path.as_path());
        assert_eq!(card.path, path);
    }
}
