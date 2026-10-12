//! Ported from de.hysky.skyblocker.utils.waypoint.SeenWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::tickable::Tickable;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::level::clip_context::ClipContext;
use mcrust::net::minecraft::world::phys::block_hit_result::BlockHitResult;
use mcrust::net::minecraft::world::phys::hit_result::HitResult;
use mcrust::net::minecraft::world::phys::vec3;
use mcrust::net::minecraft::world::level::collision_getter::CollisionGetter as _;

/// java: public class SeenWaypoint extends Waypoint implements Tickable
#[derive(Clone)]
pub struct SeenWaypointBase {
    /// java: extends Waypoint
    pub base: WaypointBase,
    /// java: private boolean seen = false;
    pub seen: bool,
}

impl std::ops::Deref for SeenWaypointBase {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for SeenWaypointBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl SeenWaypointBase {
    /// java: public SeenWaypoint(BlockPos pos, Type type, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SeenWaypoint>, __R)) -> (compat::Shared<dyn SeenWaypoint>, __R) {
        let (_, (__this, __r)) = WaypointBase::new5_in(pos.clone(), r#type, color_components.clone(), false, move |__base| {
            let this = Self {
                base: __base,
                seen: false,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public SeenWaypoint(BlockPos pos, Type type, float[] colorComponents)
    pub fn new(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new_in(pos, r#type, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SeenWaypoint>), __h)
        }).1
    }

    /// java: public SeenWaypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn SeenWaypoint>, __R)) -> (compat::Shared<dyn SeenWaypoint>, __R) {
        let (_, (__this, __r)) = WaypointBase::new6_in(pos.clone(), type_supplier.clone(), color_components.clone(), false, move |__base| {
            let this = Self {
                base: __base,
                seen: false,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public SeenWaypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents)
    pub fn new2(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new2_in(pos, type_supplier, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn SeenWaypoint>), __h)
        }).1
    }

}

/// java: the virtual methods of SeenWaypoint (implemented by SeenWaypointBase and every subclass)
pub trait SeenWaypoint: Waypoint + Tickable {
    fn seen_waypoint_base(&self) -> &SeenWaypointBase;
    fn seen_waypoint_base_mut(&mut self) -> &mut SeenWaypointBase;

    /// java: public boolean isSeen() (dispatch: the body for this class)
    fn is_seen(&self) -> fn(&compat::Shared<dyn SeenWaypoint>) -> bool {
        <dyn SeenWaypoint>::is_seen_body
    }

}

impl dyn SeenWaypoint {
    /// java: public boolean isSeen()
    pub fn is_seen_body(__this: &compat::Shared<dyn SeenWaypoint>) -> bool {
        return compat::read(__this, |__t| __t.seen_waypoint_base().seen);
    }

    /// java: @Override public Type getRenderType()
    pub fn get_render_type_body(__this: &compat::Shared<dyn SeenWaypoint>) -> Type {
        return (if compat::read(__this, |__t| __t.seen_waypoint_base().seen) { <dyn Waypoint>::get_render_type_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) } else { (<dyn Waypoint>::get_render_type_body(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>))).without_beacon() });
    }

    /// SeenWaypoint.getRenderType as the dispatch of Waypoint.getRenderType for the concrete class `__C`
    pub fn get_render_type_from<__C: SeenWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) -> Type {
        <dyn SeenWaypoint>::get_render_type_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SeenWaypoint>))
    }

    /// java: @Override public boolean shouldRender()
    pub fn should_render_body(__this: &compat::Shared<dyn SeenWaypoint>) -> bool {
        return compat::read(__this, |__t| __t.seen_waypoint_base().seen);
    }

    /// SeenWaypoint.shouldRender as the dispatch of Waypoint.shouldRender for the concrete class `__C`
    pub fn should_render_from<__C: SeenWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) -> bool {
        <dyn SeenWaypoint>::should_render_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SeenWaypoint>))
    }

    /// java: @Override public void tick(Minecraft client)
    pub fn tick_body(__this: &compat::Shared<dyn SeenWaypoint>, client: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        if !(compat::read(__this, |__t| __t.seen_waypoint_base().seen)) && { let __f = compat::dispatch(__this, |__t| __t.is_enabled()); __f(&compat::view(__this, |a| a as compat::Shared<dyn Waypoint>)) } && mcrust::mcrust::modding::game::level().is_some() && mcrust::mcrust::modding::game::with_player(|_| ()).is_some() && mcrust::mcrust::modding::game::level().unwrap().get_chunk_for_collisions((__this.lock().borrow().seen_waypoint_base().pos.as_ref().unwrap().get_x()).wrapping_shr((4) as u32), (__this.lock().borrow().seen_waypoint_base().pos.as_ref().unwrap().get_z()).wrapping_shr((4) as u32)).is_some() {
            // TODO(fabric2seer): field `OUTLINE` of a value of unknown type
            //   java: BlockHitResult blockHitResult = client.level.clip(new ClipContext(client.player.getEyePosition(), Vec3.atCenterOf(pos), ClipContext.Block.OUTLINE, ClipContext.Fluid.NONE, client.player));
            let mut block_hit_result: BlockHitResult = todo!("fabric2seer: the initialiser of `blockHitResult` (TODO above)");
            // TODO(fabric2seer): static `HitResult.Type` has no Rust counterpart in the port's HitResult file
            // TODO(fabric2seer): the condition: field `MISS` of a value of unknown type
            //   java: blockHitResult.getType() == HitResult.Type.MISS || blockHitResult.getBlockPos().equals(pos)
            if todo!("fabric2seer: the condition (TODO above)") {
                { let __v = true; compat::write(__this, |__t| __t.seen_waypoint_base_mut().seen = __v); }
            }
        }
    }

    /// SeenWaypoint.tick as the dispatch of Tickable.tick for the concrete class `__C`
    pub fn tick_from<__C: SeenWaypoint + 'static>(__this: &compat::Shared<dyn Tickable>, __p0: &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn SeenWaypoint>::tick_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn SeenWaypoint>), __p0)
    }

}

impl SeenWaypoint for SeenWaypointBase {
    fn seen_waypoint_base(&self) -> &SeenWaypointBase {
        self
    }
    fn seen_waypoint_base_mut(&mut self) -> &mut SeenWaypointBase {
        self
    }

}

impl Waypoint for SeenWaypointBase {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
    }

    /// java: @Override public Type getRenderType() (dispatch: the body for this class)
    fn get_render_type(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Type {
        <dyn SeenWaypoint>::get_render_type_from::<Self>
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        <dyn SeenWaypoint>::should_render_from::<Self>
    }

}

impl Renderable for SeenWaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

impl Tickable for SeenWaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void tick(Minecraft client) (dispatch: the body for this class)
    fn tick(&self) -> fn(&compat::Shared<dyn Tickable>, &mut mcrust::net::minecraft::client::minecraft::Minecraft) {
        <dyn SeenWaypoint>::tick_from::<Self>
    }

}
