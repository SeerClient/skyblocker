//! Ported from de.hysky.skyblocker.skyblock.auction.widgets.AuctionTypeWidget by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::auction::slot_click_handler::SlotClickHandler;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::slider_widget::OptionInfo;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::slider_widget::SliderWidget;
use crate::de::hysky::skyblocker::skyblock::auction::widgets::slider_widget::SliderWidgetBase;
use mcrust::mcrust::modding::widgets::ModWidget;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: private static final String prefix = "auctions_gui/auction_type_widget/";
pub const OPTION_PREFIX: &str = "auctions_gui/auction_type_widget/";
/// java: private static final Identifier BACK_TEXTURE = SkyblockerMod.id(prefix + "back");
pub static OPTION_BACK_TEXTURE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id(&format!("{}back", OPTION_PREFIX)));
// TODO(fabric2seer): call to `values(..)` (not a method of this class)
//   java: private static final AuctionTypeWidget.Option[] values = values();
/// java: private static final AuctionTypeWidget.Option[] values = values();
pub static OPTION_VALUES: LazyLock<Vec<Option>> = LazyLock::new(|| todo!("fabric2seer: the initialiser of `values` - see the TODO above"));

/// java: public class AuctionTypeWidget extends SliderWidget<AuctionTypeWidget.Option>
#[derive(Clone)]
pub struct AuctionTypeWidget {
    /// java: extends SliderWidget
    pub base: SliderWidgetBase<Option>,
}

impl std::ops::Deref for AuctionTypeWidget {
    type Target = SliderWidgetBase<Option>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for AuctionTypeWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl AuctionTypeWidget {
    /// java: public AuctionTypeWidget(int x, int y, SlotClickHandler slotClick)
    pub fn new(mut x: i32, mut y: i32, mut slot_click: compat::Shared<dyn SlotClickHandler>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: SliderWidgetBase::<Option>::new(x, y, 17, 17, component::literal("Auction Type Widget"), slot_click.clone(), Option::All),
        });
        __this
    }

}

impl SliderWidget<crate::de::hysky::skyblocker::skyblock::auction::widgets::auction_type_widget::Option> for AuctionTypeWidget {
    fn slider_widget_base(&self) -> &SliderWidgetBase<crate::de::hysky::skyblocker::skyblock::auction::widgets::auction_type_widget::Option> {
        &self.base
    }
    fn slider_widget_base_mut(&mut self) -> &mut SliderWidgetBase<crate::de::hysky::skyblocker::skyblock::auction::widgets::auction_type_widget::Option> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// AuctionTypeWidget's widget half (`super`'s fields: x, y, size, message, ...), lent by the game for each call
impl compat::WidgetHalf for AuctionTypeWidget {
    fn widget_half(&mut self) -> &mut AbstractWidget {
        &mut self.base.base
    }
}

/// AuctionTypeWidget as the game's widget: `compat::WidgetAdapter::widget(handle)` where Java hands it on
impl ModWidget for compat::WidgetAdapter<AuctionTypeWidget> {
    /// java: extractWidgetRenderState - not overridden here (the base class's body has no entry point in the port)
    fn extract_widget_render_state(&mut self, widget: &AbstractWidget, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        let _ = widget;
    }

    /// java: updateWidgetNarration - not overridden here (the base class's body has no entry point in the port)
    fn update_widget_narration(&mut self, widget: &AbstractWidget, output: &mut dyn NarrationElementOutput) {
        let _ = widget;
    }

}

/// java: public enum Option implements SliderWidget.OptionInfo
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Option {
    All,
    Bin,
    Auc,
}

impl Option {
    pub const VALUES: [Option; 3] = [Option::All, Option::Bin, Option::Auc];

