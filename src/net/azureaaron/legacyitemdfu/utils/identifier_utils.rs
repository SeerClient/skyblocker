//! Ported from net.azureaaron.legacyitemdfu.utils.IdentifierUtils (legacy-item-dfu 1.0.4+26.2).

/// java: public final class IdentifierUtils
pub struct IdentifierUtils;

impl IdentifierUtils {
    /// java: public static String tryParse(String id) - `null` is `None`.
    pub fn try_parse(id: &str) -> Option<String> {
        IdentifierUtils::try_split_on(id, ':')
    }

    /// java: private static String trySplitOn(String id, char delimiter)
    fn try_split_on(id: &str, delimiter: char) -> Option<String> {
        // `indexOf` / `substring` are on an ASCII delimiter, so the byte offsets are the char offsets Java slices at.
        match id.find(delimiter) {
            Some(i) => {
                let string = &id[i + delimiter.len_utf8()..];
                if !IdentifierUtils::is_path_valid(string) {
                    None
                } else if i != 0 {
                    let string2 = &id[..i];
                    if IdentifierUtils::is_namespace_valid(string2) { Some(format!("{string2}:{string}")) } else { None }
                } else {
                    Some(format!("minecraft:{string}"))
                }
            }
            None => {
                if IdentifierUtils::is_path_valid(id) {
                    Some(format!("minecraft:{id}"))
                } else {
                    None
                }
            }
        }
    }

    /// java: private static boolean isNamespaceValid(String namespace)
    fn is_namespace_valid(namespace: &str) -> bool {
        // Every valid character is ASCII, so a non-ASCII `char` fails here exactly as each of its UTF-16 units does in Java.
        namespace.chars().all(IdentifierUtils::is_namespace_character_valid)
    }

    /// java: private static boolean isPathValid(String path)
    fn is_path_valid(path: &str) -> bool {
        path.chars().all(IdentifierUtils::is_path_character_valid)
    }

    /// java: private static boolean isNamespaceCharacterValid(char character)
    fn is_namespace_character_valid(character: char) -> bool {
        character == '_' || character == '-' || character >= 'a' && character <= 'z' || character >= '0' && character <= '9' || character == '.'
    }

    /// java: private static boolean isPathCharacterValid(char character)
    fn is_path_character_valid(character: char) -> bool {
        character == '_' || character == '-' || character >= 'a' && character <= 'z' || character >= '0' && character <= '9' || character == '/' || character == '.'
    }
}

#[cfg(test)]
mod tests {
    use super::IdentifierUtils;

    #[test]
    fn a_bare_path_gets_the_minecraft_namespace() {
        assert_eq!(IdentifierUtils::try_parse("stone").as_deref(), Some("minecraft:stone"));
        assert_eq!(IdentifierUtils::try_parse(":stone").as_deref(), Some("minecraft:stone"));
        assert_eq!(IdentifierUtils::try_parse("skyblock:stone").as_deref(), Some("skyblock:stone"));
        assert_eq!(IdentifierUtils::try_parse("STONE"), None);
        assert_eq!(IdentifierUtils::try_parse("a:b:c"), None);
    }
}
