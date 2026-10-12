//! Ported from de.hysky.skyblocker.config.backup.ConfigBackupScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::backup::config_backup_manager::ConfigBackupManager;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::mcrust::modding::widgets::ModList;
use mcrust::mcrust::modding::widgets::ModListEntry;
use mcrust::mcrust::modding::widgets::ModSelectionList;
use mcrust::mcrust::modding::widgets::WidgetRenderContext;
use mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList;
use mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
use mcrust::net::minecraft::client::gui::components::abstract_widget::OnPress;
use mcrust::net::minecraft::client::gui::components::abstract_widget::PressLambda;
use mcrust::net::minecraft::client::gui::components::button;
use mcrust::net::minecraft::client::gui::components::events::gui_event_listener::GuiEventListener;
use mcrust::net::minecraft::client::gui::components::toasts::system_toast::SystemToast;
use mcrust::net::minecraft::client::gui::gui;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::gui::narration::narratable_entry::NarratableEntry;
use mcrust::net::minecraft::client::gui::screens::confirm_screen::ConfirmScreen;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class ConfigBackupScreen extends Screen
pub struct ConfigBackupScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// java: private BackupListWidget listWidget;
    pub list_widget: Option<compat::WidgetRef>,
    /// java: private SettingsListWidget detailsWidget;
    pub details_widget: Option<compat::WidgetRef>,
    /// java: private Set<String> changedPaths = Collections.emptySet();
    pub changed_paths: std::collections::HashSet<String>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for ConfigBackupScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ConfigBackupScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ConfigBackupScreen {
    /// java: public ConfigBackupScreen(@Nullable Screen parent)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(component::translatable("skyblocker.config.general.backup.title").get_string()),
            self_ref: std::sync::Weak::new(),
            __widgets: Default::default(),
            parent: __init_parent,
            list_widget: None,
            details_widget: None,
            changed_paths: Default::default(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override protected void init()
    pub fn init(__this: &compat::Shared<Self>) {
        let mut list_width: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(4);
        let mut details_width: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_sub(list_width)).wrapping_sub(8);
        let mut list_height: i32 = (compat::read(__this, |__t| __t.height)).wrapping_sub(64);
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.list_widget.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<BackupListWidget>(__w).expect("a mod list reached from inside its own method"))), None => None } }).is_none() {
            { let __v = (Some(BackupListWidget::new(compat::minecraft(), list_width, list_height, 32, 25))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.list_widget = __v; }); }
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<BackupListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || BackupListWidget::update_entries(__h)) })).expect("a mod list reached from inside its own method")) });
        } else {
            let __wa1 = list_width;
            let __wa2 = list_height;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_size(__wa1, __wa2)) });
        }
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_x(4)) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.refresh_scroll_amount())).expect("a mod list reached from inside its own method")) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        if compat::write(__this, |__t| { let __s = &mut *__t; match __s.details_widget.as_mut() { Some(__r) => Some(__s.__widgets.with(&mut __s.base, __r, |__w| compat::list_object::<SettingsListWidget>(__w).expect("a mod list reached from inside its own method"))), None => None } }).is_none() {
            { let __v = (Some(SettingsListWidget::new(compat::minecraft(), details_width, list_height, 32, 10))).map(|__o| compat::WidgetRef::value(compat::attach_list(__o))); compat::write(__this, |__t| { let __s = &mut *__t; __s.details_widget = __v; }); }
            let __wa3 = compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<BackupListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || BackupListWidget::get_selected_path(__h)) })).expect("a mod list reached from inside its own method")) });
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.details_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<SettingsListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || SettingsListWidget::update_entries(__h, __wa3.clone())) })).expect("a mod list reached from inside its own method")) });
        } else {
            let __wa4 = details_width;
            let __wa5 = list_height;
            compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.details_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_size(__wa4, __wa5)) });
        }
        let __wa6 = list_width.wrapping_add(8);
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.details_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.set_x(__wa6)) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.details_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| __l.list.refresh_scroll_amount())).expect("a mod list reached from inside its own method")) });
        compat::write(__this, |__t| { let __s = &mut *__t; let __r = __s.details_widget.as_mut().unwrap(); __s.__widgets.add(&mut __s.base, __r, true) });
        let __this_w2 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut restore_btn: AbstractWidget = (button::builder(component::translatable("skyblocker.config.general.backup.restore").get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_2) = __this_w2.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_2, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); {
            let mut selected: Option<std::path::PathBuf> = compat::write((&__this_2), |__t| { let __s = &mut *__t; let __r = __s.list_widget.as_mut().unwrap(); __s.__widgets.with(&mut __s.base, __r, |__w| __w.mod_widget().and_then(|__ms| __ms.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|__l| { let __h = &__l.class::<compat::ListClass<BackupListWidget>>().map(|__c| __c.object().clone()).expect("a mod list without its class"); compat::lend_list(__h, __l, || BackupListWidget::get_selected_path(__h)) })).expect("a mod list reached from inside its own method")) });
            if let Some(selected) = &selected {
                // TODO(fabric2seer): static `Component.translatableEscape` has no Rust counterpart in the port
                // TODO(fabric2seer): `new ConfirmScreen(..)`: the port has no `ConfirmScreen::new`
                // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
                // rust: let __this_w1 = compat::RenderThread::new(std::sync::Arc::downgrade((&__this_2)));
                // rust: gui::set_screen(&mut ConfirmScreen::new(Arc::new(move |confirm| { let Some(__this_1) = __this_w1.get().upgrade() else { return Default::default(); }; {
                // rust: if confirm {
                // rust: // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
                // rust: let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // rust: ConfigBackupManager::restore_backup(selected);
                // rust: }));
                // rust: if let Err(e) = __result {
                // rust: logger().error(&format!("[Skyblocker] Failed to restore backup {}", std::path::PathBuf::from(selected.file_name().unwrap_or_default()).to_string_lossy().to_string()));
                // rust: // TODO(fabric2seer): field `PERIODIC_NOTIFICATION` of a value of unknown type
                // rust: //   java: minecraft.gui.toastManager().addToast(new SystemToast(SystemToast.SystemToastId.PERIODIC_NOTIFICATION,
                // rust: //   java: 									Component.translatable("skyblocker.config.general.backup.restore.error"),
                // rust: //   java: 									null
                // rust: //   java: 							));
                // rust: return;
                // rust: }
                // rust: if compat::read((&__this_1), |__t| __t.parent.clone()).is_some() {
                // rust: // TODO(fabric2seer): `set_screen` takes 14 argument(s) in the port (`gfx: &mut Minecraft, screen: &mut GuiScreen, screen_extra: &mut ScreenExtra, screen_stack: &mut Vec<HeldScreen>, screen_kind_before_pause: &mut Option<ScreenKind>, screen_is_open: &mut bool, level_screen_closed: &mut bool, frames_on_this_sc ..`), Java passes 1
                // rust: // rust: gui::set_screen(&mut SkyblockerConfigManager::create_gui(compat::read((&__this_1), |__t| __t.parent.clone())));
                // rust: } else {
                // rust: mcrust::mcrust::modding::game::set_screen_null();
                // rust: }
                // rust: } else {
                // rust: mcrust::mcrust::modding::game::set_screen(Box::new(compat::ScreenAdapter::new(((&__this_1)).clone())));
                // rust: }
                // rust: } }), component::translatable("skyblocker.config.general.backup.confirm.title"), Component::translatable_escape("skyblocker.config.general.backup.confirm.text".to_string(), std::path::PathBuf::from(selected.file_name().unwrap_or_default()).to_string_lossy().to_string()), mcrust::net::minecraft::network::chat::common_components::gui_yes(), mcrust::net::minecraft::network::chat::common_components::gui_no()));
            }
        } }) })))).size(90, 20).pos(((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(95), (compat::read(__this, |__t| __t.height)).wrapping_sub(28)).build();
        { let (__q0,) = (restore_btn.clone(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let __this_w3 = compat::RenderThread::new(std::sync::Arc::downgrade(__this));
        let mut done: AbstractWidget = (button::builder(mcrust::net::minecraft::network::chat::common_components::gui_done().get_string(), OnPress::Lambda(PressLambda::new(move |__screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, __index: usize| { let Some(__this_3) = __this_w3.get().upgrade() else { return Default::default(); }; compat::lend_screen(&__this_3, __screen, || { let mut __pidx = compat::WidgetRef::Screen(__index); { ConfigBackupScreen::on_close((&__this_3)); } }) })))).size(90, 20).pos(((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_add(5), (compat::read(__this, |__t| __t.height)).wrapping_sub(28)).build();
        { let (__q0,) = (done.clone(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
        let mut title_widget: AbstractWidget = compat::string_widget(0, 0, -1, 9, compat::read(__this, |__t| __t.title.clone()));
        title_widget.set_position(((compat::read(__this, |__t| __t.width)).wrapping_sub(compat::font().width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(compat::read(__this, |__t| __t.title.clone()))))).wrapping_div(2), 12);
        { let (__q0,) = (title_widget.clone(),); __this.lock().borrow_mut().add_renderable_widget(__q0) };
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

}

/// ConfigBackupScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for ConfigBackupScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// ConfigBackupScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<ConfigBackupScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override protected void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| ConfigBackupScreen::init(__h))
    }

    /// java: Screen.tick (the base body)
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |_| ())
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

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| ConfigBackupScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): init, onClose override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class BackupListWidget extends ObjectSelectionList<BackupEntry>
pub struct BackupListWidget {
    /// java: extends net.minecraft.client.gui.components.ObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for BackupListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for BackupListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl BackupListWidget {
    /// java: BackupListWidget(Minecraft client, int width, int height, int y, int itemHeight)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut item_height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Object, width, height, y, item_height),
        });
        BackupListWidget::update_entries(&__this);
        __this
    }

    /// java: Path getSelectedPath()
    pub fn get_selected_path(__this: &compat::Shared<Self>) -> Option<std::path::PathBuf> {
        let mut entry: Option<compat::Shared<BackupEntry>> = compat::write(__this, |__t| { let __l = &mut __t.base; __l.get_selected().and_then(|__i| compat::entry_at::<BackupEntry>(__l, __i)) });
        return (if entry.is_some() { Some(compat::read(entry.as_ref().unwrap(), |__t| __t.path.clone())) } else { None });
    }

    /// java: @Override public void setSelected(@Nullable BackupEntry entry)
    pub fn set_selected(__this: &compat::Shared<Self>, mut entry: Option<compat::Shared<BackupEntry>>) {
        compat::call_super();
        // TODO(fabric2seer): `detailsWidget` is a field of the enclosing instance (ConfigBackupScreen.this) - an inner class has no outer reference in Rust
        //   java: detailsWidget.updateEntries(entry != null ? entry.path : null);
    }

    /// java: void updateEntries()
    pub fn update_entries(__this: &compat::Shared<Self>) {
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        // java: try {..} catch (IOException _) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut backups: Vec<std::path::PathBuf> = ConfigBackupManager::list_backups();
            for backup in backups.iter() {
                let __entry = BackupEntry::new(backup.clone());
                compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        }));
        if let Err(_) = __result {
        }
    }

    /// java: @Override protected int scrollBarX()
    pub fn scroll_bar_x(__this: &compat::Shared<Self>) -> i32 {
        return ((__this.lock().borrow().get_x()).wrapping_add(__this.lock().borrow().get_width())).wrapping_sub(6);
    }

}

/// BackupListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for BackupListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// BackupListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<BackupListWidget> {
    /// java: @Override protected int scrollBarX()
    fn scroll_bar_x(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| BackupListWidget::scroll_bar_x(__h))
    }

    /// java: @Override public void setSelected(@Nullable BackupEntry entry)
    fn set_selected(&self, list: &mut AbstractSelectionList, selected: Option<usize>, super_set_selected: &mut dyn FnMut(&mut AbstractSelectionList, Option<usize>)) {
        super_set_selected(list, selected);
        let __entry = selected.and_then(|__i| compat::row_object::<BackupEntry>(list, __i));
        compat::with_super(&mut || {}, || self.ask(list, (), |__h| BackupListWidget::set_selected(__h, __entry.clone())))
    }

}

// NOTE(fabric2seer): scrollBarX, setSelected override(s) of ObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class BackupEntry extends ObjectSelectionList.Entry<BackupEntry>
pub struct BackupEntry {
    /// java: extends net.minecraft.client.gui.components.ObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final Path path;
    pub path: std::path::PathBuf,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for BackupEntry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for BackupEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl BackupEntry {
    /// java: BackupEntry(Path path)
    pub fn new(mut path: std::path::PathBuf) -> compat::Shared<Self> {
        let __init_path = path.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            path: __init_path,
        });
        __this
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, context: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        // TODO(fabric2seer): unknown name `font`
        //   java: context.centeredText(font, path.getFileName().toString(), this.getContentXMiddle(), this.getY() + 7, 0xFFFFFFFF);
        if { let (__q0, __q1,) = ((mouse_x) as f64, (mouse_y) as f64,); __this.lock().borrow().is_mouse_over(__q0, __q1) } {
            // TODO(fabric2seer): static `CursorTypes.POINTING_HAND` has no Rust counterpart in the port's CursorTypes file
            // TODO(fabric2seer): `GuiGraphicsExtractor.requestCursor` has no Rust method in the port (looked for `request_cursor` in GuiGraphicsExtractor and its supertypes)
            // rust: context.request_cursor(&CursorTypes::POINTING_HAND);
        }
    }

    /// java: @Override public Component getNarration()
    pub fn get_narration(__this: &compat::Shared<Self>) -> MutableComponent {
        return component::empty();
    }

}

/// BackupEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for BackupEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// BackupEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<BackupEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor context, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| BackupEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
    }

}

// NOTE(fabric2seer): extractContent, getNarration override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private class SettingsListWidget extends ContainerObjectSelectionList<StringEntry>
pub struct SettingsListWidget {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList
    pub base: ModSelectionList,
}

impl std::ops::Deref for SettingsListWidget {
    type Target = AbstractSelectionList;
    fn deref(&self) -> &Self::Target {
        &self.base.list
    }
}

impl std::ops::DerefMut for SettingsListWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.list
    }
}

impl SettingsListWidget {
    /// java: SettingsListWidget(Minecraft client, int width, int height, int y, int itemHeight)
    pub fn new(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft, mut width: i32, mut height: i32, mut y: i32, mut item_height: i32) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: mcrust::mcrust::modding::widgets::ModSelectionList::new(mcrust::mcrust::modding::widgets::ListKind::Container, width, height, y, item_height),
        });
        __this
    }

    /// java: void updateEntries(@Nullable Path path)
    pub fn update_entries(__this: &compat::Shared<Self>, mut path: Option<std::path::PathBuf>) {
        compat::write(__this, |__t| { let __l = &mut __t.base; __l.clear_entries() });
        // TODO(fabric2seer): `changedPaths` is a field of the enclosing instance (ConfigBackupScreen.this) - an inner class has no outer reference in Rust
        //   java: changedPaths = Collections.emptySet();
        let Some(mut path) = path.clone() else {
            return;
        };
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): static call JsonParser.parseReader(..)
            //   java: JsonElement backupJson = JsonParser.parseReader(Files.newBufferedReader(path));
            let mut backup_json: serde_json::Value = todo!("fabric2seer: the initialiser of `backupJson` (TODO above)");
            // TODO(fabric2seer): static call JsonParser.parseReader(..)
            //   java: JsonElement currentJson = JsonParser.parseReader(Files.newBufferedReader(SkyblockerConfigManager.getConfigPath()));
            let mut current_json: serde_json::Value = todo!("fabric2seer: the initialiser of `currentJson` (TODO above)");
            let mut diffs: std::collections::HashSet<String> = std::collections::HashSet::new();
            SettingsListWidget::find_diffs(__this, "".to_string(), &backup_json, &current_json, &mut diffs);
            // TODO(fabric2seer): `changedPaths` is a field of the enclosing instance (ConfigBackupScreen.this) - an inner class has no outer reference in Rust
            //   java: changedPaths = diffs;
            let mut lines: Vec<JsonLine> = Vec::new();
            SettingsListWidget::format_json(__this, &compat::jstring(&None), &backup_json, "", 0, true, &mut lines);
            for l in lines.iter() {
                let __entry = StringEntry::new(l.text.clone(), l.path.clone());
                compat::write(__this, |__t| { let __l = &mut __t.base; (__l.add_entry(Box::new(compat::EntryAdapter::new(__entry))) as i32) });
            }
        }));
        if let Err(e) = __result {
            logger().error(&format!("[Skyblocker] Error reading backup file for diff display"));
        }
    }

    /// java: @Override public int getRowWidth()
    pub fn get_row_width(__this: &compat::Shared<Self>) -> i32 {
        return ((__this.lock().borrow().get_width()).wrapping_sub(8)).wrapping_sub(20);
    }

    /// java: private void findDiffs(String pathPrefix, JsonElement backup, JsonElement current, Set<String> diffs)
    pub fn find_diffs(__this: &compat::Shared<Self>, mut path_prefix: String, backup: &serde_json::Value, current: &serde_json::Value, diffs: &mut std::collections::HashSet<String>) {
        if false /* java: backup == null: never null here */ {
            return;
        }
        // TODO(fabric2seer): the condition: JsonElement.equals
        //   java: !backup.equals(current)
        if todo!("fabric2seer: the condition (TODO above)") {
            if compat::json::is_object(&backup) && true /* java: current != null: never null here */ && compat::json::is_object(&current) {
                for entry in compat::json::get_as_json_object(&backup).iter().map(|(k, v)| (k.clone(), v.clone())) {
                    let mut next: String = (if path_prefix.is_empty() { entry.0.clone() } else { format!("{}.{}", path_prefix, entry.0) });
                    SettingsListWidget::find_diffs(__this, next.clone(), &entry.1, &compat::json::get_as_json_object(&current).get(&entry.0).cloned().unwrap(), diffs);
                }
            } else if compat::json::is_array(&backup) && true /* java: current != null: never null here */ && compat::json::is_array(&current) {
                for i in 0..(compat::json::get_as_json_array(&backup).len() as i32) {
                    let mut next: String = format!("{}[{}]", path_prefix, i);
                    let mut curr: serde_json::Value = (if i < (compat::json::get_as_json_array(&current).len() as i32) { Some(compat::json::get_as_json_array(&current)[(i) as usize].clone()) } else { None }).unwrap();
                    SettingsListWidget::find_diffs(__this, next.clone(), &compat::json::get_as_json_array(&backup)[(i) as usize], &curr, diffs);
                }
            } else {
                diffs.insert(path_prefix.clone());
            }
        }
    }

    /// java: private void formatJson(String key, JsonElement element, String path, int indent, boolean last, List<JsonLine> out)
    pub fn format_json(__this: &compat::Shared<Self>, key: &str, element: &serde_json::Value, path: &str, mut indent: i32, mut last: bool, out: &mut Vec<JsonLine>) {
        let mut ind: String = "  ".repeat((indent) as usize);
        let mut new_path: String = (if false /* java: key == null: never null here */ { path.to_string() } else { (if path.is_empty() { key.to_string() } else { format!("{}.{}", path, key) }) });
        if compat::json::is_object(&element) {
            let mut line: String = (if false /* java: key == null: never null here */ { format!("{}{{", ind) } else { format!("{}\"{}\": {{", ind, key) });
            out.push(JsonLine::new(line.clone(), None));
            let mut it = compat::json::get_as_json_object(&element).iter().map(|(k, v)| (k.clone(), v.clone()));
            // TODO(fabric2seer): the condition: Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
            //   java: it.hasNext(..)
            while todo!("fabric2seer: the condition (TODO above)") {
                let mut e = it.next().unwrap();
                // TODO(fabric2seer): Iterator.hasNext - use a `for` loop / `while let Some(x) = it.next()`
                //   java: formatJson(e.getKey(), e.getValue(), newPath, indent + 1, !it.hasNext(), out);
            }
            out.push(JsonLine::new(format!("{}}}{}", ind, (if last { "" } else { "," })), (if path.is_empty() { None } else { Some(path.to_string()) })));
        } else if compat::json::is_array(&element) {
            let mut line: String = (if false /* java: key == null: never null here */ { format!("{}[", ind) } else { format!("{}\"{}\": [", ind, key) });
            out.push(JsonLine::new(line.clone(), None));
            let mut arr = compat::json::get_as_json_array(&element);
            for i in 0..(arr.len() as i32) {
                SettingsListWidget::format_json(__this, &compat::jstring(&None), &arr[(i) as usize], &format!("{}[{}]", new_path, i), indent.wrapping_add(1), (i == (arr.len() as i32).wrapping_sub(1)), out);
            }
            out.push(JsonLine::new(format!("{}]{}", ind, (if last { "" } else { "," })), Some(path.to_string())));
        } else {
            let mut value: String = compat::json::to_string(&element);
            let mut line: String = (if false /* java: key == null: never null here */ { format!("{}{}", ind, value) } else { format!("{}\"{}\": {}", ind, key, value) });
            out.push(JsonLine::new(format!("{}{}", line, (if last { "" } else { "," })), Some(new_path.clone())));
        }
    }

    /// java: @Override protected void extractScrollbar(GuiGraphicsExtractor context, int mouseX, int mouseY)
    pub fn extract_scrollbar(__this: &compat::Shared<Self>, context: &mut GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32) {
        // TODO(fabric2seer): `ContainerObjectSelectionList.extractScrollbar` has no Rust method in the port (looked for `extract_scrollbar` in ContainerObjectSelectionList and its supertypes)
        // rust: __this.lock().borrow_mut().base.list.extract_scrollbar(context, mouse_x, mouse_y);
        if __this.lock().borrow().scrollable() {
            let mut scroll_bar_x: i32 = __this.lock().borrow().scroll_bar_x();
            let mut list_widget_y: i32 = __this.lock().borrow().get_y();
            let mut total_height: i32 = (compat::read(__this, |__t| __t.height)).wrapping_add(__this.lock().borrow().max_scroll_amount());
            let mut scrollbar_thumb_height: i32 = __this.lock().borrow().scroller_height();
            for i in 0..((compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<StringEntry>(__l) })).len() as i32) {
                let mut entry: compat::Shared<StringEntry> = (compat::write(__this, |__t| { let __l = &mut __t.base; compat::list_entries::<StringEntry>(__l) }))[(i) as usize].clone();
                // TODO(fabric2seer): the condition: `changedPaths` is a field of the enclosing instance (ConfigBackupScreen.this) - an inner class has no outer reference in Rust
                //   java: entry.path != null && changedPaths.contains(entry.path)
                if todo!("fabric2seer: the condition (TODO above)") {
                    let mut entry_y: i32 = (4i32.wrapping_add(i.wrapping_mul(compat::read(__this, |__t| __t.default_entry_height)))).wrapping_add(__this.lock().borrow().get_y());
                    let mut bar_y: i32 = (((entry_y.wrapping_mul(((compat::read(__this, |__t| __t.height)).wrapping_sub(scrollbar_thumb_height)).wrapping_sub(2))).wrapping_div(total_height.wrapping_sub(compat::read(__this, |__t| __t.default_entry_height)))).wrapping_add(list_widget_y)).wrapping_add(scrollbar_thumb_height.wrapping_div(2));
                    context.fill(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), scroll_bar_x, bar_y, scroll_bar_x.wrapping_add(6), bar_y.wrapping_add(2), 0xFFFFFF55u32 as i32);
                }
            }
        }
    }

}

