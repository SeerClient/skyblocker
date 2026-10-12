//! Ported from de.hysky.skyblocker.config.CommonTags by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public static final Component ADDED_IN_5_9_0 = Component.nullToEmpty("v5.9.0");
pub static ADDED_IN_5_9_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v5.9.0")));
/// java: public static final Component ADDED_IN_5_10_0 = Component.nullToEmpty("v5.10.0");
pub static ADDED_IN_5_10_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v5.10.0")));
/// java: public static final Component ADDED_IN_5_11_0 = Component.nullToEmpty("v5.11.0");
pub static ADDED_IN_5_11_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v5.11.0")));
/// java: public static final Component ADDED_IN_6_0_0 = Component.nullToEmpty("v6.0.0");
pub static ADDED_IN_6_0_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.0.0")));
/// java: public static final Component ADDED_IN_6_2_0 = Component.nullToEmpty("v6.2.0");
pub static ADDED_IN_6_2_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.2.0")));
/// java: public static final Component ADDED_IN_6_3_0 = Component.nullToEmpty("v6.3.0");
pub static ADDED_IN_6_3_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.3.0")));
/// java: public static final Component ADDED_IN_6_4_0 = Component.nullToEmpty("v6.4.0");
pub static ADDED_IN_6_4_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.4.0")));
/// java: public static final Component ADDED_IN_6_5_0 = Component.nullToEmpty("v6.5.0");
pub static ADDED_IN_6_5_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.5.0")));
/// java: public static final Component ADDED_IN_6_5_1 = Component.nullToEmpty("v6.5.1");
pub static ADDED_IN_6_5_1: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::null_to_empty(Some("v6.5.1")));
/// java: public static final Component ADDED_IN_6_6_0 = Component.literal("v6.6.0");
pub static ADDED_IN_6_6_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.6.0"));
/// java: public static final Component ADDED_IN_6_7_0 = Component.literal("v6.7.0");
pub static ADDED_IN_6_7_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.7.0"));
/// java: public static final Component ADDED_IN_6_8_0 = Component.literal("v6.8.0");
pub static ADDED_IN_6_8_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.8.0"));
/// java: public static final Component ADDED_IN_6_8_1 = Component.literal("v6.8.1");
pub static ADDED_IN_6_8_1: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.8.1"));
/// java: public static final Component ADDED_IN_6_8_2 = Component.literal("v6.8.2");
pub static ADDED_IN_6_8_2: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.8.2"));
/// java: public static final Component ADDED_IN_6_9_0 = Component.literal("v6.9.0");
pub static ADDED_IN_6_9_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.9.0"));
/// java: public static final Component ADDED_IN_6_9_1 = Component.literal("v6.9.1");
pub static ADDED_IN_6_9_1: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.9.1"));
/// java: public static final Component ADDED_IN_6_10_0 = Component.literal("v6.10.0");
pub static ADDED_IN_6_10_0: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("v6.10.0"));
/// java: public static final Component LATEST_VERSION_TAG = ADDED_IN_6_10_0;
pub static LATEST_VERSION_TAG: compat::MainThread<MutableComponent> = compat::MainThread::new(|| ADDED_IN_6_10_0.get_mut().clone());
/// java: public static final Component[] KEY_MAPPING = { Component.translatable("skyblocker.config.tag.keyBind"), Component.translatable("skyblocker.config.tag.keyMapping") };
pub static KEY_MAPPING: compat::MainThread<Vec<MutableComponent>> = compat::MainThread::new(|| vec![component::translatable("skyblocker.config.tag.keyBind"), component::translatable("skyblocker.config.tag.keyMapping")]);

/// java: public final class CommonTags
#[derive(Clone)]
pub struct CommonTags {}

impl CommonTags {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

}
