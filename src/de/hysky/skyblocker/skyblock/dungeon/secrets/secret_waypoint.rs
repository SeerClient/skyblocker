//! Ported from de.hysky.skyblocker.skyblock.dungeon.secrets.SecretWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::dungeons_config::SecretWaypoints;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::skyblock::dungeon::dungeon_score::DungeonScore;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::dungeon_manager;
use crate::de::hysky::skyblocker::skyblock::dungeon::secrets::room::Room;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::distanced_named_waypoint::DistancedNamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::com::mojang::brigadier::context::command_context::CommandContext;
use mcrust::com::mojang::serialization::codec;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::com::mojang::serialization::codecs::record_codec_builder;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::commands::arguments::string_representable_argument::StringRepresentableArgument;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::world::entity::entity::Entity;
use mcrust::net::minecraft::world::phys::aabb::AABB;
use crate::de::hysky::skyblocker::utils::data::json_data::JsonData as _;
use mcrust::com::mojang::serialization::decoder::Decoder as _;

/// java: public static final Codec<SecretWaypoint> CODEC = RecordCodecBuilder.create(instance -> instance.group( Codec.INT.fieldOf("secretIndex").forGetter(secretWaypoint -> secretWaypoint.secretIndex), Catego
pub static CODEC: compat::MainThread<CodecRef<compat::Shared<SecretWaypoint>>> = compat::MainThread::new(|| { /* port shape: the record is over the getters' values, `xmap`ped to the waypoint (as NamedWaypoint.CODEC) */ type Fields = (i32, Category, MutableComponent, BlockPos); let record: CodecRef<Fields> = mcrust::com::mojang::serialization::codecs::record_codec_builder::create(mcrust::com::mojang::serialization::codecs::record_codec_builder::apply4(mcrust::com::mojang::serialization::codecs::record_codec_builder::of(|o: &Fields| o.0, codec::field_of(codec::int_codec(), "secretIndex")), mcrust::com::mojang::serialization::codecs::record_codec_builder::of(|o: &Fields| o.1, codec::field_of(CATEGORY_CODEC.clone(), "category")), mcrust::com::mojang::serialization::codecs::record_codec_builder::of(|o: &Fields| o.2.clone(), codec::field_of(mcrust::net::minecraft::network::chat::component_serialization::component_codec(), "name")), mcrust::com::mojang::serialization::codecs::record_codec_builder::of(|o: &Fields| o.3.clone(), codec::field_of(mcrust::net::minecraft::core::block_pos::codec(), "pos")), |secret_index, category, name, pos| (secret_index, category, name, pos))); codec::xmap(record, |(secret_index, category, name, pos): Fields| SecretWaypoint::new2(secret_index, category, name, pos), |w: &compat::Shared<SecretWaypoint>| compat::read(w, |t| (t.secret_index, t.category.unwrap(), t.name.clone().unwrap(), t.pos.clone().unwrap()))) });
/// java: public static final Codec<List<SecretWaypoint>> LIST_CODEC = CODEC.listOf();
pub static LIST_CODEC: compat::MainThread<CodecRef<Vec<compat::Shared<SecretWaypoint>>>> = compat::MainThread::new(|| codec::list(CODEC.get_mut().clone()));
/// java: static final List<String> SECRET_ITEMS = List.of("Candycomb", "Decoy", "Defuse Kit", "Dungeon Chest Key", "Healing VIII", "Inflatable Jerry", "Spirit Leap", "Training Weights", "Trap", "Treasure Talis
pub static SECRET_ITEMS: LazyLock<Vec<String>> = LazyLock::new(|| vec!["Candycomb".to_string(), "Decoy".to_string(), "Defuse Kit".to_string(), "Dungeon Chest Key".to_string(), "Healing VIII".to_string(), "Inflatable Jerry".to_string(), "Spirit Leap".to_string(), "Training Weights".to_string(), "Trap".to_string(), "Treasure Talisman".to_string()]);
/// java: private static final Supplier<DungeonsConfig.SecretWaypoints> CONFIG = () -> SkyblockerConfigManager.get().dungeons.secretWaypoints;
pub static CONFIG: compat::MainThread<Arc<dyn Fn() -> compat::Shared<SecretWaypoints>>> = compat::MainThread::new(|| Arc::new(move || SkyblockerConfigManager::get().dungeons.secret_waypoints.clone()));
/// java: static final Supplier<Type> TYPE_SUPPLIER = () -> CONFIG.get().waypointType;
pub static TYPE_SUPPLIER: compat::MainThread<Arc<dyn Fn() -> Type>> = compat::MainThread::new(|| Arc::new(move || compat::read(&(CONFIG.get_mut())(), |__t| __t.waypoint_type)));
/// java: public static final Codec<Category> CODEC = StringRepresentable.fromEnum(Category::values);
/// (a `LazyLock`: the codec is Send + Sync, and the room loading reads it off the render thread, as Java does)
pub static CATEGORY_CODEC: LazyLock<CodecRef<Category>> = LazyLock::new(|| mcrust::net::minecraft::util::string_representable::from_enum(Category::values(), |e: &Category| e.get_serialized_name()));