/// SettingsListWidget's list (the port's `ModSelectionList`) until the game takes it (`compat::attach_list`)
impl compat::ListHalf for SettingsListWidget {
    fn list_half(&mut self) -> &mut ModSelectionList {
        &mut self.base
    }
}

/// SettingsListWidget's overrides of the list's own methods, asked by the port's list bodies
impl ModList for compat::ListClass<SettingsListWidget> {
    /// java: @Override public int getRowWidth()
    fn get_row_width(&self, list: &AbstractSelectionList, super_answer: i32) -> i32 {
        self.ask(list, super_answer, |__h| SettingsListWidget::get_row_width(__h))
    }

}

// NOTE(fabric2seer): extractScrollbar, getRowWidth override(s) of ContainerObjectSelectionList - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record JsonLine(String text, @Nullable String path)
#[derive(Clone, Debug, PartialEq)]
pub struct JsonLine {
    /// java: String text
    pub text: String,
    /// java: String path
    pub path: Option<String>,
}

impl JsonLine {
    /// java: the canonical record constructor
    pub fn new(mut text: String, mut path: Option<String>) -> Self {
        Self { text, path }
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> String {
        self.text.clone()
    }

    /// java: the record accessor `path()`
    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }

}

/// java: private class StringEntry extends ContainerObjectSelectionList.Entry<StringEntry>
pub struct StringEntry {
    /// java: extends net.minecraft.client.gui.components.ContainerObjectSelectionList$Entry
    pub base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry,
    /// java: private final String text;
    pub text: String,
    /// java: private final @Nullable String path;
    pub path: Option<String>,
    /// the row's widgets (`children()`): lent to the object by its `compat::EntryAdapter`
    pub __row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for StringEntry {
    type Target = mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for StringEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl StringEntry {
    /// java: StringEntry(String text, @Nullable String path)
    pub fn new(mut text: String, mut path: Option<String>) -> compat::Shared<Self> {
        let __init_text = text.clone();
        let __init_path = path.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry::new(0),
            __widgets: Default::default(),
            __row: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()),
            text: __init_text,
            path: __init_path,
        });
        __this
    }

    /// java: @Override public List<GuiEventListener> children()
    pub fn children(__this: &compat::Shared<Self>) -> Vec<Box<dyn GuiEventListener>> {
        return vec![];
    }

    /// java: @Override public List<NarratableEntry> narratables()
    pub fn narratables(__this: &compat::Shared<Self>) -> Vec<Box<dyn NarratableEntry>> {
        return vec![];
    }

    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    pub fn extract_content(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut hovered: bool, mut delta_ticks: f32) {
        let mut color: i32 = 0xFFFFFFFFu32 as i32;
        // TODO(fabric2seer): the condition: `changedPaths` is a field of the enclosing instance (ConfigBackupScreen.this) - an inner class has no outer reference in Rust
        //   java: path != null && changedPaths.contains(path)
        if todo!("fabric2seer: the condition (TODO above)") {
            color = 0xFFFFFF55u32 as i32;
        }
        // TODO(fabric2seer): unknown name `font`
        //   java: graphics.text(font, text, this.getX() + 2, this.getY() + 2, color, false);
    }

}

