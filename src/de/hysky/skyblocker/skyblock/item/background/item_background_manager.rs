//! Ported from de.hysky.skyblocker.skyblock.item.background.ItemBackgroundManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::item::background::adders::item_rarity_background::ItemRarityBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::adders::jacob_medal_background::JacobMedalBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::adders::legacy_attribute_background::LegacyAttributeBackground;
use crate::de::hysky::skyblocker::skyblock::item::background::colored_item_background::AnyColoredItemBackground;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;

/// java: private static final List<ColoredItemBackground<?>> BACKGROUNDS = List.of( new ItemRarityBackground(), new JacobMedalBackground(), new LegacyAttributeBackground() );
pub static BACKGROUNDS: compat::MainThread<Vec<compat::Shared<dyn AnyColoredItemBackground>>> = compat::MainThread::new(|| vec![(compat::shared(ItemRarityBackground::new()) as compat::Shared<dyn AnyColoredItemBackground>), (compat::shared(JacobMedalBackground::new()) as compat::Shared<dyn AnyColoredItemBackground>), (compat::shared(LegacyAttributeBackground::new()) as compat::Shared<dyn AnyColoredItemBackground>)]);

/// java: public class ItemBackgroundManager
#[derive(Clone)]
pub struct ItemBackgroundManager {}

impl ItemBackgroundManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // Clear the cache of every background every 5 minutes
        for background in BACKGROUNDS.get().iter() {
            let background = compat::RenderThread::new(background.clone());
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { compat::write(background.get(), |__t| __t.clear_cache()); }), 6000) });
        }
        screen_events::BEFORE_INIT.register(Arc::new(|screen: &mut Screen, width: i32, height: i32| {
            // String title = screen.getTitle().getString(); (the port keeps the title as its string)
            let mut title: String = screen.title.clone();
            for background in BACKGROUNDS.get().iter() {
                compat::write(background, |__t| __t.on_screen_change(&title, &screen));
            }
        }));
    }

    /// java: public static void drawBackgrounds(ItemStack stack, GuiGraphicsExtractor graphics, int x, int y)
    pub fn draw_backgrounds(stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32) {
        for background in BACKGROUNDS.get().iter() {
            if compat::read(background, |__t| __t.is_enabled()) {
                compat::write(background, |__t| __t.try_extract(stack, graphics, x, y));
            }
        }
    }

}
