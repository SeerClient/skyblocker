//! Ported from de.hysky.skyblocker.skyblock.item.tooltip.info.SimpleTooltipInfo by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::general_config::ItemTooltip;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::info::tooltip_info_type::TooltipInfoType;
use crate::de::hysky::skyblocker::skyblock::item::tooltip::item_tooltip;

/// java: public class SimpleTooltipInfo implements TooltipInfoType
#[derive(Clone)]
pub struct SimpleTooltipInfoBase {
    /// java: private final Predicate<GeneralConfig.ItemTooltip> tooltipEnabled;
    pub tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>,
}

impl SimpleTooltipInfoBase {
    /// java: protected SimpleTooltipInfo(Predicate<GeneralConfig.ItemTooltip> tooltipEnabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SimpleTooltipInfo>, __R)) -> (compat::Shared<dyn SimpleTooltipInfo>, __R) {
        let __init_tooltip_enabled = tooltip_enabled.clone();
        let this = Self {
            tooltip_enabled: __init_tooltip_enabled,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

    /// java: protected SimpleTooltipInfo(Predicate<GeneralConfig.ItemTooltip> tooltipEnabled)
    pub fn new(tooltip_enabled: Arc<dyn Fn(compat::Shared<ItemTooltip>) -> bool>) -> compat::Shared<Self> {
        Self::new_in(tooltip_enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SimpleTooltipInfo>), __h)
        }).1
    }

}

/// java: the virtual methods of SimpleTooltipInfo (implemented by SimpleTooltipInfoBase and every subclass)
pub trait SimpleTooltipInfo: TooltipInfoType {
    fn simple_tooltip_info_base(&self) -> &SimpleTooltipInfoBase;
    fn simple_tooltip_info_base_mut(&mut self) -> &mut SimpleTooltipInfoBase;

}

impl dyn SimpleTooltipInfo {
    /// java: @Override public boolean isTooltipEnabled()
    pub fn is_tooltip_enabled_body(__this: &compat::Shared<dyn SimpleTooltipInfo>) -> bool {
        return ((compat::read(__this, |__t| __t.simple_tooltip_info_base().tooltip_enabled.clone())))((item_tooltip::CONFIG.get_mut())());
    }

    /// SimpleTooltipInfo.isTooltipEnabled as the dispatch of TooltipInfoType.isTooltipEnabled for the concrete class `__C`
    pub fn is_tooltip_enabled_from<__C: SimpleTooltipInfo + 'static>(__this: &compat::Shared<dyn TooltipInfoType>) -> bool {
        <dyn SimpleTooltipInfo>::is_tooltip_enabled_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SimpleTooltipInfo>))
    }

}

impl SimpleTooltipInfo for SimpleTooltipInfoBase {
    fn simple_tooltip_info_base(&self) -> &SimpleTooltipInfoBase {
        self
    }
    fn simple_tooltip_info_base_mut(&mut self) -> &mut SimpleTooltipInfoBase {
        self
    }

}

impl TooltipInfoType for SimpleTooltipInfoBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean isTooltipEnabled() (dispatch: the body for this class)
    fn is_tooltip_enabled(&self) -> fn(&compat::Shared<dyn TooltipInfoType>) -> bool {
        <dyn SimpleTooltipInfo>::is_tooltip_enabled_from::<Self>
    }

}