/// java: public class SecretWaypoint extends DistancedNamedWaypoint
#[derive(Clone)]
pub struct SecretWaypoint {
    /// java: extends DistancedNamedWaypoint
    pub base: DistancedNamedWaypointBase,
    /// java: public final int secretIndex;
    pub secret_index: i32,
    /// java: public final Category category;
    pub category: Option<Category>,
}

impl std::ops::Deref for SecretWaypoint {
    type Target = DistancedNamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SecretWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SecretWaypoint {
    /// java: public SecretWaypoint(int secretIndex, Category category, String name, BlockPos pos)
    pub fn new(mut secret_index: i32, mut category: Category, mut name: String, mut pos: BlockPos) -> compat::Shared<Self> {
        let __this = Self::new2(secret_index, category, component::null_to_empty(Some(&name)), pos.clone());
        __this
    }

    /// java: SecretWaypoint(int secretIndex, Category category, Component name, BlockPos pos)
    pub fn new2(mut secret_index: i32, mut category: Category, mut name: MutableComponent, mut pos: BlockPos) -> compat::Shared<Self> {
        let __init_secret_index = secret_index;
        let __init_category = Some(category);
        let (_, __this) = DistancedNamedWaypointBase::new2_in(pos.clone(), name.clone(), TYPE_SUPPLIER.get_mut().clone(), SecretWaypoint::get_color_for(category), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                secret_index: __init_secret_index,
                category: __init_category,
            });
            ((__h.clone() as compat::Shared<dyn DistancedNamedWaypoint>), __h)
        });
        __this
    }

    /// java: private static float[] getColorFor(Category category)
    pub fn get_color_for(mut category: Category) -> Vec<f32> {
        if !(dungeon_manager::WAYPOINT_COLOR_DATA.get_mut().lock().borrow().is_loaded()) {
            return category.fields().color_components.clone();
        }
        // java: Color color = DungeonManager.WAYPOINT_COLOR_DATA.getData().get(category);
        // (the data map is read under the handle's borrow and the colour copied out)
        let color: Option<Color> = dungeon_manager::WAYPOINT_COLOR_DATA.get_mut().lock().borrow().get_data().get(&category).cloned();
        let Some(color) = color else {
            return category.fields().color_components.clone();
        };
        // java: return color.getColorComponents(null);
        return color.get_color_components(None).to_vec();
    }

    /// java: static ToDoubleFunction<SecretWaypoint> getSquaredDistanceToFunction(Entity entity)
    /// The entity's position is read when the function is made (both callers apply it at once, inside the stream
    /// the entity was handed to).
    pub fn get_squared_distance_to_function(entity: &dyn Entity) -> Arc<dyn Fn(compat::Shared<SecretWaypoint>) -> f64> {
        // Entity.distanceToSqr(Vec3) = the position's distanceToSqr
        let position = entity.base().position();
        return Arc::new(move |secret_waypoint| position.distance_to_sqr_vec(&compat::read(&secret_waypoint, |__t| __t.center_pos.clone()).unwrap()));
    }

    /// java: static Predicate<SecretWaypoint> getRangePredicate(Entity entity)
    /// See `get_squared_distance_to_function`.
    pub fn get_range_predicate(entity: &dyn Entity) -> Arc<dyn Fn(compat::Shared<SecretWaypoint>) -> bool> {
        let position = entity.base().position();
        return Arc::new(move |secret_waypoint| position.closer_than(&compat::read(&secret_waypoint, |__t| __t.center_pos.clone()).unwrap(), (16) as f64));
    }

    /// java: boolean needsInteraction()
    pub fn needs_interaction(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().category.as_ref().unwrap().needs_interaction();
    }

    /// java: boolean isLever()
    pub fn is_lever(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().category.as_ref().unwrap().is_lever();
    }

    /// java: boolean needsItemPickup()
    pub fn needs_item_pickup(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().category.as_ref().unwrap().needs_item_pickup();
    }

    /// java: boolean isBat()
    pub fn is_bat(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().category.as_ref().unwrap().is_bat();
    }

    /// java: SecretWaypoint relativeToActual(Room room)
    pub fn relative_to_actual(__this: &compat::Shared<Self>, room: &compat::Shared<dyn Room>) -> compat::Shared<SecretWaypoint> {
        return SecretWaypoint::new2(compat::read(__this, |__t| __t.secret_index), compat::read(__this, |__t| __t.category.clone()).unwrap(), compat::read(__this, |__t| __t.name.clone()).unwrap(), ({ let __f = compat::dispatch(room, |__t| __t.relative_to_actual()); __f(room, &compat::read(__this, |__t| __t.pos.clone()).unwrap()) }));
    }

}

