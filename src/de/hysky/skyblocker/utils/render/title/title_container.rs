//! Ported from de.hysky.skyblocker.utils.render.title.TitleContainer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Alignment;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Direction;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::title::title::Title;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use mcrust::net::minecraft::client::gui::font::Font;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::entity::entity::Entity as _;

/// java: private static final Identifier TITLE_CONTAINER = SkyblockerMod.id("title_container");
pub static TITLE_CONTAINER: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("title_container"));
/// java: protected static final float RENDER_SCALE = 0.03f;
pub const RENDER_SCALE: f32 = 0.03f32;
/// java: private static final Set<Title> titles = new LinkedHashSet<>();
///
/// `Title` keeps `Object`'s equals/hashCode, so the set is an identity set (`Arc::ptr_eq`); a `LinkedHashSet`
/// iterates in insertion order, which a `Vec` with a membership test keeps.
pub static TITLES: compat::MainThread<Vec<compat::Shared<Title>>> = compat::MainThread::new(|| Vec::new());

/// java: `titles.contains(title)` on the identity set
fn titles_contain(titles: &[compat::Shared<Title>], title: &compat::Shared<Title>) -> bool {
    titles.iter().any(|t| std::sync::Arc::ptr_eq(t, title))
}

/// java: public class TitleContainer
#[derive(Clone)]
pub struct TitleContainer {}

