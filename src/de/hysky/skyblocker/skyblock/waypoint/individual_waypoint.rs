//! Ported from de.hysky.skyblocker.skyblock.waypoint.IndividualWaypoint by fabric2seer.

#[allow(unused_imports)]
use mcrust::com::mojang::brigadier::builder::argument_builder::ArgumentBuilder as _;
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::constants;
use mcrust::com::mojang::brigadier::arguments::integer_argument_type::IntegerArgumentType;
use mcrust::com::mojang::brigadier::arguments::string_argument_type::StringArgumentType;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
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
                // java: dispatcher.register(ClientCommands.literal(SkyblockerMod.NAMESPACE).then(ClientCommands.literal("waypoints")
                //   .then(ClientCommands.literal("individual").then(ClientCommands.argument("x", IntegerArgumentType.integer(Integer.MIN_VALUE))
                //     .then(ClientCommands.argument("y", IntegerArgumentType.integer(Integer.MIN_VALUE))
                //       .then(ClientCommands.argument("z", IntegerArgumentType.integer(Integer.MIN_VALUE))
                //         .then(ClientCommands.argument("area", StringArgumentType.greedyString()).executes(context -> setWaypoint(
                //           context.getSource()::sendFeedback, getInteger(context, "x"), getInteger(context, "y"), getInteger(context, "z"), getString(context, "area"))))
                //         .executes(context -> setWaypoint(context.getSource()::sendFeedback, getInteger(context, "x"), getInteger(context, "y"), getInteger(context, "z"), ""))))))));
                type Source = client_command_registration_callback::FabricClientCommandSource;
                let feedback = || -> Arc<dyn Fn(MutableComponent)> { Arc::new(|message: MutableComponent| compat::batch_w_send_feedback(&message)) };
                let mut area = client_command_registration_callback::argument("area", Arc::new(StringArgumentType::greedy_string()));
                area.executes(Arc::new(move |context: &CommandContext<Source>| {
                    Ok(IndividualWaypoint::set_waypoint(feedback(), IntegerArgumentType::get_integer(context, "x"), IntegerArgumentType::get_integer(context, "y"), IntegerArgumentType::get_integer(context, "z"), StringArgumentType::get_string(context, "area")))
                }));
                let mut z = client_command_registration_callback::argument("z", Arc::new(IntegerArgumentType::integer_min(i32::MIN)));
                z.then(&area);
                z.executes(Arc::new(move |context: &CommandContext<Source>| {
                    Ok(IndividualWaypoint::set_waypoint(feedback(), IntegerArgumentType::get_integer(context, "x"), IntegerArgumentType::get_integer(context, "y"), IntegerArgumentType::get_integer(context, "z"), String::new()))
                }));
                let mut y = client_command_registration_callback::argument("y", Arc::new(IntegerArgumentType::integer_min(i32::MIN)));
                y.then(&z);
                let mut x = client_command_registration_callback::argument("x", Arc::new(IntegerArgumentType::integer_min(i32::MIN)));
                x.then(&y);
                let mut individual = client_command_registration_callback::literal("individual");
                individual.then(&x);
                let mut waypoints = client_command_registration_callback::literal("waypoints");
                waypoints.then(&individual);
                let mut root = client_command_registration_callback::literal(crate::entry::skyblocker_mod::NAMESPACE);
                root.then(&waypoints);
                dispatcher.dispatcher().register(&root);
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
        //   java: waypoint = new IndividualWaypoint(new BlockPos(x, y, z), waypointDisplay, ColorUtils.getFloatComponents(Color.GREEN.getRGB()));
        let color_components = crate::de::hysky::skyblocker::utils::color_utils::ColorUtils::get_float_components(mcrust::java::awt::color::green().get_rgb());
        *WAYPOINT.get_mut() = Some(IndividualWaypoint::new(BlockPos::new(x, y, z), waypoint_display, color_components));
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
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Shared<dyn Waypoint>) -> bool {
        <dyn NamedWaypoint>::equals_from::<Self>
    }

    /// java: `instanceof NamedWaypoint` - this class is one
    fn as_named_waypoint(&self) -> Option<&crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase> {
        Some(self.named_waypoint_base())
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