impl DistancedNamedWaypoint for SecretWaypoint {
    fn distanced_named_waypoint_base(&self) -> &DistancedNamedWaypointBase {
        &self.base
    }
    fn distanced_named_waypoint_base_mut(&mut self) -> &mut DistancedNamedWaypointBase {
        &mut self.base
    }

}

impl NamedWaypoint for SecretWaypoint {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base.base
    }

    /// java: @Override protected boolean shouldRenderName() (dispatch: the body for this class)
    fn should_render_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> bool {
        SecretWaypoint::should_render_name_dyn
    }

}

impl Waypoint for SecretWaypoint {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        SecretWaypoint::should_render_dyn
    }

    /// java: @Override public boolean equals(Object obj) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Shared<dyn Waypoint>) -> bool {
        SecretWaypoint::equals_dyn
    }

    /// java: `instanceof NamedWaypoint` - this class is one
    fn as_named_waypoint(&self) -> Option<&crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase> {
        Some(self.named_waypoint_base())
    }

    /// java: @Override public int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn Waypoint>) -> i32 {
        SecretWaypoint::hash_code_dyn
    }

    /// java: @Override protected AABB getRenderBox() (dispatch: the body for this class)
    fn get_render_box(&self) -> fn(&compat::Shared<dyn Waypoint>) -> AABB {
        SecretWaypoint::get_render_box_dyn
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

}

impl Renderable for SecretWaypoint {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn DistancedNamedWaypoint>::extract_rendering_from::<Self>
    }

}

