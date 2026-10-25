//! Ported from io.github.moulberry.repo.NEURepositoryVersion (neurepoparser 1.12.0).

use std::sync::LazyLock;

/// The `neurepoparser.properties` resource of the library jar, verbatim (the 1.12.0 jar ships it with
/// `neurepository.parser.version=1.7.0`; the port keeps what the jar says).
const NEUREPOPARSER_PROPERTIES: &str = "neurepository.parser.version=1.7.0\nneurepository.schema.major=1\nneurepository.schema.minor=1\n";

/// java: public class NEURepositoryVersion
pub struct NEURepositoryVersion;

/// java: public static Properties VERSION_PROPERTIES = new Properties(); static { VERSION_PROPERTIES.load(
///       NEURepositoryVersion.class.getClassLoader().getResourceAsStream("neurepoparser.properties")); }
pub static VERSION_PROPERTIES: LazyLock<std::collections::HashMap<String, String>> = LazyLock::new(|| {
    NEUREPOPARSER_PROPERTIES
        .lines()
        .filter_map(|line| line.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
});

fn get_property(key: &str, default: &str) -> String {
    VERSION_PROPERTIES.get(key).cloned().unwrap_or_else(|| default.to_string())
}

/// java: public static String REPOSITORY_PARSER_VERSION = VERSION_PROPERTIES.getProperty("neurepository.parser.version", "unknown");
pub static REPOSITORY_PARSER_VERSION: LazyLock<String> = LazyLock::new(|| get_property("neurepository.parser.version", "unknown"));
/// java: public static int REPOSITORY_SCHEMA_VERSION_MINOR = Integer.parseInt(VERSION_PROPERTIES.getProperty("neurepository.schema.minor", "-1"));
pub static REPOSITORY_SCHEMA_VERSION_MINOR: LazyLock<i32> =
    LazyLock::new(|| get_property("neurepository.schema.minor", "-1").parse().expect("NumberFormatException"));
/// java: public static int REPOSITORY_SCHEMA_VERSION_MAJOR = Integer.parseInt(VERSION_PROPERTIES.getProperty("neurepository.schema.major", "-1"));
pub static REPOSITORY_SCHEMA_VERSION_MAJOR: LazyLock<i32> =
    LazyLock::new(|| get_property("neurepository.schema.major", "-1").parse().expect("NumberFormatException"));
