//! Ported from de.hysky.skyblocker.skyblock.waypoint.IndividualWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use crate::de::hysky::skyblocker::utils::render::level_render_extraction_callback;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static IndividualWaypoint waypoint;
pub static WAYPOINT: compat::MainThread<Option<compat::Shared<IndividualWaypoint>>> = compat::MainThread::new(|| None);

/// java: public class IndividualWaypoint extends NamedWaypoint
#[derive(Clone)]
pub struct IndividualWaypoint {
    /// java: extends NamedWaypoint
    pub base: NamedWaypointBase,
}

impl std::ops::Deref for IndividualWaypoint {
    type Target = NamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for IndividualWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl IndividualWaypoint {
    /// java: public IndividualWaypoint(BlockPos pos, Component name, float[] colorComponents)
    pub fn new(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        let (_, __this) = NamedWaypointBase::new6_in(pos.clone(), name.clone(), color_components.clone(), waypoint::DEFAULT_HIGHLIGHT_ALPHA, true, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        });
        __this
    }

    /// java: @Init public static void init()
    pub fn init() {
        client_tick_events::END_CLIENT_TICK.register(Arc::new(|minecraft: &mut Minecraft| {
            if WAYPOINT.get_mut().is_some() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.distance_to_sqr_vec(&compat::read(WAYPOINT.get_mut().as_ref().unwrap(), |__t| __t.center_pos.clone()).unwrap())).unwrap() <= (36 as f64) {
                *WAYPOINT.get_mut() = None;
            }
        }));
        level_render_extraction_callback::EVENT.register(Arc::new(move |collector| {
            if WAYPOINT.get_mut().is_some() {
                ({ let __r = WAYPOINT.get_mut().as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.extract_rendering()); __f(&compat::view(__r, |a| a as compat::Shared<dyn Renderable>), &collector) });
            }
        }));
        client_play_connection_events::JOIN.register(Arc::new(|listener: &mut ClientPacketListener| {
            *WAYPOINT.get_mut() = None;
        }));
        client_command_registration_callback::EVENT.register(Arc::new(
            |dispatcher: &mut client_command_registration_callback::ClientCommandDispatcher| {
                // TODO(fabric2seer): a command tree this tool cannot read (arguments, redirects, requires) - SeerClient commands receive the words after the name in `ctx.args`
                //   java: dispatcher.register(..);
            },
        ));
    }

    /// java: private static int setWaypoint(Consumer<Component> feedback, int x, int y, int z, String area)
    pub fn set_waypoint(feedback: Arc<dyn Fn(MutableComponent)>, mut x: i32, mut y: i32, mut z: i32, mut area: String) -> i32 {
        IndividualWaypoint::set_waypoint_4(x, y, z, &area);
        if true /* java: area != null: never null here */ && !area.is_empty() {
            area = format!("| {}", area);
            (feedback)((constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.waypoints.chat.displayed", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(x.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(y.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(z.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(area.clone())])));
        } else {
            (feedback)((constants::PREFIX.get_mut())().append(component::translatable_args("skyblocker.waypoints.chat.displayed", vec![mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(x.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(y.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of(z.to_string()), mcrust::net::minecraft::network::chat::formatted_text::FormattedText::of("".to_string())])));
        }
        return 1;
    }

    /// java: private static void setWaypoint(int x, int y, int z, String area)
    pub fn set_waypoint_4(mut x: i32, mut y: i32, mut z: i32, area: &str) {
        let mut waypoint_name: String = (if true /* java: area != null: never null here */ && !area.is_empty() { area.to_string() } else { "Chat Waypoint".to_string() });
        let mut waypoint_display: MutableComponent;
        if compat::char_at(&waypoint_name, 0) == '⏣' {
            waypoint_display = (component::literal("⏣")).with_style_format(ChatFormatting::DarkPurple).append((component::literal(&compat::substring(&waypoint_name, 1))).with_style_format(ChatFormatting::Aqua));
        } else {
            waypoint_display = component::literal(&waypoint_name).with_style_format(ChatFormatting::Aqua);
        }
        // TODO(fabric2seer): `.getRGB(..)` on a value of unknown type
        //   java: waypoint = new IndividualWaypoint(new BlockPos(x, y, z), waypointDisplay, ColorUtils.getFloatComponents(Color.GREEN.getRGB()));
    }

    /// java: private static void onTick(Minecraft client)
    pub fn on_tick(client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if WAYPOINT.get_mut().is_some() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::with_player(|player| player.base.base.base.base.distance_to_sqr_vec(&compat::read(WAYPOINT.get_mut().as_ref().unwrap(), |__t| __t.center_pos.clone()).unwrap())).unwrap() <= (36 as f64) {
            *WAYPOINT.get_mut() = None;
        }
    }

}

impl NamedWaypoint for IndividualWaypoint {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base
    }

}

impl Waypoint for IndividualWaypoint {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public NamedWaypoint withX(int x) (dispatch: the body for this class)
    fn with_x(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_x_from::<Self>
    }

    /// java: @Override public NamedWaypoint withY(int y) (dispatch: the body for this class)
    fn with_y(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_y_from::<Self>
    }

    /// java: @Override public NamedWaypoint withZ(int z) (dispatch: the body for this class)
    fn with_z(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_z_from::<Self>
    }

    /// java: @Override public NamedWaypoint withColor(float[] colorComponents, float alpha) (dispatch: the body for this class)
    fn with_color(&self) -> fn(&compat::Shared<dyn Waypoint>, &Vec<f32>, f32) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_color_from::<Self>
    }

    /// java: @Override public NamedWaypoint withThroughWalls(boolean throughWalls) (dispatch: the body for this class)
    fn with_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_through_walls_from::<Self>
    }

    /// java: @Override public NamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier) (dispatch: the body for this class)
    fn with_type_supplier(&self) -> fn(&compat::Shared<dyn Waypoint>, Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        <dyn NamedWaypoint>::with_type_supplier_from::<Self>
    }

    /// java: @Override public int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn Waypoint>) -> i32 {
        <dyn NamedWaypoint>::hash_code_from::<Self>
    }

    /// java: @Override public boolean equals(Object obj) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Object) -> bool {
        <dyn NamedWaypoint>::equals_from::<Self>
    }

}

impl Renderable for IndividualWaypoint {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_from::<Self>
    }

}
