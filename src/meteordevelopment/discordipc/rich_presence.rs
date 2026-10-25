//! Ported from meteordevelopment.discordipc.RichPresence (discord-ipc 1.1), with its inner classes `Assets` and
//! `Timestamps`.

/// java: public class RichPresence
#[derive(Clone, Debug, Default)]
pub struct RichPresence {
    /// java: private String details;
    details: Option<String>,
    /// java: private String state;
    state: Option<String>,
    /// java: private Assets assets;
    assets: Option<Assets>,
    /// java: private Timestamps timestamps;
    timestamps: Option<Timestamps>,
}

/// java: public static class Assets
#[derive(Clone, Debug, Default)]
pub struct Assets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

/// java: public static class Timestamps
#[derive(Clone, Debug, Default)]
pub struct Timestamps {
    pub start: Option<i64>,
    pub end: Option<i64>,
}

impl RichPresence {
    /// java: the implicit no-argument constructor
    pub fn new() -> RichPresence {
        RichPresence::default()
    }

    /// java: public void setDetails(String details)
    pub fn set_details(&mut self, details: Option<String>) {
        self.details = details;
    }

    /// java: public void setState(String state)
    pub fn set_state(&mut self, state: Option<String>) {
        self.state = state;
    }

    /// java: public void setLargeImage(String key, String text)
    pub fn set_large_image(&mut self, key: Option<String>, text: Option<String>) {
        let assets = self.assets.get_or_insert_with(Assets::default);
        assets.large_image = key;
        assets.large_text = text;
    }

    /// java: public void setSmallImage(String key, String text)
    pub fn set_small_image(&mut self, key: Option<String>, text: Option<String>) {
        let assets = self.assets.get_or_insert_with(Assets::default);
        assets.small_image = key;
        assets.small_text = text;
    }

    /// java: public void setStart(long time)
    pub fn set_start(&mut self, time: i64) {
        self.timestamps.get_or_insert_with(Timestamps::default).start = Some(time);
    }

    /// java: public void setEnd(long time)
    pub fn set_end(&mut self, time: i64) {
        self.timestamps.get_or_insert_with(Timestamps::default).end = Some(time);
    }

    /// java: public JsonObject toJson()
    pub fn to_json(&self) -> serde_json::Map<String, serde_json::Value> {
        use serde_json::Value;
        // Main
        let mut o = serde_json::Map::new();

        if let Some(details) = &self.details {
            o.insert("details".into(), Value::String(details.clone()));
        }
        if let Some(state) = &self.state {
            o.insert("state".into(), Value::String(state.clone()));
        }

        // Assets
        if let Some(assets) = &self.assets {
            let mut a = serde_json::Map::new();

            if let Some(v) = &assets.large_image {
                a.insert("large_image".into(), Value::String(v.clone()));
            }
            if let Some(v) = &assets.large_text {
                a.insert("large_text".into(), Value::String(v.clone()));
            }
            if let Some(v) = &assets.small_image {
                a.insert("small_image".into(), Value::String(v.clone()));
            }
            if let Some(v) = &assets.small_text {
                a.insert("small_text".into(), Value::String(v.clone()));
            }

            o.insert("assets".into(), Value::Object(a));
        }

        // Timestamps
        if let Some(timestamps) = &self.timestamps {
            let mut t = serde_json::Map::new();

            if let Some(start) = timestamps.start {
                t.insert("start".into(), Value::from(start));
            }
            if let Some(end) = timestamps.end {
                t.insert("end".into(), Value::from(end));
            }

            o.insert("timestamps".into(), Value::Object(t));
        }

        o
    }
}
