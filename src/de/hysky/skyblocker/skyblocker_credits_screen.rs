//! Ported from de.hysky.skyblocker.SkyblockerCreditsScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::entry::skyblocker_mod;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::util::uuid::Uuid;
use mcrust::mcrust::modding::screens::ModScreen;
use mcrust::mcrust::modding::screens::ScreenRenderContext;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
use mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor;
use mcrust::net::minecraft::client::input::character_event::CharacterEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::client::input::mouse_button_info::MouseButtonInfo;
use mcrust::net::minecraft::network::chat::common_components;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::contents::objects::player_sprite::PlayerSprite;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::sounds::music::Music;
use mcrust::net::minecraft::util::formatted_char_sequence::FormattedCharSequence;
use mcrust::net::minecraft::util::formatted_char_sequence::Seq;
use mcrust::net::minecraft::world::item::component::resolvable_profile::ResolvableProfile;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::server::packs::resources::resource_provider::ResourceProvider as _;

/// java: private static final Component TITLE = Component.literal("Skyblocker Credits");
pub static TITLE: compat::MainThread<MutableComponent> = compat::MainThread::new(|| component::literal("Skyblocker Credits"));
/// java: private static final Identifier CREDITS_FILE = SkyblockerMod.id("credits.json");
pub static CREDITS_FILE: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("credits.json"));
/// java: private static final Identifier LOGO = SkyblockerMod.id("logo.png");
pub static LOGO: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::skyblocker_mod::id("logo.png"));
/// java: private static final Identifier VIGNETTE = Identifier.withDefaultNamespace("textures/misc/credits_vignette.png");
pub static VIGNETTE: compat::MainThread<Identifier> = compat::MainThread::new(|| Identifier::with_default_namespace("textures/misc/credits_vignette.png"));
/// java: private static final Music SKYBLOCKER_CREDITS_MUSIC = new Music(SoundEvents.MUSIC_DISC_CREATOR, 0, 0, true);
pub static SKYBLOCKER_CREDITS_MUSIC: compat::MainThread<Music> = compat::MainThread::new(|| Music::new(mcrust::net::minecraft::sounds::sound_events::music_disc_creator(), 0, 0, true));
/// java: private static final Component SECTION_HEADING = Component.literal("================").withStyle(ChatFormatting.WHITE);
pub static SECTION_HEADING: compat::MainThread<MutableComponent> = compat::MainThread::new(|| (component::literal("================")).with_style_format(ChatFormatting::White));
/// java: private static final float SPEEDUP_FACTOR = 5f;
pub const SPEEDUP_FACTOR: f32 = 5.0f32;
/// java: private static final float SPEEDUP_FACTOR_FAST = 15f;
pub const SPEEDUP_FACTOR_FAST: f32 = 15.0f32;
/// java: private static final float UNMODIFIED_SCROLL_SPEED = 0.75f;
pub const UNMODIFIED_SCROLL_SPEED: f32 = 0.75f32;
// TODO(fabric2seer): `PrimitiveCodec.listOf` has no Rust method in the port (looked for `list_of` in PrimitiveCodec and its supertypes)
//   java: private static final Codec<Credits> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Author.CODEC.listOf().fieldOf("authors").forGetter(Credits::authors),
//   java: 				Codec.STRING.listOf().fieldOf("creativeTeam").forGetter(Credits::creativeTeam),
//   java: 				Translation.CODEC.listOf().fieldOf("translations").forGetter(Credits::translations),
//   java: 				Library.CODEC.listOf().fieldOf("libraries").forGetter(Credits::libraries)
//   java: 				).apply(instance, Credits::new));
/// java: private static final Codec<Credits> CODEC = RecordCodecBuilder.create(instance -> instance.group( Author.CODEC.listOf().fieldOf("authors").forGetter(Credits::authors), Codec.STRING.listOf().fieldOf("c
pub static CREDITS_CODEC: compat::MainThread<CodecRef<Credits>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: private static final Credits EMPTY = new Credits(List.of(new Author("Error", UUID.fromString("647ffd4c-f99c-4b06-a8f8-66cf1a587e57"), CommonColors.RED)), List.of(), List.of(), List.of());
pub static CREDITS_EMPTY: compat::MainThread<Credits> = compat::MainThread::new(|| Credits::new(vec![Author::new("Error".to_string(), Uuid::from_string("647ffd4c-f99c-4b06-a8f8-66cf1a587e57").unwrap(), mcrust::net::minecraft::util::common_colors::RED)], vec![], vec![], vec![]));
/// java: private static final Codec<Author> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Author::name), UUIDUtil.STRING_CODEC.fieldOf("uuid").forGetter(A
pub static AUTHOR_CODEC: compat::MainThread<CodecRef<Author>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply3(record_codec_builder::of(|o: &Author| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Author| o.uuid(), codec::field_of(mcrust::net::minecraft::core::uuid_util::string_codec(), "uuid")), record_codec_builder::of(|o: &Author| o.colour(), codec::optional_field_of(mcrust::net::minecraft::util::extra_codecs::string_rgb_color(), "colour", mcrust::net::minecraft::util::common_colors::WHITE)), move |__a0, __a1, __a2| Author::new(__a0.clone(), __a1.clone(), __a2.clone()))));
// TODO(fabric2seer): `.compareToIgnoreCase(..)` on a E
//   java: private static final Codec<Translation> CODEC = RecordCodecBuilder.create(instance -> instance.group(
//   java: 				Codec.STRING.fieldOf("code").forGetter(Translation::code),
//   java: 				Codec.STRING.fieldOf("name").forGetter(Translation::name),
//   java: 				CodecUtils.sortedListCodec(Codec.STRING, String::compareToIgnoreCase).fieldOf("translators").forGetter(Translation::translators)
//   java: 				).apply(instance, Translation::new));
/// java: private static final Codec<Translation> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("code").forGetter(Translation::code), Codec.STRING.fieldOf("name").forGetter(
pub static TRANSLATION_CODEC: compat::MainThread<CodecRef<Translation>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));
/// java: private static final Codec<Library> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.STRING.fieldOf("name").forGetter(Library::name), Codec.STRING.fieldOf("credit").forGetter(Librar
pub static LIBRARY_CODEC: compat::MainThread<CodecRef<Library>> = compat::MainThread::new(|| record_codec_builder::create(record_codec_builder::apply2(record_codec_builder::of(|o: &Library| o.name(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "name")), record_codec_builder::of(|o: &Library| o.credit(), codec::field_of(mcrust::com::mojang::serialization::codec::string_codec(), "credit")), move |__a0, __a1| Library::new(__a0.clone(), __a1.clone()))));

