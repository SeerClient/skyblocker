//! Ported from de.hysky.skyblocker.utils.waypoint.DistancedNamedWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::render_helper::RenderHelper;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypoint;
use crate::de::hysky::skyblocker::utils::waypoint::named_waypoint::NamedWaypointBase;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DistancedNamedWaypoint extends NamedWaypoint
#[derive(Clone)]
pub struct DistancedNamedWaypointBase {
    /// java: extends NamedWaypoint
    pub base: NamedWaypointBase,
}

impl std::ops::Deref for DistancedNamedWaypointBase {
    type Target = NamedWaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DistancedNamedWaypointBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DistancedNamedWaypointBase {
    /// java: public DistancedNamedWaypoint(BlockPos pos, Component name, float[] colorComponents, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DistancedNamedWaypoint>, __R)) -> (compat::Shared<dyn DistancedNamedWaypoint>, __R) {
        let (_, (__this, __r)) = NamedWaypointBase::new5_in(pos.clone(), name.clone(), color_components.clone(), enabled, move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public DistancedNamedWaypoint(BlockPos pos, Component name, float[] colorComponents, boolean enabled)
    pub fn new(mut pos: BlockPos, mut name: MutableComponent, mut color_components: Vec<f32>, mut enabled: bool) -> compat::Shared<Self> {
        Self::new_in(pos, name, color_components, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn DistancedNamedWaypoint>), __h)
        }).1
    }

    /// java: public DistancedNamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DistancedNamedWaypoint>, __R)) -> (compat::Shared<dyn DistancedNamedWaypoint>, __R) {
        let (_, (__this, __r)) = NamedWaypointBase::new7_in(pos.clone(), name.clone(), type_supplier.clone(), color_components.clone(), move |__base| {
            let this = Self {
                base: __base,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn NamedWaypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public DistancedNamedWaypoint(BlockPos pos, Component name, Supplier<Type> typeSupplier, float[] colorComponents)
    pub fn new2(mut pos: BlockPos, mut name: MutableComponent, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new2_in(pos, name, type_supplier, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn DistancedNamedWaypoint>), __h)
        }).1
    }

}

/// java: the virtual methods of DistancedNamedWaypoint (implemented by DistancedNamedWaypointBase and every subclass)
pub trait DistancedNamedWaypoint: NamedWaypoint {
    fn distanced_named_waypoint_base(&self) -> &DistancedNamedWaypointBase;
    fn distanced_named_waypoint_base_mut(&mut self) -> &mut DistancedNamedWaypointBase;

    /// java: public boolean shouldRenderDistance() (dispatch: the body for this class)
    fn should_render_distance(&self) -> fn(&compat::Shared<dyn DistancedNamedWaypoint>) -> bool {
        <dyn DistancedNamedWaypoint>::should_render_distance_body
    }

}

impl dyn DistancedNamedWaypoint {
    /// java: public boolean shouldRenderDistance()
    pub fn should_render_distance_body(__this: &compat::Shared<dyn DistancedNamedWaypoint>) -> bool {
        return ({ let __f = compat::dispatch(__this, |__t| __t.should_render_name()); __f(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>)) });
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_body(__this: &compat::Shared<dyn DistancedNamedWaypoint>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn NamedWaypoint>::extract_rendering_body(&compat::view(__this, |a| a as compat::Shared<dyn NamedWaypoint>), collector);
        if { let __f = compat::dispatch(__this, |__t| __t.should_render_distance()); __f(__this) } {
            let mut distance: f64 = RenderHelper::get_camera().position().distance_to(&compat::read(__this, |__t| __t.distanced_named_waypoint_base().center_pos.clone()).unwrap());
            let mut scale: f32 = (((distance) as f32) / (10 as f32)).max((1 as f32));
            ({ let __f = compat::dispatch(collector, |__t| __t.submit_text_5()); __f(collector, &(component::literal(&format!("{}m", compat::java_round_f64(distance)))).with_style_format(ChatFormatting::Yellow), &{ let (__q0, __q1, __q2,) = ((0) as f64, (1) as f64, (0) as f64,); __this.lock().borrow().distanced_named_waypoint_base().center_pos.as_ref().unwrap().add(__q0, __q1, __q2) }, scale, (((mcrust::net::minecraft::client::gui::font::LINE_HEIGHT).wrapping_add(1)) as f32) as f32, true) });
        }
    }

    /// DistancedNamedWaypoint.extractRendering as the dispatch of Renderable.extractRendering for the concrete class `__C`
    pub fn extract_rendering_from<__C: DistancedNamedWaypoint + 'static>(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn DistancedNamedWaypoint>::extract_rendering_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn DistancedNamedWaypoint>), __p0)
    }

}

impl DistancedNamedWaypoint for DistancedNamedWaypointBase {
    fn distanced_named_waypoint_base(&self) -> &DistancedNamedWaypointBase {
        &self
    }
    fn distanced_named_waypoint_base_mut(&mut self) -> &mut DistancedNamedWaypointBase {
        &mut self
    }

}

impl NamedWaypoint for DistancedNamedWaypointBase {
    fn named_waypoint_base(&self) -> &NamedWaypointBase {
        &self.base
    }
    fn named_waypoint_base_mut(&mut self) -> &mut NamedWaypointBase {
        &mut self.base
    }

}

impl Waypoint for DistancedNamedWaypointBase {
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

impl Renderable for DistancedNamedWaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn DistancedNamedWaypoint>::extract_rendering_from::<Self>
    }

}
