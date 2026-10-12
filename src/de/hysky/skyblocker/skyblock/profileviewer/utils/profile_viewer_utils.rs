//! Ported from de.hysky.skyblocker.skyblock.profileviewer.utils.ProfileViewerUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::flexible_item_stack::FlexibleItemStack;
use crate::de::hysky::skyblocker::utils::item_utils::ItemUtils;

/// java: public class ProfileViewerUtils
#[derive(Clone)]
pub struct ProfileViewerUtils {}

impl ProfileViewerUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static FlexibleItemStack createSkull(String textureB64)
    pub fn create_skull(texture_b64: &str) -> compat::Shared<FlexibleItemStack> {
        return ItemUtils::create_skull(texture_b64);
    }

    /// java: public static String numLetterFormat(double amount)
    pub fn num_letter_format(mut amount: f64) -> String {
        if amount >= (1000000000 as f64) {
            return compat::string_format("%.4gB", &[compat::FormatArg::from(amount / (1000000000 as f64))]);
        } else if amount >= (1000000 as f64) {
            return compat::string_format("%.4gM", &[compat::FormatArg::from(amount / (1000000 as f64))]);
        } else if amount >= (1000 as f64) {
            return compat::string_format("%.4gK", &[compat::FormatArg::from(amount / (1000 as f64))]);
        } else {
            return ((amount) as i32).to_string();
        }
    }

}
