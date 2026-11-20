/// Basic statistics for a UTF-8 text document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextStats {
    pub bytes: usize,
    pub characters: usize,
    pub words: usize,
    pub lines: usize,
}

pub fn stats(input: &str) -> TextStats {
    TextStats {
        bytes: input.len(),
        characters: input.chars().count(),
        words: input.split_whitespace().count(),
        lines: input.lines().count(),
    }
}

/// Normalize CRLF and standalone CR line endings to LF.
pub fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_utf8_text() {
        let result = stats("Hola, mundo\n¡Amílcar!");
        assert_eq!(result.bytes, "Hola, mundo\n¡Amílcar!".len());
        assert_eq!(result.characters, 21);
        assert_eq!(result.words, 3);
        assert_eq!(result.lines, 2);
    }

    #[test]
    fn normalizes_newlines() {
        assert_eq!(normalize_newlines("a\r\nb\rc\n"), "a\nb\nc\n");
    }

    #[test]
    fn empty_text_has_zero_counts() {
        assert_eq!(
            stats(""),
            TextStats {
                bytes: 0,
                characters: 0,
                words: 0,
                lines: 0
            }
        );
    }
}