impl SecretWaypoint {
    /// java: @Override protected boolean shouldRenderName()
    pub fn should_render_name_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn NamedWaypoint>::should_render_name_body(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>)) && compat::read(&(CONFIG.get_mut())(), |__t| __t.show_secret_text));
    }

    /// SecretWaypoint.shouldRenderName as the dispatch of NamedWaypoint.shouldRenderName
    pub fn should_render_name_dyn(__this: &compat::Shared<dyn NamedWaypoint>) -> bool {
        Self::should_render_name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        if __this.lock().borrow().category.as_ref().unwrap().is_prince() {
            return (!(DungeonScore::was_prince_killed()) && __this.lock().borrow().category.as_ref().unwrap().is_enabled());
        }
        return (<dyn Waypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) && __this.lock().borrow().category.as_ref().unwrap().is_enabled());
    }

    /// SecretWaypoint.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean equals(Object obj)
    pub fn equals_impl(__this: &compat::Shared<Self>, obj: &compat::Shared<dyn Waypoint>) -> bool {
        // java: return this == obj || super.equals(obj) && obj instanceof SecretWaypoint other && secretIndex == other.secretIndex && category == other.category;
        // (`super` is `DistancedNamedWaypoint`, which inherits `NamedWaypoint.equals`)
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(__this), std::sync::Arc::as_ptr(obj)) {
            return true;
        }
        if !<dyn NamedWaypoint>::equals_body(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>), obj) {
            return false;
        }
        let Some((other_secret_index, other_category)) = compat::read(obj, |o| o.as_any().downcast_ref::<SecretWaypoint>().map(|other| (other.secret_index, other.category))) else {
            return false;
        };
        return compat::read(__this, |t| t.secret_index == other_secret_index && t.category == other_category);
    }

    /// SecretWaypoint.equals as the dispatch of Waypoint.equals
    pub fn equals_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: &compat::Shared<dyn Waypoint>) -> bool {
        Self::equals_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public int hashCode()
    /// `Objects.hash(secretIndex, category.getSerializedName(), name, pos)` as Java's own `int`: the value is sent to
    /// the other players' clients (`DungeonRoomHideWaypointMessage.waypointHash`, `SecretSync.syncSecretFound`) and
    /// matched against THEIR `hashCode()` (`Room.getIndexByWaypointHash`), so it must be the JVM's number.
    pub fn hash_code_impl(__this: &compat::Shared<Self>) -> i32 {
        let (secret_index, serialized_name, name, pos) = compat::read(__this, |t| (t.secret_index, t.category.as_ref().unwrap().get_serialized_name(), t.name.clone().unwrap(), t.pos.clone().unwrap()));
        // `Arrays.hashCode(Object[])`: `31 * h + hashCode(e)` from 1
        return [secret_index, compat::string_hash_code(&serialized_name), compat::component_java_hash_code(&name), pos.as_vec3i().java_hash_code()]
            .iter()
            .fold(1i32, |h, e| h.wrapping_mul(31).wrapping_add(*e));
    }

    /// SecretWaypoint.hashCode as the dispatch of Waypoint.hashCode
    pub fn hash_code_dyn(__this: &compat::Shared<dyn Waypoint>) -> i32 {
        Self::hash_code_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override protected AABB getRenderBox()
    pub fn get_render_box_impl(__this: &compat::Shared<Self>) -> AABB {
        if compat::read(&SkyblockerConfigManager::get().dungeons.secret_waypoints, |__t| __t.adaptive_box_size) && (compat::read(__this, |__t| __t.category.clone()).unwrap() == Category::Wither || compat::read(__this, |__t| __t.category.clone()).unwrap() == Category::RedstoneKey) {
            let mut r#box: Option<AABB> = RenderHelper::get_block_bounding_box(&mcrust::mcrust::modding::game::level().unwrap(), &compat::read(__this, |__t| __t.pos.clone()).unwrap());
            if let Some(r#box) = &r#box {
                return r#box.clone();
            }
        }
        return <dyn Waypoint>::get_render_box_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>));
    }

    /// SecretWaypoint.getRenderBox as the dispatch of Waypoint.getRenderBox
    pub fn get_render_box_dyn(__this: &compat::Shared<dyn Waypoint>) -> AABB {
        Self::get_render_box_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public enum Category implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Entrance,
    Superboom,
    Chest,
    Item,
    Bat,
    Wither,
    RedstoneKey,
    Lever,
    Fairysoul,
    Stonk,
    Aotv,
    Pearl,
    Prince,
    Default,
}

