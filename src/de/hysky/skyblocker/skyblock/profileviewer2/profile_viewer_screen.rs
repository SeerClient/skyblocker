//! Ported from de.hysky.skyblocker.skyblock.profileviewer2.ProfileViewerScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen::AbstractProfileViewerScreen;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::abstract_profile_viewer_screen::AbstractProfileViewerScreenBase;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::loading_information::LoadingInformation;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile::ApiProfile;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::api_profile_response::ApiProfileResponse;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::model::profile_member::ProfileMember;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::catacombs_page::CatacombsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::collections_page::CollectionsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::combat_page::CombatPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::inventory_page::InventoryPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::mining_page::MiningPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::skills_page::SkillsPage;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::utils::item_loader::ItemLoader;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::page_tab_widget::PageTabWidget;
use crate::de::hysky::skyblocker::skyblock::profileviewer2::widgets::profile_viewer_widget::ProfileViewerWidget;
use mcrust::com::mojang::authlib::game_profile::GameProfile;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout;
use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
use mcrust::net::minecraft::client::gui::screens::loading_dots_text;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::client::gui::components::renderable::Renderable as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public final class ProfileViewerScreen extends AbstractProfileViewerScreen
pub struct ProfileViewerScreen {
    /// java: extends AbstractProfileViewerScreen
    pub base: AbstractProfileViewerScreenBase,
    /// java: @SuppressWarnings("unused") private final ApiProfileResponse apiProfileResponse;
    pub api_profile_response: compat::Shared<ApiProfileResponse>,
    /// java: private final ApiProfile profile;
    pub profile: compat::Shared<ApiProfile>,
    /// java: private final GameProfile userProfile;
    pub user_profile: GameProfile,
    /// java: private final ProfileMember member;
    pub member: compat::Shared<ProfileMember>,
    /// java: private final Map<String, Integer> leaderboards;
    pub leaderboards: std::collections::HashMap<String, i32>,
    /// java: private final long openedAt = System.currentTimeMillis();
    pub opened_at: i64,
    /// java: private final List<ProfileViewerPage<?>> pages = List.of(new SkillsPage(), new CombatPage(), new CatacombsPage(), new InventoryPage(), new CollectionsPage(), ne
    pub pages: Vec<compat::Shared<dyn ProfileViewerPage<compat::Object>>>,
    /// java: private final Set<ProfileViewerPage<?>> loadedPages = new HashSet<>();
    pub loaded_pages: std::collections::HashSet<compat::Shared<dyn ProfileViewerPage<compat::Object>>>,
    /// java: private final List<PageTabWidget> tabWidgets = this.createPageTabs();
    pub tab_widgets: Vec<compat::Shared<PageTabWidget>>,
    /// java: private final FrameLayout contentLayout = new FrameLayout(CONTENT_WIDTH, CONTENT_HEIGHT);
    pub content_layout: FrameLayout,
    /// java: private int selectedPageIndex;
    pub selected_page_index: i32,
}

