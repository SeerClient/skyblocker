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
use crate::de::hysky::skyblocker::skyblock::profileviewer2::pages::profile_viewer_page::ProfileViewerPageAny;
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
    pub pages: Vec<compat::Shared<dyn ProfileViewerPageAny>>,
    /// java: private final Set<ProfileViewerPage<?>> loadedPages = new HashSet<>(); (the pages keep Object's identity
    /// `equals`: an identity set)
    pub loaded_pages: Vec<compat::Shared<dyn ProfileViewerPageAny>>,
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
                pages: vec![(compat::shared(SkillsPage::new()) as compat::Shared<dyn ProfileViewerPageAny>), (compat::shared(CombatPage::new()) as compat::Shared<dyn ProfileViewerPageAny>), (compat::shared(CatacombsPage::new()) as compat::Shared<dyn ProfileViewerPageAny>), (compat::shared(InventoryPage::new()) as compat::Shared<dyn ProfileViewerPageAny>), (compat::shared(CollectionsPage::new()) as compat::Shared<dyn ProfileViewerPageAny>), (compat::shared(MiningPage::new()) as compat::Shared<dyn ProfileViewerPageAny>)],
                loaded_pages: Vec::new(),
                // java: this.createPageTabs() - it reads `pages` and hands `this` on: run on the handle below
                tab_widgets: Vec::new(),
                content_layout: FrameLayout::of(*abstract_profile_viewer_screen::CONTENT_WIDTH, *abstract_profile_viewer_screen::CONTENT_HEIGHT),
                selected_page_index: 0,
            });
            ((__h.clone() as compat::Shared<dyn AbstractProfileViewerScreen>), __h)
        });
        { let __v = ProfileViewerScreen::create_page_tabs(&__this); compat::write(&__this, |__t| __t.tab_widgets = __v); }
        ProfileViewerScreen::load_pages(&__this);
        ProfileViewerScreen::set_selected_page(&__this, 0);
        __this
    }

    /// java: private List<PageTabWidget> createPageTabs()
    pub fn create_page_tabs(__this: &compat::Shared<Self>) -> Vec<compat::Shared<PageTabWidget>> {
        let mut tabs: Vec<compat::Shared<PageTabWidget>> = Vec::new();
        for i in 0..compat::read(__this, |__t| __t.pages.len() as i32) {
            // (the screen holds its tabs: the tab's lambda holds the screen weakly)
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __page = compat::read(__this, |__t| __t.pages[(i) as usize].clone());
            let __icon = compat::read(&__page, |__p| __p.get_icon());
            tabs.push(PageTabWidget::new(__icon, i, Arc::new(move |__a0| { if let Some(__this_1) = __this_w1.get().upgrade() { ProfileViewerScreen::set_selected_page(&__this_1, __a0); } })));
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
            // java: page.load(loadingInformation).thenAcceptAsync(layoutElement -> {..}, minecraft).exceptionallyAsync(throwable -> {..}, minecraft);
            let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
            let __page_1 = page.clone();
            let __page_2 = page.clone();
            let __load = compat::dispatch(&page, |__t| __t.load());
            __load(&page, &loading_information)
                .then_accept_async(move |layout_element: LayoutElement| {
                    let Some(__this_1) = __this_w1.get().upgrade() else { return; };
                    if compat::read(&__page_1, |__p| __p.centred()) {
                        compat::write(&__this_1, |__t| __t.content_layout.add_child(layout_element.clone()));
                    } else {
                        compat::write(&__this_1, |__t| __t.content_layout.add_child_adjusted(layout_element.clone(), &mut |l| { *l = l.align_vertically_top().align_horizontally_left(); }));
                    }
                    // java: this.loadedPages.add(page); (first here: the content layout's children are the loaded pages' layouts,
                    // in the order they were added, which is how their widgets find their page below)
                    compat::write(&__this_1, |__t| if !__t.loaded_pages.iter().any(|p| std::sync::Arc::ptr_eq(p, &__page_1)) { __t.loaded_pages.push(__page_1.clone()); });
                    // java: this.contentLayout.arrangeElements();
                    compat::write(&__this_1, |__t| __t.content_layout.arrange_elements());
                    ProfileViewerScreen::reposition_elements(&__this_1);
                })
                .exceptionally_async(move |_throwable: String| {
                    logger().error(&format!("[Skyblocker Profile Viewer] Failed to load {} page!", compat::read(&__page_2, |__p| __p.get_name()).get_string()));
                    // java: return null;
                });
        }
    }

    /// java: private ProfileViewerPage<?> getSelectedPage()
    pub fn get_selected_page(__this: &compat::Shared<Self>) -> compat::Shared<dyn ProfileViewerPageAny> {
        return compat::read(__this, |__t| __t.pages[(__t.selected_page_index) as usize].clone());
    }

    /// java: public void setSelectedPage(int index)
    pub fn set_selected_page(__this: &compat::Shared<Self>, index: i32) {
        // java: this.selectedPageIndex = Preconditions.checkPositionIndex(index, this.pages.size());
        let size = compat::read(__this, |__t| __t.pages.len() as i32);
        if index < 0 || index > size {
            panic!("IndexOutOfBoundsException: index ({}) must not be {} size ({})", index, if index < 0 { "negative" } else { "greater than" }, size);
        }
        compat::write(__this, |__t| __t.selected_page_index = index);
        let tab_widgets = compat::read(__this, |__t| __t.tab_widgets.clone());
        for tab_widget in tab_widgets.iter() {
            // java: tabWidget.setSelected(this.selectedPageIndex == this.tabWidgets.indexOf(tabWidget)); (AbstractWidget keeps
            // Object's identity `equals`)
            let __i = tab_widgets.iter().position(|e| std::sync::Arc::ptr_eq(e, tab_widget)).map_or(-1, |i| i as i32);
            PageTabWidget::set_selected(tab_widget, compat::read(__this, |__t| __t.selected_page_index) == __i);
        }
    }

    /// The content layout's widgets moved where it put them (Java's layout moves the widget objects themselves): the
    /// content layout's children are the loaded pages' layouts, in `loadedPages` order, and each one's widget elements
    /// index its page's widget list.
    pub fn place_page_widgets(__this: &compat::Shared<Self>) {
        let mut __per_page: Vec<Vec<(usize, i32, i32)>> = Vec::new();
        compat::read(__this, |__t| __t.content_layout.visit_children(&mut |__child| {
            let mut __visited = Vec::new();
            __child.visit_widgets(&mut __visited);
            __per_page.push(__visited);
        }));
        let __pages = compat::read(__this, |__t| __t.loaded_pages.clone());
        for (__page, __visited) in __pages.iter().zip(__per_page) {
            compat::write(__page, |__p| {
                let __widgets = __p.widgets_mut();
                for (__index, __x, __y) in __visited {
                    if __index < __widgets.children().len() {
                        __widgets.widget_mut(__index).set_position(__x, __y);
                    }
                }
            });
        }
    }

    /// `Screen.mouseClicked` - `ContainerEventHandler.mouseClicked:39-55` over `children()`: the selected page's widgets
    /// first (held in the page's list, which a press lambda of theirs is lent as its `screen`; taken out of the page for
    /// the call, so a press reaches the page freely), then the tabs, which the game offers the click to through
    /// `children()` when this answers `false`.
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) -> bool {
        let __page = ProfileViewerScreen::get_selected_page(__this);
        if !compat::read(__this, |__t| __t.loaded_pages.iter().any(|p| std::sync::Arc::ptr_eq(p, &__page))) {
            return false;
        }
        let mut __widgets = compat::write(&__page, |__p| std::mem::take(__p.widgets_mut()));
        let __took = compat::children_mouse_clicked(&mut __widgets, event);
        compat::write(&__page, |__p| *__p.widgets_mut() = __widgets);
        __took
    }

    /// `Screen.mouseReleased` - `ContainerEventHandler.mouseReleased:57-66` over the selected page's widgets
    pub fn mouse_released(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) {
        let __page = ProfileViewerScreen::get_selected_page(__this);
        if !compat::read(__this, |__t| __t.loaded_pages.iter().any(|p| std::sync::Arc::ptr_eq(p, &__page))) {
            return;
        }
        let mut __widgets = compat::write(&__page, |__p| std::mem::take(__p.widgets_mut()));
        compat::children_mouse_released(&mut __widgets, event);
        compat::write(&__page, |__p| *__p.widgets_mut() = __widgets);
    }

    /// java: @Override public List<? extends GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<AbstractWidget> {
        let mut children: Vec<AbstractWidget> = Vec::new();
        let __page = ProfileViewerScreen::get_selected_page(__this);
        children.extend(compat::read(&__page, |__p| __p.get_widgets()).into_iter());
        children.extend(compat::read(__this, |__t| __t.tab_widgets.clone()).into_iter().map(|__w| compat::WidgetAdapter::widget(__w)));
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
        // (the widgets follow the layout - see `place_page_widgets`)
        ProfileViewerScreen::place_page_widgets(__this);
    }

    /// java: private void extractTabButtons(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a, boolean onlySelected)
    pub fn extract_tab_buttons(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32, mut only_selected: bool) {
        for tab_widget in compat::read(__this, |__t| __t.tab_widgets.clone()) {
            if only_selected && compat::read(__this, |__t| __t.tab_widgets.iter().position(|e| std::sync::Arc::ptr_eq(e, &tab_widget)).map_or(-1, |i| i as i32)) != compat::read(__this, |__t| __t.selected_page_index) {
                continue;
            }
            //   java: tabWidget.extractRenderState(graphics, mouseX, mouseY, a);
            // `AbstractWidget.extractRenderState:55-61` over the tab's own widget half (the screen draws its tabs itself;
            // they are not in its widget list): `if (this.visible) { this.isHovered = ..; this.extractWidgetRenderState(..) }`.
            let visible: bool = compat::write(&tab_widget, |__t| {
                let widget: &mut AbstractWidget = &mut __t.base.base;
                widget.set_hovered_from_extract(mouse_x, mouse_y);
                widget.visible
            });
            if visible {
                PageTabWidget::extract_widget_render_state(&tab_widget, graphics, mouse_x, mouse_y, a);
            }
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
        let mut selected_page: compat::Shared<dyn ProfileViewerPageAny> = ProfileViewerScreen::get_selected_page(__this);
        if compat::read(__this, |__t| __t.loaded_pages.iter().any(|p| std::sync::Arc::ptr_eq(p, &selected_page))) {
            //   java: for (AbstractWidget widget : selectedPage.getWidgets()) widget.extractRenderState(graphics, mouseX, mouseY, a);
            // The page's widgets are game widgets the screen draws itself: held in a `Screen` for the call and drawn by the
            // widget pass (`WidgetRenderContext::extract_widgets` - `extractRenderState:56-62`'s hover test, then the pass).
            // (the page's own list, taken out of the page for the draw)
            let mut __page_widgets = compat::write(&selected_page, |__p| std::mem::take(__p.widgets_mut()));
            mcrust::mcrust::modding::widgets::WidgetRenderContext::new(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft()).extract_widgets(&mut __page_widgets, mouse_x, mouse_y);
            compat::write(&selected_page, |__p| *__p.widgets_mut() = __page_widgets);
        } else {
            let mut centre_x: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_x()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_WIDTH).wrapping_div(2));
            let mut centre_y: i32 = ({ let __f = compat::dispatch(__this, |__t| __t.get_background_y()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractProfileViewerScreen>)) }).wrapping_add((abstract_profile_viewer_screen::BACKGROUND_HEIGHT).wrapping_div(2));
            let mut time_loading_page: i64 = (compat::millis()).wrapping_sub(compat::read(__this, |__t| __t.opened_at));
            let mut page_loading_text: MutableComponent = component::empty().append(component::literal("Loading the ")).append(compat::read(&selected_page, |__p| __p.get_name())).append(component::literal(" page..."));
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

    /// java: Screen.mouseClicked (the base body) - `ContainerEventHandler` over `children()`
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        let __event = MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0));
        self.lend(screen, |__h| ProfileViewerScreen::mouse_clicked(__h, &__event))
    }

    /// java: Screen.mouseReleased (the base body) - `ContainerEventHandler` over `children()`
    fn mouse_released(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        let __event = MouseButtonEvent::new(x, y, MouseButtonInfo::new(button, 0));
        self.lend(screen, |__h| ProfileViewerScreen::mouse_released(__h, &__event));
        false
    }

    /// java: @Override public List<? extends GuiEventListener> children() - the selected page's widgets and the tabs
    fn children(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen) -> Option<Vec<AbstractWidget>> {
        Some(self.lend_ref(screen, |__h| ProfileViewerScreen::children(__h)))
    }

}
