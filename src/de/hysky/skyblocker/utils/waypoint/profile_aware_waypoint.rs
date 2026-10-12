//! Ported from de.hysky.skyblocker.utils.waypoint.ProfileAwareWaypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use crate::de::hysky::skyblocker::utils::utils::Utils;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Type;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::Waypoint;
use crate::de::hysky::skyblocker::utils::waypoint::waypoint::WaypointBase;
use mcrust::net::minecraft::core::block_pos::BlockPos;

/// java: public class ProfileAwareWaypoint extends Waypoint
#[derive(Clone)]
pub struct ProfileAwareWaypointBase {
    /// java: extends Waypoint
    pub base: WaypointBase,
    /// java: public final Set<String> foundProfiles = new HashSet<>();
    pub found_profiles: std::collections::HashSet<String>,
    /// java: private final float[] missingColor;
    pub missing_color: Vec<f32>,
    /// java: private final float[] foundColor;
    pub found_color: Vec<f32>,
}

impl std::ops::Deref for ProfileAwareWaypointBase {
    type Target = WaypointBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ProfileAwareWaypointBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ProfileAwareWaypointBase {
    /// java: public ProfileAwareWaypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] missingColor, float[] foundColor) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut missing_color: Vec<f32>, mut found_color: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn ProfileAwareWaypoint>, __R)) -> (compat::Shared<dyn ProfileAwareWaypoint>, __R) {
        let __init_missing_color = missing_color.clone();
        let __init_found_color = found_color.clone();
        let (_, (__this, __r)) = WaypointBase::new2_in(pos.clone(), type_supplier.clone(), /* java: (float[]) null - this class answers its own colours (getRenderColorComponents) */ Vec::new(), move |__base| {
            let this = Self {
                base: __base,
                found_profiles: std::collections::HashSet::new(),
                missing_color: __init_missing_color,
                found_color: __init_found_color,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), (__h, __r))
        });
        (__this, __r)
    }

    /// java: public ProfileAwareWaypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] missingColor, float[] foundColor)
    pub fn new(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut missing_color: Vec<f32>, mut found_color: Vec<f32>) -> compat::Shared<Self> {
        Self::new_in(pos, type_supplier, missing_color, found_color, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn ProfileAwareWaypoint>), __h)
        }).1
    }

}

/// java: the virtual methods of ProfileAwareWaypoint (implemented by ProfileAwareWaypointBase and every subclass)
pub trait ProfileAwareWaypoint: Waypoint {
    fn profile_aware_waypoint_base(&self) -> &ProfileAwareWaypointBase;
    fn profile_aware_waypoint_base_mut(&mut self) -> &mut ProfileAwareWaypointBase;

    /// java: public void setFound(String profile) (dispatch: the body for this class)
    fn set_found_1(&self) -> fn(&compat::Shared<dyn ProfileAwareWaypoint>, String) {
        <dyn ProfileAwareWaypoint>::set_found_1_body
    }

}

impl dyn ProfileAwareWaypoint {
    /// java: @Override public boolean shouldRender()
    pub fn should_render_body(__this: &compat::Shared<dyn ProfileAwareWaypoint>) -> bool {
        return !(__this.lock().borrow().profile_aware_waypoint_base().found_profiles.contains(&Utils::get_profile()));
    }

    /// ProfileAwareWaypoint.shouldRender as the dispatch of Waypoint.shouldRender for the concrete class `__C`
    pub fn should_render_from<__C: ProfileAwareWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) -> bool {
        <dyn ProfileAwareWaypoint>::should_render_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>))
    }

    /// java: @Override public void setFound()
    pub fn set_found_body(__this: &compat::Shared<dyn ProfileAwareWaypoint>) {
        let __h0 = Utils::get_profile();
        __this.lock().borrow_mut().profile_aware_waypoint_base_mut().found_profiles.insert(__h0);
    }

    /// ProfileAwareWaypoint.setFound as the dispatch of Waypoint.setFound for the concrete class `__C`
    pub fn set_found_from<__C: ProfileAwareWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_found_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>))
    }

    /// java: public void setFound(String profile)
    pub fn set_found_1_body(__this: &compat::Shared<dyn ProfileAwareWaypoint>, mut profile: String) {
        let __h0 = profile.clone();
        __this.lock().borrow_mut().profile_aware_waypoint_base_mut().found_profiles.insert(__h0);
    }

    /// java: @Override public void setMissing()
    pub fn set_missing_body(__this: &compat::Shared<dyn ProfileAwareWaypoint>) {
        let __h0 = &Utils::get_profile();
        __this.lock().borrow_mut().profile_aware_waypoint_base_mut().found_profiles.remove(__h0);
    }

    /// ProfileAwareWaypoint.setMissing as the dispatch of Waypoint.setMissing for the concrete class `__C`
    pub fn set_missing_from<__C: ProfileAwareWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_missing_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>))
    }

    /// java: @Override public float[] getRenderColorComponents()
    pub fn get_render_color_components_body(__this: &compat::Shared<dyn ProfileAwareWaypoint>) -> Vec<f32> {
        return (if __this.lock().borrow().profile_aware_waypoint_base().found_profiles.contains(&Utils::get_profile()) { compat::read(__this, |__t| __t.profile_aware_waypoint_base().found_color.clone()) } else { compat::read(__this, |__t| __t.profile_aware_waypoint_base().missing_color.clone()) });
    }

    /// ProfileAwareWaypoint.getRenderColorComponents as the dispatch of Waypoint.getRenderColorComponents for the concrete class `__C`
    pub fn get_render_color_components_from<__C: ProfileAwareWaypoint + 'static>(__this: &compat::Shared<dyn Waypoint>) -> Vec<f32> {
        <dyn ProfileAwareWaypoint>::get_render_color_components_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn ProfileAwareWaypoint>))
    }

}

impl ProfileAwareWaypoint for ProfileAwareWaypointBase {
    fn profile_aware_waypoint_base(&self) -> &ProfileAwareWaypointBase {
        self
    }
    fn profile_aware_waypoint_base_mut(&mut self) -> &mut ProfileAwareWaypointBase {
        self
    }

}

impl Waypoint for ProfileAwareWaypointBase {
    fn waypoint_base(&self) -> &WaypointBase {
        &self.base
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self.base
    }

    /// java: @Override public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        <dyn ProfileAwareWaypoint>::should_render_from::<Self>
    }

    /// java: @Override public void setFound() (dispatch: the body for this class)
    fn set_found(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_found_from::<Self>
    }

    /// java: @Override public void setMissing() (dispatch: the body for this class)
    fn set_missing(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn ProfileAwareWaypoint>::set_missing_from::<Self>
    }

    /// java: @Override public float[] getRenderColorComponents() (dispatch: the body for this class)
    fn get_render_color_components(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Vec<f32> {
        <dyn ProfileAwareWaypoint>::get_render_color_components_from::<Self>
    }

}

impl Renderable for ProfileAwareWaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}