impl Category {
    pub const VALUES: [Category; 14] = [Category::Entrance, Category::Superboom, Category::Chest, Category::Item, Category::Bat, Category::Wither, Category::RedstoneKey, Category::Lever, Category::Fairysoul, Category::Stonk, Category::Aotv, Category::Pearl, Category::Prince, Category::Default];

    /// java: values()
    pub fn values() -> Vec<Category> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            Category::Entrance => "ENTRANCE",
            Category::Superboom => "SUPERBOOM",
            Category::Chest => "CHEST",
            Category::Item => "ITEM",
            Category::Bat => "BAT",
            Category::Wither => "WITHER",
            Category::RedstoneKey => "REDSTONE_KEY",
            Category::Lever => "LEVER",
            Category::Fairysoul => "FAIRYSOUL",
            Category::Stonk => "STONK",
            Category::Aotv => "AOTV",
            Category::Pearl => "PEARL",
            Category::Prince => "PRINCE",
            Category::Default => "DEFAULT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Category {
        match name {
            "ENTRANCE" => Category::Entrance,
            "SUPERBOOM" => Category::Superboom,
            "CHEST" => Category::Chest,
            "ITEM" => Category::Item,
            "BAT" => Category::Bat,
            "WITHER" => Category::Wither,
            "REDSTONE_KEY" => Category::RedstoneKey,
            "LEVER" => Category::Lever,
            "FAIRYSOUL" => Category::Fairysoul,
            "STONK" => Category::Stonk,
            "AOTV" => Category::Aotv,
            "PEARL" => Category::Pearl,
            "PRINCE" => Category::Prince,
            "DEFAULT" => Category::Default,
            _ => panic!("IllegalArgumentException: No enum constant Category.{name}"),
        }
    }

    /// the constant's constructor data (Java keeps it in the constant's fields)
    pub fn fields(self) -> &'static CategoryFields {
        static FIELDS: LazyLock<Vec<CategoryFields>> = LazyLock::new(|| vec![
            CategoryFields::new("entrance".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_entrance_waypoints)), vec![0, 255, 0]),
            CategoryFields::new("superboom".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_superboom_waypoints)), vec![255, 0, 0]),
            CategoryFields::new("chest".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_chest_waypoints)), vec![2, 213, 250]),
            CategoryFields::new("item".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_item_waypoints)), vec![2, 64, 250]),
            CategoryFields::new("bat".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_bat_waypoints)), vec![142, 66, 0]),
            CategoryFields::new("wither".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_wither_waypoints)), vec![30, 30, 30]),
            CategoryFields::new("key".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_redstone_key_waypoints)), vec![200, 30, 30]),
            CategoryFields::new("lever".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_lever_waypoints)), vec![250, 217, 2]),
            CategoryFields::new("fairysoul".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_fairy_soul_waypoints)), vec![255, 85, 255]),
            CategoryFields::new("stonk".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_stonk_waypoints)), vec![146, 52, 235]),
            CategoryFields::new("aotv".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_aotv_waypoints)), vec![252, 98, 3]),
            CategoryFields::new("pearl".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_pearl_waypoints)), vec![57, 117, 125]),
            CategoryFields::new("prince".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_prince_waypoints)), vec![133, 21, 13]),
            CategoryFields::new("default".to_string(), Arc::new(move |secret_waypoints| compat::read(&secret_waypoints, |__t| __t.enable_default_waypoints)), vec![190, 255, 252]),
        ]);
        &FIELDS[self as usize]
    }

    /// java: public float[] getColorComponents()
    pub fn get_color_components(&self) -> Vec<f32> {
        return self.fields().color_components.clone();
    }

    /// java: static Category get(JsonObject waypointJson)
    pub fn get(waypoint_json: &serde_json::Map<String, serde_json::Value>) -> Category {
        return (CATEGORY_CODEC.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &compat::json_to_port(&waypoint_json.get("category").cloned().unwrap()))).result_or_partial_with(|__a0| { logger().error(&__a0); }).unwrap_or(Category::Default);
    }

    /// java: boolean needsInteraction()
    pub fn needs_interaction(&self) -> bool {
        return (*self == Category::Chest || *self == Category::Wither || *self == Category::RedstoneKey);
    }

    /// java: boolean isLever()
    pub fn is_lever(&self) -> bool {
        return (*self == Category::Lever);
    }

    /// java: boolean needsItemPickup()
    pub fn needs_item_pickup(&self) -> bool {
        return (*self == Category::Item);
    }

    /// java: boolean isBat()
    pub fn is_bat(&self) -> bool {
        return (*self == Category::Bat);
    }

    /// java: boolean isPrince()
    pub fn is_prince(&self) -> bool {
        return (*self == Category::Prince);
    }

    /// java: boolean isEnabled()
    pub fn is_enabled(&self) -> bool {
        return (self.fields().enabled_predicate)(SkyblockerConfigManager::get().dungeons.secret_waypoints.clone());
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return self.fields().name.clone();
    }

    /// java: @Override public String getSerializedName()
    pub fn get_serialized_name(&self) -> String {
        return self.fields().name.clone();
    }

}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Category::to_string(self))
    }
}

