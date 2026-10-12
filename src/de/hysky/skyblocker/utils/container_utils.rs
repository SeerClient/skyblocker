//! Ported from de.hysky.skyblocker.utils.ContainerUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class ContainerUtils
#[derive(Clone)]
pub struct ContainerUtils {}

impl ContainerUtils {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static int getContainerClickButton(@MouseButtonInfo.MouseButton int mouseButton)
    pub fn get_container_click_button(mut mouse_button: i32) -> i32 {
        return mouse_button;
    }

}