    /// java: values()
    pub fn values() -> Vec<Option> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Option::All => "ALL",
            Option::Bin => "BIN",
            Option::Auc => "AUC",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Option {
        match name {
            "ALL" => Option::All,
            "BIN" => Option::Bin,
            "AUC" => Option::Auc,
            _ => panic!("IllegalArgumentException: No enum constant Option.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static OptionFields {
        static FIELDS: LazyLock<Vec<OptionFields>> = LazyLock::new(|| vec![
            OptionFields::new("all".to_string()),
            OptionFields::new("bin".to_string()),
            OptionFields::new("auctions".to_string()),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public static AuctionTypeWidget.Option get(int ordinal)
    pub fn get(mut ordinal: i32) -> Option {
        return OPTION_VALUES[(ordinal.clamp(0, (OPTION_VALUES.len() as i32).wrapping_sub(1))) as usize];
    }

    /// java: @Override public boolean isVertical()
    pub fn is_vertical(&self) -> bool {
        return true;
    }

    /// java: @Override public int getOffset()
    pub fn get_offset(&self) -> i32 {
        // TODO(fabric2seer): call to `ordinal(..)` (not a method of this class)
        //   java: return 4 * ordinal();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public int[] getOptionSize()
    pub fn get_option_size(&self) -> Vec<i32> {
        return vec![17, 9];
    }

    /// java: @Override public Identifier getOptionTexture()
    pub fn get_option_texture(&self) -> Identifier {
        return self.fields().texture.clone();
    }

    /// java: @Override public Identifier getBackTexture()
    pub fn get_back_texture(&self) -> Identifier {
        return OPTION_BACK_TEXTURE.get_mut().clone();
    }

    /// java: @Override public Identifier getHoverTexture()
    pub fn get_hover_texture(&self) -> Identifier {
        return self.fields().hover_texture.clone();
    }

}

impl std::fmt::Display for Option {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Option constant
pub struct OptionFields {
    pub texture: Identifier,
    pub hover_texture: Identifier,
}

impl OptionFields {
    fn new(mut texture_name: String) -> Self {
        let mut this = Self {
            texture: Default::default(),
            hover_texture: Default::default(),
        };
        this.texture = crate::entry::skyblocker_mod::id(&format!("{}{}", OPTION_PREFIX, texture_name));
        this.hover_texture = crate::entry::skyblocker_mod::id(&format!("{}{}_hover", OPTION_PREFIX, texture_name));
        this
    }

}

impl OptionInfo for Option {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn is_vertical(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> bool {
        Option::is_vertical_dyn
    }

    fn get_offset(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> i32 {
        Option::get_offset_dyn
    }

    fn get_option_size(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Vec<i32> {
        Option::get_option_size_dyn
    }

    fn get_option_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier {
        Option::get_option_texture_dyn
    }

    fn get_back_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier {
        Option::get_back_texture_dyn
    }

    fn get_hover_texture(&self) -> fn(&compat::Shared<dyn OptionInfo>) -> Identifier {
        Option::get_hover_texture_dyn
    }

}

impl Option {
    /// Option.isVertical as the dispatch of OptionInfo.isVertical
    pub fn is_vertical_dyn(__this: &compat::Shared<dyn OptionInfo>) -> bool {
        compat::read_as::<Option, _, _>(__this, |__e| __e.is_vertical())
    }

    /// Option.getOffset as the dispatch of OptionInfo.getOffset
    pub fn get_offset_dyn(__this: &compat::Shared<dyn OptionInfo>) -> i32 {
        compat::read_as::<Option, _, _>(__this, |__e| __e.get_offset())
    }

    /// Option.getOptionSize as the dispatch of OptionInfo.getOptionSize
    pub fn get_option_size_dyn(__this: &compat::Shared<dyn OptionInfo>) -> Vec<i32> {
        compat::read_as::<Option, _, _>(__this, |__e| __e.get_option_size())
    }

    /// Option.getOptionTexture as the dispatch of OptionInfo.getOptionTexture
    pub fn get_option_texture_dyn(__this: &compat::Shared<dyn OptionInfo>) -> Identifier {
        compat::read_as::<Option, _, _>(__this, |__e| __e.get_option_texture())
    }

    /// Option.getBackTexture as the dispatch of OptionInfo.getBackTexture
    pub fn get_back_texture_dyn(__this: &compat::Shared<dyn OptionInfo>) -> Identifier {
        compat::read_as::<Option, _, _>(__this, |__e| __e.get_back_texture())
    }

    /// Option.getHoverTexture as the dispatch of OptionInfo.getHoverTexture
    pub fn get_hover_texture_dyn(__this: &compat::Shared<dyn OptionInfo>) -> Identifier {
        compat::read_as::<Option, _, _>(__this, |__e| __e.get_hover_texture())
    }

}