impl std::ops::Deref for ProfileViewerScreen {
    type Target = AbstractProfileViewerScreenBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileViewerScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileViewerScreen {
    /// java: protected ProfileViewerScreen(ApiProfileResponse apiProfileResponse, ApiProfile profile, GameProfile userProfile, ProfileMember member, Map<String, Integer> leaderboards)
    pub fn new(mut api_profile_response: compat::Shared<ApiProfileResponse>, mut profile: compat::Shared<ApiProfile>, mut user_profile: GameProfile, mut member: compat::Shared<ProfileMember>, mut leaderboards: std::collections::HashMap<String, i32>) -> compat::Shared<Self> {
        let __init_api_profile_response = api_profile_response.clone();
        let __init_profile = profile.clone();
        let __init_user_profile = user_profile.clone();
        let __init_member = member.clone();
        let __init_leaderboards = leaderboards.clone();
        let (_, __this) = AbstractProfileViewerScreenBase::new_in(component::literal("Skyblocker Profile Viewer"), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                api_profile_response: __init_api_profile_response,
                profile: __init_profile,
                user_profile: __init_user_profile,
                member: __init_member,
                leaderboards: __init_leaderboards,
                opened_at: compat::millis(),
                pages: vec![(compat::shared(SkillsPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>), (compat::shared(CombatPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>), (compat::shared(CatacombsPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>), (compat::shared(InventoryPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>), (compat::shared(CollectionsPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>), (compat::shared(MiningPage::new()) as compat::Shared<dyn ProfileViewerPage<compat::Object>>)],
                loaded_pages: std::collections::HashSet::new(),
                tab_widgets: ProfileViewerScreen::create_page_tabs(&this),
                content_layout: FrameLayout::of(*abstract_profile_viewer_screen::CONTENT_WIDTH, *abstract_profile_viewer_screen::CONTENT_HEIGHT),
                selected_page_index: 0,
            });
            ((__h.clone() as compat::Shared<dyn AbstractProfileViewerScreen>), __h)
        });
        ProfileViewerScreen::load_pages(&__this);
        ProfileViewerScreen::set_selected_page(&__this, 0);
        __this
    }

    /// java: private List<PageTabWidget> createPageTabs()
    pub fn create_page_tabs(__this: &compat::Shared<Self>) -> Vec<compat::Shared<PageTabWidget>> {
        let mut tabs: Vec<compat::Shared<PageTabWidget>> = Vec::new();
        for i in 0..(__this.lock().borrow().pages.len() as i32) {
            let __this_1 = compat::RenderThread::new(__this.clone());
            tabs.push(PageTabWidget::new((__this.lock().borrow().pages[(i) as usize].lock().borrow()).get_icon(), i, Arc::new(move |__a0| { ProfileViewerScreen::set_selected_page(__this_1.get(), __a0); })));
        }
        return tabs.iter().cloned().collect::<Vec<_>>();
    }

    /// java: private LoadingInformation createLoadingInformation()
    pub fn create_loading_information(__this: &compat::Shared<Self>) -> LoadingInformation {
        return LoadingInformation::new(compat::read(__this, |__t| __t.profile.clone()), compat::read(__this, |__t| __t.user_profile.clone()), compat::read(__this, |__t| __t.member.clone()), compat::read(__this, |__t| __t.leaderboards.clone()), ItemLoader::decode_items(&compat::read(__this, |__t| __t.member.clone())));
    }

    /// java: private void loadPages()
    pub fn load_pages(__this: &compat::Shared<Self>) {
        let mut loading_information: LoadingInformation = ProfileViewerScreen::create_loading_information(__this);
        for page in compat::read(__this, |__t| __t.pages.clone()) {
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            { let (__q0, __q1,) = (|throwable| {
                logger().error(&format!("[Skyblocker Profile Viewer] Failed to load {} page!", page.lock().borrow().get_name().get_string()));
                return Default::default(); // java: return null
            }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)),); ({ let (__q0, __q1,) = (|layout_element| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
                if page.lock().borrow().centred() {
                    { let (__q0,) = (layout_element.clone(),); (&__this_1).lock().borrow_mut().content_layout.add_child(__q0) };
                } else {
                    { let (__q0, __q1,) = (layout_element.clone(), &mut |l| { l.align_vertically_top().align_horizontally_left(); },); (&__this_1).lock().borrow_mut().content_layout.add_child_adjusted(__q0, __q1) };
                }
                { (&__this_1).lock().borrow_mut().content_layout.arrange_elements(); let mut __vw = Vec::new(); (&__this_1).lock().borrow().content_layout.visit_widgets(&mut __vw); for (__i, __x, __y) in __vw { compat::write((&__this_1), |__t| __t.widget_mut(__i).set_position(__x, __y)); } };
                ProfileViewerScreen::reposition_elements((&__this_1));
                let __h0 = page.clone();
                (&__this_1).lock().borrow_mut().loaded_pages.insert(__h0);
            } }, std::sync::Arc::new(|__t: Box<dyn FnOnce() + Send>| mcrust::mcrust::modding::game::execute(__t)),); page.lock().borrow().load(&loading_information).then_accept_async(__q0, __q1) }).exceptionally_async(__q0, __q1) };
        }
    }

    /// java: private ProfileViewerPage<?> getSelectedPage()
    pub fn get_selected_page(__this: &compat::Shared<Self>) -> compat::Shared<dyn ProfileViewerPage<compat::Object>> {
        return __this.lock().borrow().pages[(compat::read(__this, |__t| __t.selected_page_index)) as usize].clone();
    }

    /// java: public void setSelectedPage(int index)
    pub fn set_selected_page(__this: &compat::Shared<Self>, mut index: i32) {
        // TODO(fabric2seer): static call Preconditions.checkPositionIndex(..)
        //   java: this.selectedPageIndex = Preconditions.checkPositionIndex(index, this.pages.size());
        for tab_widget in compat::read(__this, |__t| __t.tab_widgets.clone()) {
            PageTabWidget::set_selected(&tab_widget, (compat::read(__this, |__t| __t.selected_page_index) == __this.lock().borrow().tab_widgets.iter().position(|e| *e == tab_widget.clone()).map_or(-1, |i| i as i32)));
        }
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        let mut children: Vec<AbstractWidget> = Vec::new();
        children.extend(ProfileViewerScreen::get_selected_page(__this).lock().borrow().get_widgets().into_iter());
        children.extend(compat::read(__this, |__t| __t.tab_widgets.clone()).into_iter());
        return children;
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        ProfileViewerScreen::reposition_elements(__this);
    }

    /// java: @Override protected void repositionElements()
    pub fn reposition_elements(__this: &compat::Shared<Self>) {
        for widget in compat::read(__this, |__t| __t.tab_widgets.clone()) {
            ({ let __f = compat::dispatch(&widget, |__t| __t.update_position()); __f(&compat::view(&widget, |a| a as compat::Shared<dyn ProfileViewerWidget>), ({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }), ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) })) });
        }
        { let (__q0, __q1,) = (({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add(abstract_profile_viewer_screen::PADDING), ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add(abstract_profile_viewer_screen::PADDING),); __this.lock().borrow_mut().content_layout.set_position(__q0, __q1) };
    }

    /// java: private void extractTabButtons(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, boolean onlySelected)
    pub fn extract_tab_buttons(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32, mut only_selected: bool) {
        for tab_widget in compat::read(__this, |__t| __t.tab_widgets.clone()) {
            if only_selected && __this.lock().borrow().tab_widgets.iter().position(|e| *e == tab_widget.clone()).map_or(-1, |i| i as i32) != compat::read(__this, |__t| __t.selected_page_index) {
                continue;
            }
            // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
            // rust: let __wa1 = graphics.clone();
            // rust: let __wa2 = mouse_x;
            // rust: let __wa3 = mouse_y;
            // rust: let __wa4 = a;
            // rust: let __o1 = tab_widget.clone();
            // rust: compat::write(&__o1, |__t| __t.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), __wa2, __wa3, __wa4));
        }
    }

}