/// java: public class SkyblockerCreditsScreen extends Screen
pub struct SkyblockerCreditsScreen {
    /// java: extends net.minecraft.client.gui.screens.Screen
    pub base: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    /// java: private float scroll;
    pub scroll: f32,
    /// java: private final List<FormattedCharSequence> lines = new ArrayList<>();
    pub lines: Vec<Seq>,
    /// java: private final List<Component> narratorComponents = new ArrayList<>();
    pub narrator_components: Vec<MutableComponent>,
    /// java: private final IntSet centredLines = new IntOpenHashSet();
    pub centred_lines: std::collections::HashSet<i32>,
    /// java: private int totalScrollLength;
    pub total_scroll_length: i32,
    /// java: private boolean speedupActive;
    pub speedup_active: bool,
    /// java: private final IntSet speedupModifiers = new IntOpenHashSet();
    pub speedup_modifiers: std::collections::HashSet<i32>,
    /// java: private float scrollSpeed;
    pub scroll_speed: f32,
    /// java: private int direction;
    pub direction: i32,
    /// java: private final @Nullable Screen parent;
    pub parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    /// the screen's widgets as this object last saw them (see `compat::WidgetCache`)
    pub __widgets: compat::WidgetCache,
}

impl std::ops::Deref for SkyblockerCreditsScreen {
    type Target = mcrust::net::minecraft::client::gui::screens::screen::Screen;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SkyblockerCreditsScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SkyblockerCreditsScreen {
    /// java: protected SkyblockerCreditsScreen(@Nullable Screen parent)
    pub fn new(mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> compat::Shared<Self> {
        let __init_direction = 1;
        let __init_scroll_speed = UNMODIFIED_SCROLL_SPEED;
        let __init_parent = parent.clone();
        let __this = compat::shared(Self {
            base: mcrust::net::minecraft::client::gui::screens::screen::Screen::new(TITLE.get_mut().get_string()),
            __widgets: Default::default(),
            scroll: 0.0,
            lines: Vec::new(),
            narrator_components: Vec::new(),
            centred_lines: std::collections::HashSet::new(),
            total_scroll_length: 0,
            speedup_active: false,
            speedup_modifiers: std::collections::HashSet::new(),
            scroll_speed: __init_scroll_speed,
            direction: __init_direction,
            parent: __init_parent,
        });
        __this
    }

    /// java: @Init public static void initClass()
    pub fn init_class() {
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private float calculateScrollSpeed()
    pub fn calculate_scroll_speed(__this: &compat::Shared<Self>) -> f32 {
        return (if compat::read(__this, |__t| __t.speedup_active) { ((UNMODIFIED_SCROLL_SPEED * (SPEEDUP_FACTOR + (((__this.lock().borrow().speedup_modifiers.len() as i32) as f32) * SPEEDUP_FACTOR_FAST))) * (compat::read(__this, |__t| __t.direction) as f32)) } else { (UNMODIFIED_SCROLL_SPEED * (compat::read(__this, |__t| __t.direction) as f32)) });
    }

    /// java: @Override public void tick()
    pub fn tick(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `tick` takes 3 argument(s) in the port (`ctx: &mut MusicContext<'_>, music_volume: f32, level_loading_screen: bool`), Java passes 0
        // rust: compat::minecraft().get_music_manager().tick();
        // TODO(fabric2seer): `tick` takes 4 argument(s) in the port (`_al: &OpenAl, library: &mut Library, options: &OptionsData, paused: bool`), Java passes 1
        // rust: compat::minecraft().get_sound_manager().tick(&false);
        let mut max_scroll: f32 = (((((compat::read(__this, |__t| __t.total_scroll_length)).wrapping_add(compat::read(__this, |__t| __t.height))).wrapping_add(compat::read(__this, |__t| __t.height))).wrapping_add(24)) as f32) as f32;
        if compat::read(__this, |__t| __t.scroll) > max_scroll {
            SkyblockerCreditsScreen::on_close(__this);
        }
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    pub fn key_pressed(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if key_event.is_up() {
            { let __v = -1; compat::write(__this, |__t| __t.direction = __v); }
        } else if key_event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_LCONTROL || key_event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RCONTROL {
            let __h0 = key_event.key();
            __this.lock().borrow_mut().speedup_modifiers.insert(__h0);
        } else if key_event.key() == 32 {
            { let __v = true; compat::write(__this, |__t| __t.speedup_active = __v); }
        }
        { let __v = SkyblockerCreditsScreen::calculate_scroll_speed(__this); compat::write(__this, |__t| __t.scroll_speed = __v); }
        return (if key_event.is_escape() && true { SkyblockerCreditsScreen::on_close(__this); true } else { false });
    }

    /// java: @Override public boolean keyReleased(KeyEvent keyEvent)
    pub fn key_released(__this: &compat::Shared<Self>, key_event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if key_event.is_up() {
            { let __v = 1; compat::write(__this, |__t| __t.direction = __v); }
        }
        if key_event.key() == 32 {
            { let __v = false; compat::write(__this, |__t| __t.speedup_active = __v); }
        } else if key_event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_LCONTROL || key_event.key() == mcrust::com::mojang::blaze3d::platform::input_constants::KEY_RCONTROL {
            let __h0 = &key_event.key();
            __this.lock().borrow_mut().speedup_modifiers.remove(__h0);
        }
        { let __v = SkyblockerCreditsScreen::calculate_scroll_speed(__this); compat::write(__this, |__t| __t.scroll_speed = __v); }
        return false;
    }

    /// java: @Override public void init()
    pub fn init(__this: &compat::Shared<Self>) {
        if __this.lock().borrow().lines.is_empty() {
            let mut credits: Credits = SkyblockerCreditsScreen::read_credits_file(__this);
            // TODO(fabric2seer): `ModMetadata.getContributors` (the loader's ModMetadata)
            //   java: List<String> contributors = SkyblockerMod.SKYBLOCKER_MOD.getMetadata().getContributors().stream()
            //   java: 					.map(Person::getName)
            //   java: 					.toList();
            let mut contributors: Vec<String> = todo!("fabric2seer: the initialiser of `contributors` (TODO above)");
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("Authors").with_style_format(ChatFormatting::Yellow), true, true);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            for author in credits.authors() {
                SkyblockerCreditsScreen::add_credits_line(__this, author.as_text(), false, true);
                SkyblockerCreditsScreen::add_empty_line(__this);
            }
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("Creative Team").with_style_format(ChatFormatting::Yellow), true, true);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            for member in credits.creative_team() {
                SkyblockerCreditsScreen::add_credits_line(__this, component::literal(&member).with_style_format(ChatFormatting::White), false, true);
                SkyblockerCreditsScreen::add_empty_line(__this);
            }
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("Code Contributors").with_style_format(ChatFormatting::Yellow), true, true);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            for contributor in contributors.iter() {
                SkyblockerCreditsScreen::add_credits_line(__this, component::literal(contributor).with_style_format(ChatFormatting::White), false, true);
            }
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("and any other code contributors not listed").with_style_format(ChatFormatting::White), false, true);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("Translations").with_style_format(ChatFormatting::Yellow), true, true);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            for translation in credits.translations() {
                SkyblockerCreditsScreen::add_credits_line(__this, component::literal(&translation.name()).with_style_format(ChatFormatting::Gray), false, true);
                for translator in translation.translators() {
                    let mut text: MutableComponent = component::empty().append_str("           ").append(component::literal(&translator).with_style_format(ChatFormatting::White));
                    SkyblockerCreditsScreen::add_credits_line(__this, text.clone(), false, true);
                }
                SkyblockerCreditsScreen::add_empty_line(__this);
            }
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_credits_line(__this, component::literal("Libraries").with_style_format(ChatFormatting::Yellow), true, true);
            SkyblockerCreditsScreen::add_credits_line(__this, SECTION_HEADING.get_mut().clone(), true, false);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            for library in credits.libraries() {
                let mut library_credit_text: MutableComponent = component::empty().append_str("           ").append(component::literal(&library.credit()).with_style_format(ChatFormatting::White));
                SkyblockerCreditsScreen::add_credits_line(__this, component::literal(&library.name()).with_style_format(ChatFormatting::Gray), false, true);
                SkyblockerCreditsScreen::add_credits_line(__this, library_credit_text.clone(), false, true);
                SkyblockerCreditsScreen::add_empty_line(__this);
            }
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_empty_line(__this);
            SkyblockerCreditsScreen::add_credits_line(__this, (component::literal("Special thanks to all of our users!")).with_style_format(ChatFormatting::Yellow), true, true);
            { let __v = (__this.lock().borrow().lines.len() as i32).wrapping_mul(12); compat::write(__this, |__t| __t.total_scroll_length = __v); }
        }
    }

    /// java: @Override public Component getNarrationMessage()
    pub fn get_narration_message(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::component_of(common_components::join_for_narration(&compat::read(__this, |__t| __t.narrator_components.clone())));
    }

    /// java: private Credits readCreditsFile()
    pub fn read_credits_file(__this: &compat::Shared<Self>) -> Credits {
        {
            let mut reader: crate::unresolved::BufferedReader = compat::minecraft().get_resource_manager().open_as_reader(CREDITS_FILE.get_mut()).unwrap();
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): static call JsonParser.parseReader(..)
                //   java: return Credits.CODEC.parse(JsonOps.INSTANCE, JsonParser.parseReader(reader)).getOrThrow();
            })) {
                Ok(__v) => return __v,
                Err(e) => {
                    logger().error(&format!("[Skyblocker Credits] Failed to load credits file."));
                }
            }
        }
        return CREDITS_EMPTY.get_mut().clone();
    }

    /// java: private void addEmptyLine()
    pub fn add_empty_line(__this: &compat::Shared<Self>) {
        let __h0 = mcrust::net::minecraft::util::formatted_char_sequence::empty();
        __this.lock().borrow_mut().lines.push(__h0);
        let __h0 = mcrust::net::minecraft::network::chat::common_components::empty();
        __this.lock().borrow_mut().narrator_components.push(__h0);
    }

    /// java: private void addCreditsLine(Component text, boolean centred, boolean narratable)
    pub fn add_credits_line(__this: &compat::Shared<Self>, mut text: MutableComponent, mut centred: bool, mut narratable: bool) {
        if centred {
            let __h0 = (__this.lock().borrow().lines.len() as i32);
            __this.lock().borrow_mut().centred_lines.insert(__h0);
        }
        let __h0 = text.get_visual_order_text();
        __this.lock().borrow_mut().lines.push(__h0);
        if narratable {
            let __h0 = text.clone();
            __this.lock().borrow_mut().narrator_components.push(__h0);
        }
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    pub fn extract_render_state(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut mouse_x: i32, mut mouse_y: i32, mut a: f32) {
        ();
        SkyblockerCreditsScreen::extract_vignette(__this, graphics);
        { let __v = (0.0f32).max((compat::read(__this, |__t| __t.scroll) + (a * compat::read(__this, |__t| __t.scroll_speed)))); compat::write(__this, |__t| __t.scroll = __v); }
        let mut logo_x: i32 = ((compat::read(__this, |__t| __t.width)).wrapping_div(2)).wrapping_sub(128);
        let mut logo_y: i32 = (compat::read(__this, |__t| __t.height)).wrapping_add(50);
        let mut y_offs: f32 = (-compat::read(__this, |__t| __t.scroll));
        let __pose_1 = gui_graphics_extractor::push_pose();
        gui_graphics_extractor::pose_translate((0.0f32) as f32, (y_offs) as f32);
        gui_graphics_extractor::next_stratum();
        SkyblockerCreditsScreen::extract_logo(__this, graphics, compat::read(__this, |__t| __t.width), logo_y);
        let mut y_pos: i32 = logo_y.wrapping_add(100);
        for i in 0..(__this.lock().borrow().lines.len() as i32) {
            if i == (__this.lock().borrow().lines.len() as i32).wrapping_sub(1) {
                let mut diff: f32 = (((y_pos as f32) + y_offs) - ((((compat::read(__this, |__t| __t.height)) as f32) / (2 as f32)) - (6 as f32)));
                if diff < 0.0f32 {
                    gui_graphics_extractor::pose_translate((0.0f32) as f32, ((-diff)) as f32);
                }
            }
            if ((((y_pos as f32) + y_offs) + 12.0f32) + 8.0f32) > 0.0f32 && ((y_pos as f32) + y_offs) < (compat::read(__this, |__t| __t.height) as f32) {
                let line = &__this.lock().borrow().lines[(i) as usize];
                if __this.lock().borrow().centred_lines.contains(&i) {
                    // TODO(fabric2seer): `GuiGraphicsExtractor.centeredText` has no Rust method in the port (looked for `centered_text` in GuiGraphicsExtractor and its supertypes)
                    // rust: graphics.centered_text(&compat::font(), line, logo_x.wrapping_add(128), y_pos, mcrust::net::minecraft::util::common_colors::WHITE);
                } else {
                    gui_graphics_extractor::draw_formatted_char_sequence(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), line, logo_x, y_pos, mcrust::net::minecraft::util::common_colors::WHITE);
                }
            }
            y_pos = y_pos.wrapping_add(12);
        }
        gui_graphics_extractor::pop_pose(__pose_1);
    }

    /// java: private void extractVignette(GuiGraphicsExtractor graphics)
    pub fn extract_vignette(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor) {
        gui_graphics_extractor::blit_identifier(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::vignette(), VIGNETTE.get_mut(), 0, 0, 0.0f32, 0.0f32, compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height), compat::read(__this, |__t| __t.width), compat::read(__this, |__t| __t.height));
    }

    /// java: private void extractLogo(GuiGraphicsExtractor graphics, int width, int heightOffset)
    pub fn extract_logo(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut width: i32, mut height_offset: i32) {
        let mut logo_x: i32 = (width.wrapping_div(2)).wrapping_sub(128);
        gui_graphics_extractor::blit_identifier_with_color(mcrust::com::mojang::blaze3d::opengl::gl_state_manager::gl(), &*compat::minecraft(), mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(), LOGO.get_mut(), logo_x, height_offset, 0.0f32, 0.0f32, 256, 64, 256, 64, mcrust::net::minecraft::util::common_colors::WHITE);
    }

    /// java: @Override protected void extractMenuBackground(GuiGraphicsExtractor graphics, int x, int y, int width, int height)
    pub fn extract_menu_background(__this: &compat::Shared<Self>, graphics: &GuiGraphicsExtractor, mut x: i32, mut y: i32, mut width: i32, mut height: i32) {
        let mut v: f32 = (compat::read(__this, |__t| __t.scroll) * 0.5f32);
        // TODO(fabric2seer): static `Screen.MENU_BACKGROUND` has no Rust counterpart in the port's Screen file
        // TODO(fabric2seer): static `Screen.extractMenuBackgroundTexture` has no Rust counterpart in the port
        // rust: Screen::extract_menu_background_texture(graphics.clone(), mcrust::net::minecraft::client::gui::screens::screen::Screen::MENU_BACKGROUND.clone(), 0, 0, 0.0f32, v, width, height);
    }

    /// java: @Override public void onClose()
    pub fn on_close(__this: &compat::Shared<Self>) {
        mcrust::mcrust::modding::game::set_screen_to_parent();
    }

    /// java: @Override public void removed()
    pub fn removed(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): the port's `stop_playing_music` ports this Java method with other parameters (ctx, music for Java's music)
        // TODO(fabric2seer): `stop_playing_music` takes 2 argument(s) in the port (`ctx: &mut MusicContext<'_>, music: &Music`), Java passes 1
        // rust: compat::minecraft().get_music_manager().stop_playing_music(SKYBLOCKER_CREDITS_MUSIC.get_mut());
    }

    /// java: @Override public Music getBackgroundMusic()
    pub fn get_background_music(__this: &compat::Shared<Self>) -> Music {
        return SKYBLOCKER_CREDITS_MUSIC.get_mut().clone();
    }

}

