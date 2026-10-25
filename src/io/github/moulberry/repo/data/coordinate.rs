//! Ported from io.github.moulberry.repo.data.Coordinate (neurepoparser 1.12.0).

use serde_json::Value;

/// java: @Data @AllArgsConstructor public class Coordinate
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Coordinate {
    /// java: int x;
    pub(crate) x: i32,
    /// java: int y;
    pub(crate) y: i32,
    /// java: int z;
    pub(crate) z: i32,
}

/// java: public static class CoordinateSerializer implements JsonDeserializer<Coordinate>
pub struct CoordinateSerializer;

impl CoordinateSerializer {
    /// java: public Coordinate deserialize(JsonElement json, Type typeOfT, JsonDeserializationContext context) throws JsonParseException
    ///
    /// A `JsonParseException` / `NumberFormatException` is a panic (the repository's reload catches it as Java's).
    pub fn deserialize(json: &Value) -> Coordinate {
        // java: String asString = json.getAsString();
        let as_string: String = crate::io::github::moulberry::repo::neu_repository::gson::as_string(json)
            .unwrap_or_else(|| panic!("java.lang.UnsupportedOperationException: getAsString on {}", json));
        // java: String[] split = asString.split(","); - Java drops trailing empty strings
        let mut split: Vec<&str> = as_string.split(',').collect();
        while split.len() > 1 && split.last() == Some(&"") {
            split.pop();
        }
        if split.len() != 3 {
            panic!("JsonParseException: Coordinate must be a string consisting of 3 numbers seperated by , but is `{}`", as_string);
        }
        let parse = |s: &str| -> i32 { s.parse::<i32>().unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", s)) };
        // java: return new Coordinate(Integer.parseInt(split[0]), Integer.parseInt(split[1]), Integer.parseInt(split[2]));
        Coordinate::new(parse(split[0]), parse(split[1]), parse(split[2]))
    }
}

impl Coordinate {
    /// java: public Coordinate(final int x, final int y, final int z)
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate { x, y, z }
    }

    /// java: public int getX()
    pub fn get_x(&self) -> i32 {
        self.x
    }

    /// java: public int getY()
    pub fn get_y(&self) -> i32 {
        self.y
    }

    /// java: public int getZ()
    pub fn get_z(&self) -> i32 {
        self.z
    }

    /// java: public void setX(final int x)
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    /// java: public void setY(final int y)
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    /// java: public void setZ(final int z)
    pub fn set_z(&mut self, z: i32) {
        self.z = z;
    }

    /// java: public int hashCode() (lombok: PRIME 59)
    pub fn hash_code(&self) -> i32 {
        let mut result: i32 = 1;
        result = result.wrapping_mul(59).wrapping_add(self.x);
        result = result.wrapping_mul(59).wrapping_add(self.y);
        result = result.wrapping_mul(59).wrapping_add(self.z);
        result
    }
}

impl std::fmt::Display for Coordinate {
    /// java: public String toString()
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}
