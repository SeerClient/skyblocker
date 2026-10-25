//! Ported from net.azureaaron.legacyitemdfu.fixers.fixes.UuidFixes (legacy-item-dfu 1.0.4+26.2).

use mcrust::com::mojang::serialization::dynamic::Dynamic;
use mcrust::com::mojang::serialization::dynamic_like::DynamicLike as _;
use mcrust::java::util::uuid::Uuid;

/// java: public class UuidFixes
pub struct UuidFixes;

impl UuidFixes {
    /// java: public static Optional<Dynamic<?>> updateStringUuid(Dynamic<?> dynamic, String oldKey, String newKey)
    pub fn update_string_uuid(dynamic: &Dynamic, old_key: &str, new_key: &str) -> Option<Dynamic> {
        UuidFixes::create_array_from_string_uuid(dynamic, old_key).map(|dynamic2| dynamic.remove(old_key).set(new_key, &dynamic2))
    }

    /// java: private static Optional<Dynamic<?>> createArrayFromStringUuid(Dynamic<?> dynamic, String key)
    fn create_array_from_string_uuid(dynamic: &Dynamic, key: &str) -> Option<Dynamic> {
        dynamic.get(key).result().and_then(|dynamic2| {
            // java: String string = dynamic2.asString(null);
            let string = dynamic2.as_string().result();
            if let Some(string) = string {
                // java: try { UUID uUID = UUID.fromString(string); .. } catch (IllegalArgumentException var4) {}
                if let Some(u_uid) = Uuid::from_string(&string) {
                    return UuidFixes::create_array(dynamic, u_uid.get_most_significant_bits(), u_uid.get_least_significant_bits());
                }
            }
            None
        })
    }

    /// java: public static Optional<Dynamic<?>> updateRegularMostLeast(Dynamic<?> dynamic, String oldKey, String newKey)
    pub fn update_regular_most_least(dynamic: &Dynamic, old_key: &str, new_key: &str) -> Option<Dynamic> {
        let string = format!("{old_key}Most");
        let string2 = format!("{old_key}Least");
        UuidFixes::create_array_from_most_least_tags(dynamic, &string, &string2).map(|dynamic2| dynamic.remove(&string).remove(&string2).set(new_key, &dynamic2))
    }

    /// java: private static Optional<Dynamic<?>> createArrayFromMostLeastTags(Dynamic<?> dynamic, String mostBitsKey, String leastBitsKey)
    fn create_array_from_most_least_tags(dynamic: &Dynamic, most_bits_key: &str, least_bits_key: &str) -> Option<Dynamic> {
        let l = dynamic.get(most_bits_key).as_long(0);
        let m = dynamic.get(least_bits_key).as_long(0);
        if l != 0 && m != 0 { UuidFixes::create_array(dynamic, l, m) } else { None }
    }

    /// java: private static Optional<Dynamic<?>> createArray(Dynamic<?> dynamic, long mostBits, long leastBits)
    fn create_array(dynamic: &Dynamic, most_bits: i64, least_bits: i64) -> Option<Dynamic> {
        Some(dynamic.create_int_list(&[(most_bits >> 32) as i32, most_bits as i32, (least_bits >> 32) as i32, least_bits as i32]))
    }
}