/// SkyblockerCreditsScreen's `Screen` half (`super`'s fields and widget tree), lent by the game for each call
impl compat::ScreenHalf for SkyblockerCreditsScreen {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen {
        &mut self.base
    }
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut compat::WidgetCache>) {
        let __s = self;
        (&mut __s.base, Some(&mut __s.__widgets))
    }
}

/// SkyblockerCreditsScreen as the game's screen: `game::set_screen(Box::new(compat::ScreenAdapter::new(..)))`
impl ModScreen for compat::ScreenAdapter<SkyblockerCreditsScreen> {
    fn get_title(&self) -> String {
        self.title()
    }

    /// java: @Override public void init()
    fn init(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| SkyblockerCreditsScreen::init(__h))
    }

    /// java: @Override public void tick()
    fn tick(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen) {
        self.lend(screen, |__h| SkyblockerCreditsScreen::tick(__h))
    }

    /// java: @Override public void removed()
    fn removed(&mut self) {
        self.call(|__h| SkyblockerCreditsScreen::removed(__h))
    }

    /// java: @Override public void extractRenderState(GuiGraphicsExtractor graphics, int mouseX, int mouseY, float a)
    fn extract_render_state(&mut self, screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen, context: &mut ScreenRenderContext<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
        self.lend_ref(screen, |__h| SkyblockerCreditsScreen::extract_render_state(__h, &context.graphics, mouse_x, mouse_y, a))
    }

    /// java: @Override public boolean keyPressed(KeyEvent keyEvent)
    fn key_pressed(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.lend(screen, |__h| SkyblockerCreditsScreen::key_pressed(__h, event))
    }

    /// java: Screen.mouseClicked (the base body)
    fn mouse_clicked(&mut self, screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen, x: f64, y: f64, button: i32) -> bool {
        self.lend(screen, |_| false)
    }

    /// java: @Override public void onClose()
    fn on_close(&mut self) {
        self.call(|__h| SkyblockerCreditsScreen::on_close(__h))
    }

}

