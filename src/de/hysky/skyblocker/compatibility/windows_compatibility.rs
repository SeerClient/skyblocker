//! Ported from de.hysky.skyblocker.compatibility.WindowsCompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final int LOCALE_STIMEFORMAT = 0x00001003;
pub const LOCALE_STIMEFORMAT: i32 = 0x00001003;

/// java: public class WindowsCompatibility
#[derive(Clone)]
pub struct WindowsCompatibility {}

impl WindowsCompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean is12HourClock()
    pub fn is12_hour_clock() -> bool {
        {
            // TODO(fabric2seer): static call Arena.ofConfined(..)
            //   java: Arena arena = Arena.ofConfined(..);
            let mut arena: crate::unresolved::Arena = todo!("fabric2seer: the initialiser of `arena` (TODO above)");
            // java: try {..} catch (Throwable t) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call SymbolLookup.libraryLookup(..)
                //   java: SymbolLookup kernel32Lookup = SymbolLookup.libraryLookup("Kernel32", arena);
                let mut kernel32_lookup: crate::unresolved::SymbolLookup = todo!("fabric2seer: the initialiser of `kernel32Lookup` (TODO above)");
                // TODO(fabric2seer): static call Linker.nativeLinker(..)
                //   java: MethodHandle getLocaleInfoEx = Linker.nativeLinker().downcallHandle(
                //   java: 					kernel32Lookup.findOrThrow("GetLocaleInfoEx"),
                //   java: 					FunctionDescriptor.of(ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT, ValueLayout.ADDRESS, ValueLayout.JAVA_INT)
                //   java: 					);
                let mut get_locale_info_ex: crate::unresolved::MethodHandle = todo!("fabric2seer: the initialiser of `getLocaleInfoEx` (TODO above)");
                // TODO(fabric2seer): `.invokeExact(..)` on a MethodHandle
                //   java: int segmentSize = (int) getLocaleInfoEx.invokeExact(MemorySegment.NULL, LOCALE_STIMEFORMAT, MemorySegment.NULL, 0);
                let mut segment_size: i32 = todo!("fabric2seer: the initialiser of `segmentSize` (TODO above)");
                // TODO(fabric2seer): `.allocate(..)` on a Arena
                //   java: MemorySegment segment = arena.allocate(segmentSize * ValueLayout.JAVA_CHAR.byteSize());
                let mut segment: crate::unresolved::MemorySegment = todo!("fabric2seer: the initialiser of `segment` (TODO above)");
                // TODO(fabric2seer): `.invoke(..)` on a MethodHandle
                //   java: getLocaleInfoEx.invoke(MemorySegment.NULL, LOCALE_STIMEFORMAT, segment, segmentSize);
                // TODO(fabric2seer): `.getString(..)` on a MemorySegment
                //   java: String timeFormat = segment.getString(0);
                let mut time_format: String = todo!("fabric2seer: the initialiser of `timeFormat` (TODO above)");
                return time_format.contains("h");
            })) {
                Ok(__v) => return __v,
                Err(t) => {
                    logger().error(&format!("[Skyblocker Windows Compatibility] Failed to check for 12/24 hours clock."));
                }
            }
        }
        return false;
    }

}
