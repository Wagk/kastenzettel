use regex::Regex;
/// The Card is the main unit of analysis. We use it to represent a
/// zettel file.
use std::path::{Path, PathBuf};

pub struct Card {
    /// The zettel on the filesystem.
    // I'm not decided on whether I should immediately read the zettel
    // into memory, considering if that would impact tracking updates
    // to the zettel.
    path: PathBuf,
}

impl Card {}

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
