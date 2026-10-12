//! Ported from de.hysky.skyblocker.utils.waypoint.OrderedNamedWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::config::configs::ui_and_visuals_config::Waypoints;
use crate::de::hysky::skyblocker::config::skyblocker_config_manager::SkyblockerConfigManager;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final float[] RED_COLOR_COMPONENTS = {1f, 0f, 0f};
pub static RED_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 0.0f32, 0.0f32]);
/// java: private static final float[] WHITE_COLOR_COMPONENTS = {1f, 1f, 1f};
pub static WHITE_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![1.0f32, 1.0f32, 1.0f32]);
/// java: private static final float[] GREEN_COLOR_COMPONENTS = {0f, 1f, 0f};
pub static GREEN_COLOR_COMPONENTS: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 1.0f32, 0.0f32]);
/// java: private static final float[] FLOAT_ARRAY = new float[4];
pub static FLOAT_ARRAY: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0; (4) as usize]);

/// java: public class OrderedNamedWaypoint extends NamedWaypoint
#[derive(Clone)]
pub struct OrderedNamedWaypoint {
    /// java: extends NamedWaypoint
    pub base: NamedWaypointBase,
    /// java: int index;
    pub index: i32,
    /// java: RelativeIndex relativeIndex = RelativeIndex.NONE;
    pub relative_index: RelativeIndex,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for OrderedNamedWaypoint {
    type Target = NamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for OrderedNamedWaypoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl OrderedNamedWaypoint {
    /// java: public OrderedNamedWaypoint(NamedWaypoint namedWaypoint)
    pub fn new(mut named_waypoint: compat::Shared<dyn NamedWaypoint>) -> compat::Shared<Self> {
        let __this = Self::new_7(compat::read(&named_waypoint, |__t| __t.named_waypoint_base().pos.clone()).unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().name.clone()).unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().color_components.clone()), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().alpha), ({ let __f = compat::dispatch(&named_waypoint, |__t| __t.is_enabled()); __f(&compat::view(&named_waypoint, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(&named_waypoint, |__t| __t.named_waypoint_base().through_walls));
        __this
    }

    /// java: public OrderedNamedWaypoint(BlockPos pos, String name, float[] colorComponents)
    pub fn new_3(mut pos: BlockPos, mut name: String, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        let __this = Self::new_6(pos.clone(), component::null_to_empty(Some(&name)), Arc::new(move || SkyblockerConfigManager::get().ui_and_visuals.waypoints.waypoint_type), color_components.clone(), waypoint::DEFAULT_HIGHLIGHT_ALPHA, true);
        __this
    }

    /// java: public OrderedNamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean shouldRender)
    pub fn new_6(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut should_render: bool) -> compat::Shared<Self> {
        let __this = Self::new_7(pos.clone(), name.clone(), type_supplier.clone(), color_components.clone(), alpha, should_render, true);
        __this
    }

    /// java: public OrderedNamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, boolean shouldRender, boolean throughWalls)
    pub fn new_7(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut should_render: bool, mut through_walls: bool) -> compat::Shared<Self> {
        let (_, __this) = NamedWaypointBase::new_7_in(pos.clone(), name.clone(), type_supplier.clone(), color_components.clone(), alpha, should_render, through_walls, move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                index: 0,
                relative_index: RelativeIndex::None,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), __h)
        });
        __this
    }

}

impl NamedWaypoint for OrderedNamedWaypoint {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base
    }

    /// java: @Override public OrderedNamedWaypoint copy() (dispatch: the body for this class)
    fn copy(&self) -> fn(&compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        OrderedNamedWaypoint::copy_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withName(String name) (dispatch: the body for this class)
    fn with_name(&self) -> fn(&compat::Shared<dyn NamedWaypoint>, &str) -> compat::Shared<dyn NamedWaypoint> {
        OrderedNamedWaypoint::with_name_dyn
    }

}

