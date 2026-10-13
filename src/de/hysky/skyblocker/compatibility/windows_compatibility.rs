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
        // java: try (Arena arena = Arena.ofConfined()) {..} catch (Throwable t) - a failure here is a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(|| {
            //   java: SymbolLookup kernel32Lookup = SymbolLookup.libraryLookup("Kernel32", arena);
            //   java: MethodHandle getLocaleInfoEx = Linker.nativeLinker().downcallHandle(kernel32Lookup.findOrThrow("GetLocaleInfoEx"),
            //   java: 		FunctionDescriptor.of(JAVA_INT, ADDRESS, JAVA_INT, ADDRESS, JAVA_INT));
            // The same downcall, linked by Rust's own native linker (`kernel32`).
            // Get the necessary segment size for storing the time format
            //   java: int segmentSize = (int) getLocaleInfoEx.invokeExact(MemorySegment.NULL, LOCALE_STIMEFORMAT, MemorySegment.NULL, 0);
            let segment_size: i32 = kernel32::get_locale_info_ex(std::ptr::null(), LOCALE_STIMEFORMAT, std::ptr::null_mut(), 0);
            //   java: MemorySegment segment = arena.allocate(segmentSize * ValueLayout.JAVA_CHAR.byteSize());
            let mut segment: Vec<u16> = vec![0u16; segment_size.max(0) as usize];
            // Store the time format into the segment
            //   java: getLocaleInfoEx.invoke(MemorySegment.NULL, LOCALE_STIMEFORMAT, segment, segmentSize);
            kernel32::get_locale_info_ex(std::ptr::null(), LOCALE_STIMEFORMAT, segment.as_mut_ptr(), segment_size);
            //   java: String timeFormat = segment.getString(0);
            // `MemorySegment.getString(0)` decodes UTF-8 up to the first zero BYTE - over the UTF-16 the call wrote, that is
            // the first character only (its high byte is zero), as Java reads it.
            let bytes: Vec<u8> = segment.iter().flat_map(|unit| unit.to_le_bytes()).collect();
            let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
            let time_format: String = String::from_utf8_lossy(&bytes[..end]).into_owned();
            // The time format contains lower case h's when its 12 hour, and upper case ones when its 24 hour
            return time_format.contains("h");
        }) {
            Ok(__v) => return __v,
            Err(t) => {
                logger().error(&format!("[Skyblocker Windows Compatibility] Failed to check for 12/24 hours clock. {}", compat::panic_message(&t)));
            }
        }
        return false;
    }
}

/// `GetLocaleInfoEx` (winnls.h, Kernel32): `int GetLocaleInfoEx(LPCWSTR lpLocaleName, LCTYPE LCType, LPWSTR lpLCData,
/// int cchData)` - the downcall `WindowsCompatibility.is12HourClock` makes.
mod kernel32 {
    #[cfg(windows)]
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLocaleInfoEx(lp_locale_name: *const u16, lc_type: u32, lp_lc_data: *mut u16, cch_data: i32) -> i32;
    }

    /// One call of `GetLocaleInfoEx`. `lp_lc_data` holds `cch_data` UTF-16 units (or is null with `cch_data` 0).
    #[cfg(windows)]
    pub fn get_locale_info_ex(lp_locale_name: *const u16, lc_type: i32, lp_lc_data: *mut u16, cch_data: i32) -> i32 {
        // SAFETY: a Win32 call over a buffer of `cch_data` units the caller allocated (or none, for the size query).
        unsafe { GetLocaleInfoEx(lp_locale_name, lc_type as u32, lp_lc_data, cch_data) }
    }

    /// Not Windows: `SymbolLookup.libraryLookup("Kernel32", ..)` throws `IllegalArgumentException` (no such library),
    /// which the caller's `catch (Throwable)` turns into the 24-hour answer.
    #[cfg(not(windows))]
    pub fn get_locale_info_ex(_lp_locale_name: *const u16, _lc_type: i32, _lp_lc_data: *mut u16, _cch_data: i32) -> i32 {
        panic!("IllegalArgumentException: Cannot open library: Kernel32");
    }
}
