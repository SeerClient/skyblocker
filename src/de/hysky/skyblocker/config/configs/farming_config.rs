//! Ported from de.hysky.skyblocker.config.configs.FarmingConfig by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;

// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
/// java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
pub static TYPE_CODEC: compat::MainThread<CodecRef<Type>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class FarmingConfig
#[derive(Clone)]
pub struct FarmingConfig {
    /// java: @Deprecated public transient FarmingHud farmingHud = new FarmingHud();
    pub farming_hud: FarmingHud,
    /// java: public PestHighlighter pestHighlighter = new PestHighlighter();
    pub pest_highlighter: PestHighlighter,
    /// java: public MouseLock mouseLock = new MouseLock();
    pub mouse_lock: MouseLock,
    /// java: public PlotsWidget plotsWidget = new PlotsWidget();
    pub plots_widget: PlotsWidget,
    /// java: public VisitorHelper visitorHelper = new VisitorHelper();
    pub visitor_helper: VisitorHelper,
    /// java: public Greenhouse greenhouse = new Greenhouse();
    pub greenhouse: Greenhouse,
}

impl FarmingConfig {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            farming_hud: FarmingHud::new(),
            pest_highlighter: PestHighlighter::new(),
            mouse_lock: MouseLock::new(),
            plots_widget: PlotsWidget::new(),
            visitor_helper: VisitorHelper::new(),
            greenhouse: Greenhouse::new(),
        };
        this
    }

}

/// java: public static class Greenhouse
#[derive(Clone)]
pub struct Greenhouse {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean showMutationSlot = true;
    pub show_mutation_slot: bool,
}

impl Greenhouse {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            show_mutation_slot: true,
        };
        this
    }

}

/// java: public static class PestHighlighter
#[derive(Clone)]
pub struct PestHighlighter {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean vinylHighlighter = true;
    pub vinyl_highlighter: bool,
    /// java: public boolean contestHighlighter = true;
    pub contest_highlighter: bool,
    /// java: public boolean enableStereoHarmonyHelperForContest = true;
    pub enable_stereo_harmony_helper_for_contest: bool,
}

impl PestHighlighter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            vinyl_highlighter: true,
            contest_highlighter: true,
            enable_stereo_harmony_helper_for_contest: true,
        };
        this
    }

}

/// java: public static class MouseLock
#[derive(Clone)]
pub struct MouseLock {
    /// java: public boolean lockMouseTool = false;
    pub lock_mouse_tool: bool,
    /// java: public boolean lockMouseGroundOnly = false;
    pub lock_mouse_ground_only: bool,
}

impl MouseLock {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            lock_mouse_tool: false,
            lock_mouse_ground_only: false,
        };
        this
    }

}

/// java: public static class PlotsWidget
#[derive(Clone)]
pub struct PlotsWidget {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean closeScreenOnPlotClick = false;
    pub close_screen_on_plot_click: bool,
    /// java: public int x = 0;
    pub x: i32,
    /// java: public int y = 0;
    pub y: i32,
}

impl PlotsWidget {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            close_screen_on_plot_click: false,
            x: 0,
            y: 0,
        };
        this
    }

}

/// java: public static class VisitorHelper
#[derive(Clone)]
pub struct VisitorHelper {
    /// java: public boolean enabled = true;
    pub enabled: bool,
    /// java: public boolean showInGardenOnly = true;
    pub show_in_garden_only: bool,
    /// java: public boolean showInStacks = false;
    pub show_in_stacks: bool,
}

impl VisitorHelper {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            show_in_garden_only: true,
            show_in_stacks: false,
        };
        this
    }

}

/// java: public static class FarmingHud
#[derive(Clone)]
pub struct FarmingHud {
    /// java: @Deprecated public transient boolean enabled = true;
    pub enabled: bool,
    /// java: @Deprecated public transient boolean counter = true;
    pub counter: bool,
    /// java: @Deprecated public transient boolean coins = true;
    pub coins: bool,
    /// java: @Deprecated public transient Type type = Type.BOTH;
    pub r#type: Type,
    /// java: @Deprecated public transient boolean includeSeedsPrice = true;
    pub include_seeds_price: bool,
    /// java: @Deprecated public transient boolean experience = true;
    pub experience: bool,
}

impl FarmingHud {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            enabled: true,
            counter: true,
            coins: true,
            r#type: Type::Both,
            include_seeds_price: true,
            experience: true,
        };
        this
    }

}

/// java: public enum Type implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Both,
    Npc,
    Bazaar,
}

impl Type {
    pub const VALUES: [Type; 3] = [Type::Both, Type::Npc, Type::Bazaar];

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
            Type::Both => "BOTH",
            Type::Npc => "NPC",
            Type::Bazaar => "BAZAAR",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "BOTH" => Type::Both,
            "NPC" => Type::Npc,
            "BAZAAR" => Type::Bazaar,
            _ => panic!("IllegalArgumentException: No enum constant Type.{name}"),
        }
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return name().toLowerCase(Locale.ENGLISH);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        // TODO(fabric2seer): call to `name(..)` (not a method of this class)
        //   java: return I18n.get("skyblocker.config.farming.farmingHud.type." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Type::to_string(*self))
    }
}