// NOTE(fabric2seer): extractMenuBackground, extractRenderState, getBackgroundMusic, getNarrationMessage, init, keyPressed, keyReleased, onClose, removed, tick override(s) of Screen - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: private record Credits(List<Author> authors, List<String> creativeTeam, List<Translation> translations, List<Library> libraries)
#[derive(Clone, Debug, PartialEq)]
pub struct Credits {
    /// java: List<Author> authors
    pub authors: Vec<Author>,
    /// java: List<String> creativeTeam
    pub creative_team: Vec<String>,
    /// java: List<Translation> translations
    pub translations: Vec<Translation>,
    /// java: List<Library> libraries
    pub libraries: Vec<Library>,
}

impl Credits {
    /// java: the canonical record constructor
    pub fn new(mut authors: Vec<Author>, mut creative_team: Vec<String>, mut translations: Vec<Translation>, mut libraries: Vec<Library>) -> Self {
        Self { authors, creative_team, translations, libraries }
    }

    /// java: the record accessor `authors()`
    pub fn authors(&self) -> Vec<Author> {
        self.authors.clone()
    }

    /// java: the record accessor `creativeTeam()`
    pub fn creative_team(&self) -> Vec<String> {
        self.creative_team.clone()
    }

    /// java: the record accessor `translations()`
    pub fn translations(&self) -> Vec<Translation> {
        self.translations.clone()
    }

