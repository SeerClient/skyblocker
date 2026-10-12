//! Ported from de.hysky.skyblocker.skyblock.radialMenu.RadialMenuManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::bags_menu::BagsMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::bank_menu::BankMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::booster_cookie_menu::BoosterCookieMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::collections_menu::CollectionsMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::farming_toolkit::FarmingToolkit;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::fast_travel_menu::FastTravelMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::hunting_menu::HuntingMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::profile_management_menu::ProfileManagementMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::sack_of_sacks_menu::SackOfSacksMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::sacks_menu::SacksMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::skills_menu::SkillsMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::skyblock_menu::SkyblockMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::storage::Storage;
use crate::de::hysky::skyblocker::skyblock::radial_menu::menus::wardrobe_menu::WardrobeMenu;
use crate::de::hysky::skyblocker::skyblock::radial_menu::radial_menu::RadialMenu;

/// java: private static final RadialMenu[] menus = new RadialMenu[]{ new BagsMenu(), new BankMenu(), new BoosterCookieMenu(), new CollectionsMenu(), new FastTravelMenu(), new HuntingMenu(), new FarmingToolkit(
pub static MENUS: compat::MainThread<Vec<compat::Shared<dyn RadialMenu>>> = compat::MainThread::new(|| vec![(BagsMenu::new() as compat::Shared<dyn RadialMenu>), (BankMenu::new() as compat::Shared<dyn RadialMenu>), (BoosterCookieMenu::new() as compat::Shared<dyn RadialMenu>), (CollectionsMenu::new() as compat::Shared<dyn RadialMenu>), (FastTravelMenu::new() as compat::Shared<dyn RadialMenu>), (HuntingMenu::new() as compat::Shared<dyn RadialMenu>), (FarmingToolkit::new() as compat::Shared<dyn RadialMenu>), (ProfileManagementMenu::new() as compat::Shared<dyn RadialMenu>), (SackOfSacksMenu::new() as compat::Shared<dyn RadialMenu>), (SacksMenu::new() as compat::Shared<dyn RadialMenu>), (SkillsMenu::new() as compat::Shared<dyn RadialMenu>), (SkyblockMenu::new() as compat::Shared<dyn RadialMenu>), (Storage::new() as compat::Shared<dyn RadialMenu>), (WardrobeMenu::new() as compat::Shared<dyn RadialMenu>)]);

/// java: public class RadialMenuManager
#[derive(Clone)]
pub struct RadialMenuManager {}

impl RadialMenuManager {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static boolean isMenuExistsFromTitle(String title)
    pub fn is_menu_exists_from_title(title: &str) -> bool {
        return RadialMenuManager::get_menu_stream().any(|menu| ({ let __f = compat::dispatch(&menu, |__t| __t.title_matches()); __f(&menu, title) }));
    }

    /// java: public static @Nullable RadialMenu getMenuFromTitle(String title)
    pub fn get_menu_from_title(title: &str) -> Option<compat::Shared<dyn RadialMenu>> {
        return RadialMenuManager::get_menu_stream().filter(|menu| ({ let __f = compat::dispatch(menu, |__t| __t.title_matches()); __f(menu, title) })).next();
    }

    /// java: public static Stream<RadialMenu> getMenuStream()
    pub fn get_menu_stream() -> Box<dyn Iterator<Item = compat::Shared<dyn RadialMenu>>> {
        return Box::new(MENUS.get_mut().iter().cloned());
    }

}