impl AbstractProfileViewerScreen for ProfileViewerScreen {
    fn abstract_profile_viewer_screen_base(&self) -> &AbstractProfileViewerScreenBase {
        &self.base
    }
    fn abstract_profile_viewer_screen_base_mut(&mut self) -> &mut AbstractProfileViewerScreenBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a) (dispatch: the body for this class)
    fn extract_render_state(&self) -> fn(&compat::Shared<dyn AbstractProfileViewerScreen>, &GuiGraphicsExtractor, i32, i32, f32) {
        ProfileViewerScreen::extract_render_state_dyn
    }

}

impl ProfileViewerScreen {
    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state_impl(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ProfileViewerScreen::extract_tab_buttons(__this, graphics, mouse_x, mouse_y, a, false);
        <dyn AbstractProfileViewerScreen>::extract_render_state_body(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>), graphics, mouse_x, mouse_y, a);
        ProfileViewerScreen::extract_tab_buttons(__this, graphics, mouse_x, mouse_y, a, true);
        let mut selected_page: compat::Shared<dyn ProfileViewerPage<compat::Object>> = ProfileViewerScreen::get_selected_page(__this);
        if __this.lock().borrow().loaded_pages.contains(&selected_page) {
            for widget in selected_page.lock().borrow().get_widgets() {
                // TODO(fabric2seer): `extract_render_state` is a method of the port's trait `Renderable`, which the port's `AbstractWidget` does not implement
                // rust: widget.extract_render_state(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mouse_x, mouse_y, a);
            }
        } else {
            let mut centre_x: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_WIDTH).wrapping_div(2));
            let mut centre_y: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_HEIGHT).wrapping_div(2));
            let mut time_loading_page: i64 = (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.opened_at));
            let mut page_loading_text: MutableComponent = { let (__q0,) = (component::literal(" page..."),); component::empty().append(component::literal("Loading the ")).append(selected_page.lock().borrow().get_name()).append(__q0) };
            let mut loading_dots_text: MutableComponent = component::literal(loading_dots_text::get(time_loading_page));
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(page_loading_text.clone()), centre_x, centre_y.wrapping_sub(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
            gui_graphics_extractor::centered_text(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(loading_dots_text.clone()), centre_x, centre_y.wrapping_add(mcrust::net::minecraft::client::gui::font::LINE_HEIGHT), mcrust::net::minecraft::util::common_colors::WHITE);
        }
    }

    /// ProfileViewerScreen.extractRenderState as the dispatch of AbstractProfileViewerScreen.extractRenderState
    pub fn extract_render_state_dyn(__this: &compat::Shared<dyn AbstractProfileViewerScreen>, __p0: &GuiGraphicsExtractor, __p1: i32, __p2: i32, __p3: f32) {
        Self::extract_render_state_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2, __p3)
    }

}

/// ProfileViewerScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ProfileViewerScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = &mut self.base;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ProfileViewerScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ProfileViewerScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ProfileViewerScreen::init(__h))
    }

    /// java: @Override protected void repositionElements()
    fn reposition_elements(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ProfileViewerScreen::reposition_elements(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| ({ let __f = compat::dispatch(__h, |__t| __t.extract_render_state()); __f(&compat::view(__h, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>), &context.graphics, mouse_x, mouse_y, a) }))
    }

    /// java: Screen.keyPressed (the base body)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |_| ());
        if event.is_escape() && self.should_close_on_esc() {
            self.on_close();
            return true;
        }
        false
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

}