impl Waypoint for OrderedNamedWaypoint {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base.base
    }

    /// java: @Override public OrderedNamedWaypoint withX(int x) (dispatch: the body for this class)
    fn with_x(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_x_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withY(int y) (dispatch: the body for this class)
    fn with_y(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_y_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withZ(int z) (dispatch: the body for this class)
    fn with_z(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_z_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withColor(float[] colorComponents, float alpha) (dispatch: the body for this class)
    fn with_color(&self) -> fn(&compat::Shared<dyn Waypoint>, &Vec<f32>, f32) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_color_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withThroughWalls(boolean throughWalls) (dispatch: the body for this class)
    fn with_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_through_walls_dyn
    }

    /// java: @Override public OrderedNamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier) (dispatch: the body for this class)
    fn with_type_supplier(&self) -> fn(&compat::Shared<dyn Waypoint>, Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        OrderedNamedWaypoint::with_type_supplier_dyn
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        OrderedNamedWaypoint::should_render_dyn
    }

    /// java: @Override public float[] getRenderColorComponents() (dispatch: the body for this class)
    fn get_render_color_components(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Vec<f32> {
        OrderedNamedWaypoint::get_render_color_components_dyn
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

impl Renderable for OrderedNamedWaypoint {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        OrderedNamedWaypoint::extract_rendering_dyn
    }

}

impl OrderedNamedWaypoint {
    /// java: @Override public OrderedNamedWaypoint copy()
    pub fn copy_impl(__this: &compat::Shared<Self>) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new((__this.clone() as compat::Shared<dyn NamedWaypoint>));
    }

    /// OrderedNamedWaypoint.copy as the dispatch of NamedWaypoint.copy
    pub fn copy_dyn(__this: &compat::Shared<dyn NamedWaypoint>) -> compat::Shared<dyn NamedWaypoint> {
        Self::copy_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public OrderedNamedWaypoint withName(String name)
    pub fn with_name_impl(__this: &compat::Shared<Self>, name: &str) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_6(compat::read(__this, |__t| __t.pos.clone()).unwrap(), component::null_to_empty(Some(name)), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }));
    }

    /// OrderedNamedWaypoint.withName as the dispatch of NamedWaypoint.withName
    pub fn with_name_dyn(__this: &compat::Shared<dyn NamedWaypoint>, __p0: &str) -> compat::Shared<dyn NamedWaypoint> {
        Self::with_name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OrderedNamedWaypoint withX(int x)
    pub fn with_x_impl(__this: &compat::Shared<Self>, mut x: i32) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(BlockPos::new(x, __this.lock().borrow().pos.as_ref().unwrap().get_y(), __this.lock().borrow().pos.as_ref().unwrap().get_z()), compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.through_walls));
    }

    /// OrderedNamedWaypoint.withX as the dispatch of Waypoint.withX
    pub fn with_x_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        Self::with_x_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OrderedNamedWaypoint withY(int y)
    pub fn with_y_impl(__this: &compat::Shared<Self>, mut y: i32) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(BlockPos::new(__this.lock().borrow().pos.as_ref().unwrap().get_x(), y, __this.lock().borrow().pos.as_ref().unwrap().get_z()), compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.through_walls));
    }

    /// OrderedNamedWaypoint.withY as the dispatch of Waypoint.withY
    pub fn with_y_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        Self::with_y_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OrderedNamedWaypoint withZ(int z)
    pub fn with_z_impl(__this: &compat::Shared<Self>, mut z: i32) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(BlockPos::new(__this.lock().borrow().pos.as_ref().unwrap().get_x(), __this.lock().borrow().pos.as_ref().unwrap().get_y(), z), compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.through_walls));
    }

    /// OrderedNamedWaypoint.withZ as the dispatch of Waypoint.withZ
    pub fn with_z_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: i32) -> compat::Shared<dyn Waypoint> {
        Self::with_z_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OrderedNamedWaypoint withColor(float[] colorComponents, float alpha)
    pub fn with_color_impl(__this: &compat::Shared<Self>, color_components: &Vec<f32>, mut alpha: f32) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(compat::read(__this, |__t| __t.pos.clone()).unwrap(), compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), color_components.clone(), alpha, ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.through_walls));
    }

    /// OrderedNamedWaypoint.withColor as the dispatch of Waypoint.withColor
    pub fn with_color_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: &Vec<f32>, __p1: f32) -> compat::Shared<dyn Waypoint> {
        Self::with_color_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

    /// java: @Override public OrderedNamedWaypoint withThroughWalls(boolean throughWalls)
    pub fn with_through_walls_impl(__this: &compat::Shared<Self>, mut through_walls: bool) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(compat::read(__this, |__t| __t.pos.clone()).unwrap(), compat::read(__this, |__t| __t.name.clone()).unwrap(), compat::read(__this, |__t| __t.type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), through_walls);
    }

    /// OrderedNamedWaypoint.withThroughWalls as the dispatch of Waypoint.withThroughWalls
    pub fn with_through_walls_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: bool) -> compat::Shared<dyn Waypoint> {
        Self::with_through_walls_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OrderedNamedWaypoint withTypeSupplier(Supplier<Type> typeSupplier)
    pub fn with_type_supplier_impl(__this: &compat::Shared<Self>, type_supplier: Arc<dyn Fn() -> Type>) -> compat::Shared<OrderedNamedWaypoint> {
        return OrderedNamedWaypoint::new_7(compat::read(__this, |__t| __t.pos.clone()).unwrap(), compat::read(__this, |__t| __t.name.clone()).unwrap(), type_supplier.clone(), compat::read(__this, |__t| __t.color_components.clone()), compat::read(__this, |__t| __t.alpha), ({ let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) }), compat::read(__this, |__t| __t.through_walls));
    }

    /// OrderedNamedWaypoint.withTypeSupplier as the dispatch of Waypoint.withTypeSupplier
    pub fn with_type_supplier_dyn(__this: &compat::Shared<dyn Waypoint>, __p0: Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        Self::with_type_supplier_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_impl(__this: &compat::Shared<Self>) -> bool {
        return (<dyn Waypoint>::should_render_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) && __this.lock().borrow().relative_index.should_render());
    }

    /// OrderedNamedWaypoint.shouldRender as the dispatch of Waypoint.shouldRender
    pub fn should_render_dyn(__this: &compat::Shared<dyn Waypoint>) -> bool {
        Self::should_render_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public float[] getRenderColorComponents()
    pub fn get_render_color_components_impl(__this: &compat::Shared<Self>) -> Vec<f32> {
        return match compat::read(__this, |__t| __t.relative_index) {
            RelativeIndex::None => <dyn Waypoint>::get_render_color_components_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)),
            RelativeIndex::Previous => RED_COLOR_COMPONENTS.clone(),
            RelativeIndex::Current => WHITE_COLOR_COMPONENTS.clone(),
            RelativeIndex::Next => GREEN_COLOR_COMPONENTS.clone(),
            _ => unreachable!("switch expression: no case matched"),
        };
    }

    /// OrderedNamedWaypoint.getRenderColorComponents as the dispatch of Waypoint.getRenderColorComponents
    pub fn get_render_color_components_dyn(__this: &compat::Shared<dyn Waypoint>) -> Vec<f32> {
        Self::get_render_color_components_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_impl(__this: &compat::Shared<Self>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_body(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>), collector);
        let waypoints = &SkyblockerConfigManager::get().ui_and_visuals.waypoints;
        if waypoints.render_line && compat::read(__this, |__t| __t.relative_index) == RelativeIndex::Next && { let __f = compat::dispatch(__this, |__t| __t.should_render()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) } {
            // TODO(fabric2seer): `.getComponents(..)` on a Color
            //   java: float[] components = waypoints.lineColor.getComponents(FLOAT_ARRAY);
            let mut components: Vec<f32> = todo!("fabric2seer: the initialiser of `components` (TODO above)");
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_line_from_cursor()); __f(collector, &compat::read(__this, |__t| __t.center_pos.clone()).unwrap(), &components, components[(3) as usize], waypoints.line_width) });
        }
        if { let __f = compat::dispatch(__this, |__t| __t.should_render_name()); __f(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>)) } {
            let mut scale: f32 = (((RenderHelper::get_camera().position().distance_to(&compat::read(__this, |__t| __t.center_pos.clone()).unwrap())) as f32) / (10 as f32)).max((1 as f32));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_5()); __f(collector, &component::null_to_empty(Some(&(compat::read(__this, |__t| __t.index)).wrapping_add(1).to_string())), &{ let (__q0, __q1, __q2,) = ((0) as f64, (1) as f64, (0) as f64,); __this.lock().borrow().center_pos.as_ref().unwrap().add(__q0, __q1, __q2) }, scale, (((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1)) as f32) as f32, true) });
        }
    }

    /// OrderedNamedWaypoint.extractRendering as the dispatch of Renderable.extractRendering
    pub fn extract_rendering_dyn(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        Self::extract_rendering_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}

/// java: public enum RelativeIndex
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RelativeIndex {
    None,
    Previous,
    Current,
    Next,
}

impl RelativeIndex {
    pub const VALUES: [RelativeIndex; 4] = [RelativeIndex::None, RelativeIndex::Previous, RelativeIndex::Current, RelativeIndex::Next];

    /// java: values()
    pub fn values() -> Vec<RelativeIndex> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            RelativeIndex::None => "NONE",
            RelativeIndex::Previous => "PREVIOUS",
            RelativeIndex::Current => "CURRENT",
            RelativeIndex::Next => "NEXT",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> RelativeIndex {
        match name {
            "NONE" => RelativeIndex::None,
            "PREVIOUS" => RelativeIndex::Previous,
            "CURRENT" => RelativeIndex::Current,
            "NEXT" => RelativeIndex::Next,
            _ => panic!("IllegalArgumentException: No enum constant RelativeIndex.{name}"),
        }
    }

    /// java: public boolean shouldRender()
    pub fn should_render(&self) -> bool {
        return (*self != NONE);
    }

}

impl std::fmt::Display for RelativeIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