/// the constructor data of each Category constant
pub struct CategoryFields {
    pub name: String,
    pub enabled_predicate: Arc<dyn Fn(compat::Shared<SecretWaypoints>) -> bool + Send + Sync>,
    pub color_components: Vec<f32>,
}

impl CategoryFields {
    fn new(mut name: String, enabled_predicate: Arc<dyn Fn(compat::Shared<SecretWaypoints>) -> bool + Send + Sync>, mut int_color_components: Vec<i32>) -> Self {
        let mut this = Self {
            name: name.clone(),
            enabled_predicate: enabled_predicate.clone(),
            color_components: vec![0.0; ((int_color_components.len() as i32)) as usize],
        };
        for i in 0..(int_color_components.len() as i32) {
            this.color_components[(i) as usize] = ((int_color_components[(i) as usize] as f32) / 255.0f32);
        }
        this
    }

}

/// java: static class CategoryArgumentType extends StringRepresentableArgument<Category>
pub struct CategoryArgumentType {
    /// java: extends net.minecraft.commands.arguments.StringRepresentableArgument
    pub base: StringRepresentableArgument<Category>,
}

impl std::ops::Deref for CategoryArgumentType {
    type Target = StringRepresentableArgument<Category>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for CategoryArgumentType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl CategoryArgumentType {
    /// java: CategoryArgumentType()
    pub fn new() -> Self {
        // java: super(Category.CODEC, Category::values); - the two-argument constructor is `this(codec, values, s -> s)`; the
        // port's is handed the names the codec writes (`CODEC`'s `getSerializedName`) and that identity.
        let mut this: Self = Self {
            base: StringRepresentableArgument::new(Category::values(), |value: &Category| value.get_serialized_name(), |id: &str| id.to_string()),
        };
        this
    }

    /// java: static CategoryArgumentType category()
    pub fn category() -> CategoryArgumentType {
        return CategoryArgumentType::new();
    }

    /// java: static <S> Category getCategory(CommandContext<S> context, String name)
    pub fn get_category<S: 'static>(context: &CommandContext<S>, name: &str) -> Category {
        return context.get_argument::<Category>(name).cloned().unwrap();
    }

}