impl TitleContainer {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        hud_render_callback::EVENT.register(Arc::new(|ctx: &mut hud_render_callback::HudRenderContext<'_>| {
            // SAFETY: `ctx` outlives every call made through `graphics` (this callback).
            let graphics = unsafe { ScriptGraphics::new(ctx.gl, &*ctx.minecraft, ctx.gui_width, ctx.gui_height) };
            let _session = graphics.session();
            // java: TitleContainer::render, the HUD element - the port's GuiGraphicsExtractor is stateless; the script
            // graphics session above scopes the pose stack for the duration of the element
            TitleContainer::render_5(&GuiGraphicsExtractor::new(), TITLES.get_mut(), SkyblockerConfigManager::get().ui_and_visuals.title_container.x, SkyblockerConfigManager::get().ui_and_visuals.title_container.y, ctx.partial_tick);
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // java: dispatcher.register(ClientCommands.literal("skyblocker").then(ClientCommands.literal("hud")
                //   .then(ClientCommands.literal("titleContainer").executes(Scheduler.queueOpenScreenCommand(TitleContainerConfigScreen::new)))))
                use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
                let mut leaf = client_command_registration_callback::literal("titleContainer");
                leaf.executes(crate::de::hysky::skyblocker::utils::scheduler::scheduler::SchedulerBase::queue_open_screen_command(Arc::new(|| {
                    Box::new(compat::ScreenAdapter::new(crate::de::hysky::skyblocker::utils::render::title::title_container_config_screen::TitleContainerConfigScreen::new())) as Box<dyn mcrust::mcrust::modding::screens::ModScreen>
                })));
                let mut hud = client_command_registration_callback::literal("hud");
                hud.then(&leaf);
                let mut root = client_command_registration_callback::literal("skyblocker");
                root.then(&hud);
                dispatcher.dispatcher().register(&root);
            },
        ));
    }

    /// java: public static boolean containsTitle(Title title)
    pub fn contains_title(title: &compat::Shared<Title>) -> bool {
        return titles_contain(TITLES.get_mut(), title);
    }

    /// java: public static boolean addTitle(Title title)
    pub fn add_title(mut title: compat::Shared<Title>) -> bool {
        // java: if (titles.add(title)) - adds only when absent
        if !titles_contain(TITLES.get_mut(), &title) {
            TITLES.get_mut().push(title.clone());
            Title::reset_pos(&title);
            return true;
        }
        return false;
    }

    /// java: public static boolean addTitle(Title title, int ticks)
    pub fn add_title_2(title: &compat::Shared<Title>, mut ticks: i32) -> bool {
        if TitleContainer::add_title(title.clone()) {
            ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule()); let __title = title.clone(); __f(__r, Arc::new(move || { TitleContainer::remove_title(&__title); }), ticks) });
            return true;
        }
        return false;
    }

    /// java: public static boolean addTitleAndPlaySound(Title title)
    pub fn add_title_and_play_sound(title: &compat::Shared<Title>) -> bool {
        if TitleContainer::add_title(title.clone()) {
            TitleContainer::play_notification_sound();
            return true;
        }
        return false;
    }

    /// java: public static boolean addTitleAndPlaySound(Title title, int ticks)
    pub fn add_title_and_play_sound_2(title: &compat::Shared<Title>, mut ticks: i32) -> bool {
        if TitleContainer::add_title_2(title, ticks) {
            TitleContainer::play_notification_sound();
            return true;
        }
        return false;
    }

    /// java: public static void playNotificationSound()
    pub fn play_notification_sound() {
        if mcrust::mcrust::modding::game::with_player(|_| ()).is_some() {
            mcrust::mcrust::modding::game::with_player(|player| { player.play_sound(mcrust::net::minecraft::sounds::sound_events::experience_orb_pickup().clone(), 100.0f32, 0.1f32); });
        }
    }

    /// java: public static void removeTitle(Title title)
    pub fn remove_title(title: &compat::Shared<Title>) {
        // java: titles.remove(title);
        TITLES.get_mut().retain(|t| !std::sync::Arc::ptr_eq(t, title));
    }

    /// java: private static void render(GuiGraphicsExtractor graphics, DeltaTracker tickCounter)
    ///
    /// (`DeltaTracker` is the client's `DeltaTracker.Timer`, the one the HUD is handed)
    pub fn render(graphics: &GuiGraphicsExtractor, tick_counter: &mcrust::net::minecraft::client::delta_tracker::Timer) {
        TitleContainer::render_5(graphics, TITLES.get_mut(), SkyblockerConfigManager::get().ui_and_visuals.title_container.x, SkyblockerConfigManager::get().ui_and_visuals.title_container.y, tick_counter.get_game_time_delta_partial_tick(true));
    }

    /// java: protected static void render(GuiGraphicsExtractor graphics, Set<Title> titles, int xPos, int yPos, float tickDelta)
    pub fn render_5(graphics: &GuiGraphicsExtractor, titles: &[compat::Shared<Title>], mut x_pos: i32, mut y_pos: i32, mut tick_delta: f32) {
        let config = &SkyblockerConfigManager::get().ui_and_visuals.title_container;
        let mut scale: f32 = (config.title_container_scale * RENDER_SCALE);
        let mut direction: Direction = config.direction;
        let mut alignment: Alignment = config.alignment;
        TitleContainer::render_8(graphics, titles, x_pos, y_pos, tick_delta, scale, direction, alignment);
    }

    /// java: protected static void render(GuiGraphicsExtractor graphics, Set<Title> titles, int xPos, int yPos, float tickDelta, float scale, UIAndVisualsConfig.Direction direction, UIAndVisualsConfig.Alignment al
    pub fn render_8(graphics: &GuiGraphicsExtractor, titles: &[compat::Shared<Title>], mut x_pos: i32, mut y_pos: i32, mut tick_delta: f32, mut scale: f32, mut direction: Direction, mut alignment: Alignment) {
        if titles.is_empty() {
            return;
        }
        let mut text_renderer: Font = compat::font();
        let mut x: f32 = (if x_pos >= 0 { (x_pos) as f32 } else { ((compat::minecraft().get_window().get_gui_scaled_width() as f32) / 2.0f32) });
        let mut y: f32 = (if y_pos >= 0 { (y_pos) as f32 } else { ((compat::minecraft().get_window().get_gui_scaled_height() as f32) * 0.6f32) });
        let mut total_width: f32 = ((TitleContainer::get_width(&text_renderer, direction, scale, titles)) as f32) as f32;
        if alignment == Alignment::Middle && direction == Direction::Horizontal {
            x = ((x_pos as f32) - (total_width / (2 as f32)));
        }
        for title in titles.iter() {
            let mut x_text_left: f32 = x;
            if alignment == Alignment::Right {
                x_text_left = (x - ((text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Title::get_text(title))) as f32) * scale));
            } else if direction == Direction::Vertical && alignment == Alignment::Middle {
                x_text_left = (x - (((text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Title::get_text(title))) as f32) * scale) / (2 as f32)));
            }
            if Title::is_default_pos(title) {
                { let __v = x_text_left; compat::write(title, |__t| __t.x = __v); }
                { let __v = y; compat::write(title, |__t| __t.y = __v); }
            }
            { let __v = mth::lerp_f32((tick_delta * 0.5f32), compat::read(title, |__t| __t.x), x_text_left); compat::write(title, |__t| __t.x = __v); }
            { let __v = mth::lerp_f32((tick_delta * 0.5f32), compat::read(title, |__t| __t.y), y); compat::write(title, |__t| __t.y = __v); }
            let __pose_1 = gui_graphics_extractor::push_pose();
            gui_graphics_extractor::pose_translate((compat::read(title, |__t| __t.x)) as f32, (compat::read(title, |__t| __t.y)) as f32);
            gui_graphics_extractor::pose_scale((scale) as f32, (scale) as f32);
            gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Title::get_text(title)), 0, 0, mcrust::net::minecraft::util::common_colors::WHITE);
            gui_graphics_extractor::pop_pose(__pose_1);
            if direction == Direction::Horizontal {
                if alignment == Alignment::Middle || alignment == Alignment::Left {
                    x = (x + (((text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Title::get_text(title)))).wrapping_add(10) as f32) * scale));
                } else if alignment == Alignment::Right {
                    x = (x - (((text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(Title::get_text(title)))).wrapping_add(10) as f32) * scale));
                }
            } else {
                y = (y + (((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1) as f32) * scale));
            }
        }
    }

    /// java: protected static int getWidth(Font textRenderer, UIAndVisualsConfig.Direction direction, float scale, Set<Title> titles)
    pub fn get_width(text_renderer: &Font, mut direction: Direction, mut scale: f32, titles: &[compat::Shared<Title>]) -> i32 {
        return (if direction == Direction::Horizontal { ((((titles.iter().cloned().map(|__a0| Title::get_text(&__a0)).map(|__a0| text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__a0.clone()))).map(|width| (width.wrapping_add(10)) as f64).sum::<f64>() - (10 as f64)) * (scale as f64))) as i32) } else { ((((titles.iter().cloned().map(|__a0| Title::get_text(&__a0)).map(|__a0| text_renderer.width_of_cached(&mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(__a0.clone()))).max().unwrap_or(0) as f32) * scale)) as i32) });
    }

    /// java: protected static int getHeight(Font textRenderer, UIAndVisualsConfig.Direction direction, float scale, Set<Title> titles)
    pub fn get_height(text_renderer: &Font, mut direction: Direction, mut scale: f32, titles: &[compat::Shared<Title>]) -> i32 {
        return (if direction == Direction::Horizontal { ((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT as f32) * scale)) as i32) } else { ((((((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1)).wrapping_mul(titles.len() as i32) as f32) * scale)) as i32) });
    }

}
