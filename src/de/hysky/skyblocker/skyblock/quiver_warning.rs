//! Ported from de.hysky.skyblocker.skyblock.QuiverWarning by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler;
use crate::de::hysky::skyblocker::utils::scheduler::scheduler::Scheduler;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::hud;
use mcrust::net::minecraft::client::gui::hud::Hud;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: private static @Nullable Type warning = null;
pub static WARNING: LazyLock<Mutex<Option<Type>>> = LazyLock::new(|| Mutex::new(None));

/// java: public class QuiverWarning
#[derive(Clone)]
pub struct QuiverWarning {}

impl QuiverWarning {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Init public static void init()
    pub fn init() {
        // TODO(fabric2seer): ClientReceiveMessageEvents.ALLOW_GAME.register: no SeerClient equivalent yet
        //   java: ClientReceiveMessageEvents.ALLOW_GAME.register(QuiverWarning::onChatMessage);
        ({ let __r = scheduler::INSTANCE.get_mut(); let __f = compat::dispatch(__r, |__t| __t.schedule_cyclic()); __f(__r, Arc::new(move || { QuiverWarning::update(); }), 10) });
    }

    /// java: public static boolean onChatMessage(Component text, boolean overlay)
    pub fn on_chat_message(text: &MutableComponent, mut overlay: bool) -> bool {
        let mut message: String = text.get_string();
        if SkyblockerConfigManager::get().general.quiver_warning.enable_quiver_warning && message.ends_with("left in your Quiver!") {
            compat::minecraft().gui.hud.reset_title_times();
            if message.starts_with("You only have 50") {
                QuiverWarning::on_chat_message_1(Type::FiftyLeft);
            } else if message.starts_with("You only have 10") {
                QuiverWarning::on_chat_message_1(Type::TenLeft);
            } else if message.starts_with("You don't have any more") {
                QuiverWarning::on_chat_message_1(Type::Empty);
            }
        }
        return true;
    }

    /// java: private static void onChatMessage(Type warning)
    pub fn on_chat_message_1(mut warning: Type) {
        if !(Utils::is_in_dungeons()) {
            hud::set_title_time(component::translatable(&warning.fields().key).with_style_format(ChatFormatting::Red));
        } else if SkyblockerConfigManager::get().general.quiver_warning.enable_quiver_warning_in_dungeons {
            hud::set_title_time(component::translatable(&warning.fields().key).with_style_format(ChatFormatting::Red));
            compat::set_state(&WARNING, Some(warning));
        }
    }

    /// java: public static void update()
    pub fn update() {
        if compat::get_state(&WARNING).is_some() && SkyblockerConfigManager::get().general.quiver_warning.enable_quiver_warning && SkyblockerConfigManager::get().general.quiver_warning.enable_quiver_warning_after_dungeon && !(Utils::is_in_dungeons()) {
            let hud = &mut compat::minecraft().gui.hud;
            hud.reset_title_times();
            hud::set_title_time(component::translatable(&compat::lock(&WARNING).unwrap().fields().key).with_style_format(ChatFormatting::Red));
            compat::set_state(&WARNING, None);
        }
    }

}

/// java: private enum Type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    None,
    FiftyLeft,
    TenLeft,
    Empty,
}

impl Type {
    pub const VALUES: [Type; 4] = [Type::None, Type::FiftyLeft, Type::TenLeft, Type::Empty];

    /// java: values()
    pub fn values() -> Vec<Type> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Type::None => "NONE",
            Type::FiftyLeft => "FIFTY_LEFT",
            Type::TenLeft => "TEN_LEFT",
            Type::Empty => "EMPTY",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "NONE" => Type::None,
            "FIFTY_LEFT" => Type::FiftyLeft,
            "TEN_LEFT" => Type::TenLeft,
            "EMPTY" => Type::Empty,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static TypeFields {
        static FIELDS: LazyLock<Vec<TypeFields>> = LazyLock::new(|| vec![
            TypeFields::new("".to_string()),
            TypeFields::new("50Left".to_string()),
            TypeFields::new("10Left".to_string()),
            TypeFields::new("empty".to_string()),
        ]);
        &FIELDS[self as usize]
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// the constructor data of each Type constant
pub struct TypeFields {
    pub key: String,
}

impl TypeFields {
    fn new(mut key: String) -> Self {
        let mut this = Self {
            key: Default::default(),
        };
        this.key = format!("skyblocker.quiverWarning.{}", key);
        this
    }

}
