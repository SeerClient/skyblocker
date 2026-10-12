//! Ported from de.hysky.skyblocker.utils.EntityUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public static final int PLACEHOLDER_ID = -1;
pub const PLACEHOLDER_ID: i32 = -1;

/// java: public final class EntityUtils
#[derive(Clone)]
pub struct EntityUtils {}

impl EntityUtils {
    /// java: private EntityUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