    /// java: the record accessor `libraries()`
    pub fn libraries(&self) -> Vec<Library> {
        self.libraries.clone()
    }

}

/// java: private record Author(String name, UUID uuid, int colour)
#[derive(Clone, Debug, PartialEq)]
pub struct Author {
    /// java: String name
    pub name: String,
    /// java: UUID uuid
    pub uuid: Uuid,
    /// java: int colour
    pub colour: i32,
}

impl Author {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut uuid: Uuid, mut colour: i32) -> Self {
        Self { name, uuid, colour }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `uuid()`
    pub fn uuid(&self) -> Uuid {
        self.uuid.clone()
    }

    /// java: the record accessor `colour()`
    pub fn colour(&self) -> i32 {
        self.colour
    }

    /// java: public Component asText()
    pub fn as_text(&self) -> MutableComponent {
        let mut profile: ResolvableProfile = ResolvableProfile::create_unresolved_id(self.uuid.clone()).unwrap();
        // TODO(fabric2seer): `Minecraft.playerSkinRenderCache` has no Rust method in the port (looked for `player_skin_render_cache` in Minecraft and its supertypes)
        // rust: compat::minecraft().player_skin_render_cache().lookup(&profile);
        return (component::empty().append(component::object(PlayerSprite::new(profile, true)))).append((component::literal(&format!(" {}", self.name))).with_color(self.colour));
    }

}

/// java: private record Translation(String code, String name, List<String> translators)
#[derive(Clone, Debug, PartialEq)]
pub struct Translation {
    /// java: String code
    pub code: String,
    /// java: String name
    pub name: String,
    /// java: List<String> translators
    pub translators: Vec<String>,
}

impl Translation {
    /// java: the canonical record constructor
    pub fn new(mut code: String, mut name: String, mut translators: Vec<String>) -> Self {
        Self { code, name, translators }
    }

    /// java: the record accessor `code()`
    pub fn code(&self) -> String {
        self.code.clone()
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `translators()`
    pub fn translators(&self) -> Vec<String> {
        self.translators.clone()
    }

}

/// java: private record Library(String name, String credit)
#[derive(Clone, Debug, PartialEq)]
pub struct Library {
    /// java: String name
    pub name: String,
    /// java: String credit
    pub credit: String,
}

impl Library {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut credit: String) -> Self {
        Self { name, credit }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `credit()`
    pub fn credit(&self) -> String {
        self.credit.clone()
    }

}