/// StringEntry's row box and its widgets, lent by its `compat::EntryAdapter` for each call
impl compat::RowHalf for StringEntry {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry {
        &mut self.base
    }
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut compat::WidgetCache) {
        let __s = self;
        (&mut __s.__row, &mut __s.__widgets)
    }
}

/// StringEntry as a row of the game's list (`addEntry` attaches it)
impl ModListEntry for compat::EntryAdapter<StringEntry> {
    /// java: @Override public void extractContent(GuiGraphicsExtractor graphics, int mouseX, int mouseY, boolean hovered, float deltaTicks)
    fn extract_content(&mut self, list: &mut ModSelectionList, index: usize, context: &mut WidgetRenderContext<'_>, mouse_x: i32, mouse_y: i32, hovered: bool, a: f32) {
        self.lend(Some(list.list.entry(index)), |__h| StringEntry::extract_content(__h, &context.graphics, mouse_x, mouse_y, hovered, a));
        // the row's widgets (Java's `child.extractRenderState(..)` over `children()`)
        context.extract_widgets(&mut self.row, mouse_x, mouse_y);
    }

    fn children(&mut self) -> Option<&mut mcrust::net::minecraft::client::gui::screens::screen::Screen> {
        Some(&mut self.row)
    }

}

// NOTE(fabric2seer): children, extractContent, narratables override(s) of Entry - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
