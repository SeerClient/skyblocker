//! java: public class Utils (net.azureaaron.networth.utils)

/// java: public class Utils
pub struct Utils;

impl Utils {
    /// java: public static <T> T make(T object, Consumer<T> initializer) - useful for instantiating constant fields
    /// containing Maps for example.
    pub fn make<T>(mut object: T, initializer: impl FnOnce(&mut T)) -> T {
        initializer(&mut object);

        object
    }
}
