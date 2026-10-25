//! Ported from meteordevelopment.discordipc.IPCUser (discord-ipc 1.1).

/// java: public class IPCUser - filled by `GSON.fromJson(.., IPCUser.class)`
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IPCUser {
    pub id: Option<String>,
    pub username: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub bot: bool,
    pub flags: Option<String>,
    pub premium_type: i32,
}

impl IPCUser {
    /// `GSON.fromJson(json, IPCUser.class)` - Gson's reflective adapter: each field from the member of its name; a
    /// missing or `null` member leaves the field's default; a `String` field reads a JSON number or boolean as its text
    /// (Gson's `STRING` adapter); `int`/`boolean` fields read their JSON values (a mismatch is the
    /// `JsonSyntaxException`).
    pub fn from_json(json: &serde_json::Map<String, serde_json::Value>) -> IPCUser {
        fn string(json: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<String> {
            match json.get(key) {
                None | Some(serde_json::Value::Null) => None,
                Some(serde_json::Value::String(s)) => Some(s.clone()),
                Some(serde_json::Value::Number(n)) => Some(n.to_string()),
                Some(serde_json::Value::Bool(b)) => Some(b.to_string()),
                Some(other) => panic!("JsonSyntaxException: Expected a string but was {other}"),
            }
        }
        IPCUser {
            id: string(json, "id"),
            username: string(json, "username"),
            discriminator: string(json, "discriminator"),
            avatar: string(json, "avatar"),
            bot: match json.get("bot") {
                None | Some(serde_json::Value::Null) => false,
                Some(serde_json::Value::Bool(b)) => *b,
                // Gson's BOOLEAN adapter also reads a string through `Boolean.parseBoolean`
                Some(serde_json::Value::String(s)) => s.eq_ignore_ascii_case("true"),
                Some(other) => panic!("JsonSyntaxException: Expected a boolean but was {other}"),
            },
            flags: string(json, "flags"),
            premium_type: match json.get("premium_type") {
                None | Some(serde_json::Value::Null) => 0,
                Some(serde_json::Value::Number(n)) => n.as_i64().map(|v| v as i32).unwrap_or_else(|| panic!("JsonSyntaxException: {n}")),
                Some(serde_json::Value::String(s)) => s.parse::<i32>().unwrap_or_else(|e| panic!("JsonSyntaxException: {e}")),
                Some(other) => panic!("JsonSyntaxException: Expected an int but was {other}"),
            },
        }
    }
}
