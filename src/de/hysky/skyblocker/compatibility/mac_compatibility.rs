//! Ported from de.hysky.skyblocker.compatibility.MacCompatibility by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class MacCompatibility
#[derive(Clone)]
pub struct MacCompatibility {}

impl MacCompatibility {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean is12HourClock()
    pub fn is12_hour_clock() -> bool {
        // The j formatting template returns the preferred formatting for the time
        // If the format contains a (am/pm pattern) then the preference is to use the 12 hour clock, otherwise its the 24 hour clock
        //   java: Object locale = Client.getInstance().send("NSLocale", "currentLocale");
        //   java: String timeFormat = (String) Client.getInstance().send("NSDateFormatter", "dateFormatFromTemplate:options:locale:", "j", 0, locale);
        // The same two Objective-C messages (`Client.send` is `objc_msgSend` with the Java values bridged), sent through the
        // Objective-C runtime directly.
        let time_format: String = objc::time_format_from_template("j");
        return time_format.contains("a");
    }

}

/// The Objective-C runtime calls `ca.weblite.objc.Client.send` makes for `MacCompatibility`.
mod objc {
    #[cfg(target_os = "macos")]
    mod runtime {
        use std::ffi::{c_char, c_void, CStr, CString};

        #[link(name = "objc")]
        unsafe extern "C" {
            fn objc_getClass(name: *const c_char) -> *mut c_void;
            fn sel_registerName(name: *const c_char) -> *mut c_void;
            fn objc_msgSend();
        }

        #[link(name = "Foundation", kind = "framework")]
        unsafe extern "C" {}

        type Id = *mut c_void;
        type Sel = *mut c_void;

        fn class(name: &str) -> Id {
            let name = CString::new(name).expect("class name");
            // SAFETY: a NUL-terminated name; an unknown class answers null.
            unsafe { objc_getClass(name.as_ptr()) }
        }

        fn selector(name: &str) -> Sel {
            let name = CString::new(name).expect("selector name");
            // SAFETY: a NUL-terminated name.
            unsafe { sel_registerName(name.as_ptr()) }
        }

        /// `[NSString stringWithUTF8String:s]` - `Client.send`'s conversion of a Java `String` argument.
        fn ns_string(s: &str) -> Id {
            let s = CString::new(s).expect("string");
            // SAFETY: `objc_msgSend` called through the message's own signature (`id (id, SEL, const char *)`).
            unsafe {
                let send: unsafe extern "C" fn(Id, Sel, *const c_char) -> Id = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
                send(class("NSString"), selector("stringWithUTF8String:"), s.as_ptr())
            }
        }

        /// `(String) send(..)` of an `NSString` answer - `[s UTF8String]`.
        fn java_string(s: Id) -> String {
            if s.is_null() {
                panic!("NullPointerException: the message answered nil");
            }
            // SAFETY: `UTF8String` is `const char * (id, SEL)`; the C string lives as long as `s`.
            unsafe {
                let send: unsafe extern "C" fn(Id, Sel) -> *const c_char = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
                CStr::from_ptr(send(s, selector("UTF8String"))).to_string_lossy().into_owned()
            }
        }

        pub fn time_format_from_template(template: &str) -> String {
            // SAFETY: each `objc_msgSend` is called through its message's signature.
            unsafe {
                //   java: Object locale = Client.getInstance().send("NSLocale", "currentLocale");
                let send0: unsafe extern "C" fn(Id, Sel) -> Id = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
                let locale: Id = send0(class("NSLocale"), selector("currentLocale"));
                //   java: send("NSDateFormatter", "dateFormatFromTemplate:options:locale:", "j", 0, locale)
                let send3: unsafe extern "C" fn(Id, Sel, Id, usize, Id) -> Id = std::mem::transmute(objc_msgSend as unsafe extern "C" fn());
                java_string(send3(class("NSDateFormatter"), selector("dateFormatFromTemplate:options:locale:"), ns_string(template), 0, locale))
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub use runtime::time_format_from_template;

    /// Not macOS: `Client.getInstance()` loads `libobjc`, which throws there (`UnsatisfiedLinkError`); the one caller
    /// (`Formatters.is12HourClock`) asks only on macOS.
    #[cfg(not(target_os = "macos"))]
    pub fn time_format_from_template(_template: &str) -> String {
        panic!("UnsatisfiedLinkError: libobjc is only on macOS");
    }
}
