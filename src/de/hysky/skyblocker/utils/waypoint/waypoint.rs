//! Ported from de.hysky.skyblocker.utils.waypoint.Waypoint by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::render::primitive::primitive_collector::PrimitiveCollector;
use crate::de::hysky::skyblocker::utils::render::renderable::Renderable;
use mcrust::com::mojang::serialization::codec::Codec;
use mcrust::com::mojang::serialization::codec::CodecRef;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::phys::aabb::AABB;

/// java: protected static final float DEFAULT_HIGHLIGHT_ALPHA = 0.5f;
pub const DEFAULT_HIGHLIGHT_ALPHA: f32 = 0.5f32;
/// java: protected static final float DEFAULT_LINE_WIDTH = 5f;
pub const DEFAULT_LINE_WIDTH: f32 = 5.0f32;
// TODO(fabric2seer): `from_enum` takes 2 argument(s) in the port (`values: Vec<E>, serialized_name: impl Fn(&E) -> String + Send + Sync + Copy + 'static`), Java passes 1
//   java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
/// java: public static final Codec<Type> CODEC = StringRepresentable.fromEnum(Type::values);
pub static TYPE_CODEC: compat::MainThread<CodecRef<Type>> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `CODEC` - see the TODO above"));

/// java: public class Waypoint implements Renderable
#[derive(Clone)]
pub struct WaypointBase {
    /// java: public final BlockPos pos;
    pub pos: Option<BlockPos>,
    /// java: private final transient AABB box;
    pub r#box: Option<AABB>,
    /// java: final Supplier<Type> typeSupplier;
    pub type_supplier: Option<Arc<dyn Fn() -> Type>>,
    /// java: public final float[] colorComponents;
    pub color_components: Vec<f32>,
    /// java: public final float alpha;
    pub alpha: f32,
    /// java: public final float lineWidth;
    pub line_width: f32,
    /// java: public final boolean throughWalls;
    pub through_walls: bool,
    /// java: private boolean enabled;
    pub enabled: bool,
}

impl WaypointBase {
    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new3_in(pos.clone(), r#type, color_components.clone(), DEFAULT_HIGHLIGHT_ALPHA, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents)
    pub fn new(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new_in(pos, r#type, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new2_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new_5_in(pos.clone(), type_supplier.clone(), color_components.clone(), DEFAULT_HIGHLIGHT_ALPHA, DEFAULT_LINE_WIDTH, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents)
    pub fn new2(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>) -> compat::Shared<Self> {
        Self::new2_in(pos, type_supplier, color_components, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents, float alpha) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new3_in<__R>(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, mut alpha: f32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new_5_in(pos.clone(), Arc::new(move || r#type), color_components.clone(), alpha, DEFAULT_LINE_WIDTH, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents, float alpha)
    pub fn new3(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, mut alpha: f32) -> compat::Shared<Self> {
        Self::new3_in(pos, r#type, color_components, alpha, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_5_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new_6_in(pos.clone(), type_supplier.clone(), color_components.clone(), alpha, line_width, true, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth)
    pub fn new_5(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32) -> compat::Shared<Self> {
        Self::new_5_in(pos, type_supplier, color_components, alpha, line_width, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents, boolean throughWalls) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new5_in<__R>(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, mut through_walls: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new6_in(pos.clone(), Arc::new(move || r#type), color_components.clone(), through_walls, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Type type, float[] colorComponents, boolean throughWalls)
    pub fn new5(mut pos: BlockPos, mut r#type: Type, mut color_components: Vec<f32>, mut through_walls: bool) -> compat::Shared<Self> {
        Self::new5_in(pos, r#type, color_components, through_walls, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, boolean throughWalls) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new6_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut through_walls: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new_6_in(pos.clone(), type_supplier.clone(), color_components.clone(), DEFAULT_HIGHLIGHT_ALPHA, DEFAULT_LINE_WIDTH, through_walls, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, boolean throughWalls)
    pub fn new6(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut through_walls: bool) -> compat::Shared<Self> {
        Self::new6_in(pos, type_supplier, color_components, through_walls, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth, boolean throughWalls) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_6_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let (__this, __r) = Self::new_7_in(pos.clone(), type_supplier.clone(), color_components.clone(), alpha, line_width, through_walls, true, __wrap);
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth, boolean throughWalls)
    pub fn new_6(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool) -> compat::Shared<Self> {
        Self::new_6_in(pos, type_supplier, color_components, alpha, line_width, through_walls, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth, boolean throughWalls, boolean enabled) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_7_in<__R>(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool, mut enabled: bool, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn Waypoint>, __R)) -> (compat::Shared<dyn Waypoint>, __R) {
        let __init_pos = Some(pos.clone());
        let this = Self {
            pos: __init_pos,
            r#box: None,
            type_supplier: None,
            color_components: Default::default(),
            alpha: 0.0,
            line_width: 0.0,
            through_walls: false,
            enabled: false,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        { let __v = Some(AABB::of_block_pos(&compat::read(&__this, |__t| __t.waypoint_base().pos.clone()).unwrap())); compat::write(&__this, |__t| __t.waypoint_base_mut().r#box = __v); }
        { let __v = Some(type_supplier.clone()); compat::write(&__this, |__t| __t.waypoint_base_mut().type_supplier = __v); }
        { let __v = color_components.clone(); compat::write(&__this, |__t| __t.waypoint_base_mut().color_components = __v); }
        { let __v = alpha; compat::write(&__this, |__t| __t.waypoint_base_mut().alpha = __v); }
        { let __v = line_width; compat::write(&__this, |__t| __t.waypoint_base_mut().line_width = __v); }
        { let __v = through_walls; compat::write(&__this, |__t| __t.waypoint_base_mut().through_walls = __v); }
        { let __v = enabled; compat::write(&__this, |__t| __t.waypoint_base_mut().enabled = __v); }
        (__this, __r)
    }

    /// java: public Waypoint(BlockPos pos, Supplier<Type> typeSupplier, float[] colorComponents, float alpha, float lineWidth, boolean throughWalls, boolean enabled)
    pub fn new_7(mut pos: BlockPos, type_supplier: Arc<dyn Fn() -> Type>, mut color_components: Vec<f32>, mut alpha: f32, mut line_width: f32, mut through_walls: bool, mut enabled: bool) -> compat::Shared<Self> {
        Self::new_7_in(pos, type_supplier, color_components, alpha, line_width, through_walls, enabled, |this| {
            let __h = compat::shared(this);
            ((__h.clone() as compat::Shared<dyn Waypoint>), __h)
        }).1
    }

}

/// java: the virtual methods of Waypoint (implemented by WaypointBase and every subclass)
pub trait Waypoint: Renderable {
    fn waypoint_base(&self) -> &WaypointBase;
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase;

    /// java: public Waypoint withX(int x) (dispatch: the body for this class)
    fn with_x(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_x_body
    }

    /// java: public Waypoint withY(int y) (dispatch: the body for this class)
    fn with_y(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_y_body
    }

    /// java: public Waypoint withZ(int z) (dispatch: the body for this class)
    fn with_z(&self) -> fn(&compat::Shared<dyn Waypoint>, i32) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_z_body
    }

    /// java: public Waypoint withColor(float[] colorComponents, float alpha) (dispatch: the body for this class)
    fn with_color(&self) -> fn(&compat::Shared<dyn Waypoint>, &Vec<f32>, f32) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_color_body
    }

    /// java: public Waypoint withThroughWalls(boolean throughWalls) (dispatch: the body for this class)
    fn with_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_through_walls_body
    }

    /// java: public Waypoint withTypeSupplier(Supplier<Type> typeSupplier) (dispatch: the body for this class)
    fn with_type_supplier(&self) -> fn(&compat::Shared<dyn Waypoint>, Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        <dyn Waypoint>::with_type_supplier_body
    }

    /// java: public boolean shouldRender() (dispatch: the body for this class)
    fn should_render(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        <dyn Waypoint>::should_render_body
    }

    /// java: public void setFound() (dispatch: the body for this class)
    fn set_found(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn Waypoint>::set_found_body
    }

    /// java: public void setMissing() (dispatch: the body for this class)
    fn set_missing(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn Waypoint>::set_missing_body
    }

    /// java: public void toggle() (dispatch: the body for this class)
    fn toggle(&self) -> fn(&compat::Shared<dyn Waypoint>) {
        <dyn Waypoint>::toggle_body
    }

    /// java: public final boolean isEnabled() (dispatch: the body for this class)
    fn is_enabled(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        <dyn Waypoint>::is_enabled_body
    }

    /// java: public final void setEnabled(boolean enabled) (dispatch: the body for this class)
    fn set_enabled(&self) -> fn(&compat::Shared<dyn Waypoint>, bool) {
        <dyn Waypoint>::set_enabled_body
    }

    /// java: public Type getRenderType() (dispatch: the body for this class)
    fn get_render_type(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Type {
        <dyn Waypoint>::get_render_type_body
    }

    /// java: protected AABB getRenderBox() (dispatch: the body for this class)
    fn get_render_box(&self) -> fn(&compat::Shared<dyn Waypoint>) -> AABB {
        <dyn Waypoint>::get_render_box_body
    }

    /// java: public float[] getRenderColorComponents() (dispatch: the body for this class)
    fn get_render_color_components(&self) -> fn(&compat::Shared<dyn Waypoint>) -> Vec<f32> {
        <dyn Waypoint>::get_render_color_components_body
    }

    /// java: public boolean shouldRenderThroughWalls() (dispatch: the body for this class)
    fn should_render_through_walls(&self) -> fn(&compat::Shared<dyn Waypoint>) -> bool {
        <dyn Waypoint>::should_render_through_walls_body
    }

    /// java: @Override public int hashCode() (dispatch: the body for this class)
    fn hash_code(&self) -> fn(&compat::Shared<dyn Waypoint>) -> i32 {
        <dyn Waypoint>::hash_code_body
    }

    /// java: @Override public boolean equals(Object obj) (dispatch: the body for this class)
    fn equals(&self) -> fn(&compat::Shared<dyn Waypoint>, &compat::Object) -> bool {
        <dyn Waypoint>::equals_body
    }

}

impl dyn Waypoint {
    /// java: public Waypoint withX(int x)
    pub fn with_x_body(__this: &compat::Shared<dyn Waypoint>, mut x: i32) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(BlockPos::new(x, __this.lock().borrow().waypoint_base().pos.as_ref().unwrap().get_y(), __this.lock().borrow().waypoint_base().pos.as_ref().unwrap().get_z()), compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.waypoint_base().alpha), compat::read(__this, |__t| __t.waypoint_base().line_width), compat::read(__this, |__t| __t.waypoint_base().through_walls), compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public Waypoint withY(int y)
    pub fn with_y_body(__this: &compat::Shared<dyn Waypoint>, mut y: i32) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(__this.lock().borrow().waypoint_base().pos.as_ref().unwrap().at_y(y), compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.waypoint_base().alpha), compat::read(__this, |__t| __t.waypoint_base().line_width), compat::read(__this, |__t| __t.waypoint_base().through_walls), compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public Waypoint withZ(int z)
    pub fn with_z_body(__this: &compat::Shared<dyn Waypoint>, mut z: i32) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(BlockPos::new(__this.lock().borrow().waypoint_base().pos.as_ref().unwrap().get_x(), __this.lock().borrow().waypoint_base().pos.as_ref().unwrap().get_y(), z), compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.waypoint_base().alpha), compat::read(__this, |__t| __t.waypoint_base().line_width), compat::read(__this, |__t| __t.waypoint_base().through_walls), compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public Waypoint withColor(float[] colorComponents, float alpha)
    pub fn with_color_body(__this: &compat::Shared<dyn Waypoint>, color_components: &Vec<f32>, mut alpha: f32) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(compat::read(__this, |__t| __t.waypoint_base().pos.clone()).unwrap(), compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).clone().unwrap(), color_components.clone(), alpha, compat::read(__this, |__t| __t.waypoint_base().line_width), compat::read(__this, |__t| __t.waypoint_base().through_walls), compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public Waypoint withThroughWalls(boolean throughWalls)
    pub fn with_through_walls_body(__this: &compat::Shared<dyn Waypoint>, mut through_walls: bool) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(compat::read(__this, |__t| __t.waypoint_base().pos.clone()).unwrap(), compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).clone().unwrap(), compat::read(__this, |__t| __t.waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.waypoint_base().alpha), compat::read(__this, |__t| __t.waypoint_base().line_width), through_walls, compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public Waypoint withTypeSupplier(Supplier<Type> typeSupplier)
    pub fn with_type_supplier_body(__this: &compat::Shared<dyn Waypoint>, type_supplier: Arc<dyn Fn() -> Type>) -> compat::Shared<dyn Waypoint> {
        return (WaypointBase::new_7(compat::read(__this, |__t| __t.waypoint_base().pos.clone()).unwrap(), type_supplier.clone(), compat::read(__this, |__t| __t.waypoint_base().color_components.clone()), compat::read(__this, |__t| __t.waypoint_base().alpha), compat::read(__this, |__t| __t.waypoint_base().line_width), compat::read(__this, |__t| __t.waypoint_base().through_walls), compat::read(__this, |__t| __t.waypoint_base().enabled)) as compat::Shared<dyn Waypoint>);
    }

    /// java: public boolean shouldRender()
    pub fn should_render_body(__this: &compat::Shared<dyn Waypoint>) -> bool {
        return compat::read(__this, |__t| __t.waypoint_base().enabled);
    }

    /// java: public void setFound()
    pub fn set_found_body(__this: &compat::Shared<dyn Waypoint>) {
        { let __v = false; compat::write(__this, |__t| __t.waypoint_base_mut().enabled = __v); }
    }

    /// java: public void setMissing()
    pub fn set_missing_body(__this: &compat::Shared<dyn Waypoint>) {
        { let __v = true; compat::write(__this, |__t| __t.waypoint_base_mut().enabled = __v); }
    }

    /// java: public void toggle()
    pub fn toggle_body(__this: &compat::Shared<dyn Waypoint>) {
        if compat::read(__this, |__t| __t.waypoint_base().enabled) {
            ({ let __f = compat::dispatch(__this, |__t| __t.set_found()); __f(__this) });
        } else {
            ({ let __f = compat::dispatch(__this, |__t| __t.set_missing()); __f(__this) });
        }
    }

    /// java: public final boolean isEnabled()
    pub fn is_enabled_body(__this: &compat::Shared<dyn Waypoint>) -> bool {
        return compat::read(__this, |__t| __t.waypoint_base().enabled);
    }

    /// java: public final void setEnabled(boolean enabled)
    pub fn set_enabled_body(__this: &compat::Shared<dyn Waypoint>, mut enabled: bool) {
        { let __v = enabled; compat::write(__this, |__t| __t.waypoint_base_mut().enabled = __v); }
    }

    /// java: public Type getRenderType()
    pub fn get_render_type_body(__this: &compat::Shared<dyn Waypoint>) -> Type {
        return ((compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).unwrap()))();
    }

    /// java: protected AABB getRenderBox()
    pub fn get_render_box_body(__this: &compat::Shared<dyn Waypoint>) -> AABB {
        return compat::read(__this, |__t| __t.waypoint_base().r#box.clone()).unwrap();
    }

    /// java: public float[] getRenderColorComponents()
    pub fn get_render_color_components_body(__this: &compat::Shared<dyn Waypoint>) -> Vec<f32> {
        return compat::read(__this, |__t| __t.waypoint_base().color_components.clone());
    }

    /// java: public boolean shouldRenderThroughWalls()
    pub fn should_render_through_walls_body(__this: &compat::Shared<dyn Waypoint>) -> bool {
        return compat::read(__this, |__t| __t.waypoint_base().through_walls);
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector)
    pub fn extract_rendering_body(__this: &compat::Shared<dyn Waypoint>, collector: &compat::Shared<dyn PrimitiveCollector>) {
        if !({ let __f = compat::dispatch(__this, |__t| __t.should_render()); __f(__this) }) {
            return;
        }
        let mut r#box: AABB = ({ let __f = compat::dispatch(__this, |__t| __t.get_render_box()); __f(__this) });
        let mut color_components: Vec<f32> = ({ let __f = compat::dispatch(__this, |__t| __t.get_render_color_components()); __f(__this) });
        let mut through_walls: bool = ({ let __f = compat::dispatch(__this, |__t| __t.should_render_through_walls()); __f(__this) });
        match ({ let __f = compat::dispatch(__this, |__t| __t.get_render_type()); __f(__this) }) {
            Type::Waypoint => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().alpha), through_walls) });
            }
            Type::OutlinedWaypoint => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_with_beacon_beam_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().alpha), through_walls) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().line_width), through_walls) });
            }
            Type::Highlight => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().alpha), through_walls) });
            }
            Type::OutlinedHighlight => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_filled_box_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().alpha), through_walls) });
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().line_width), through_walls) });
            }
            Type::Outline => {
                ({ let __f = compat::dispatch(collector, |__t| __t.submit_outlined_box_aabb_float_float_boolean()); __f(collector, &r#box, &color_components, compat::read(__this, |__t| __t.waypoint_base().line_width), through_walls) });
            }
            _ => {}
        }
    }

    /// Waypoint.extractRendering as the dispatch of Renderable.extractRendering for the concrete class `__C`
    pub fn extract_rendering_from<__C: Waypoint + 'static>(__this: &compat::Shared<dyn Renderable>, __p0: &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Waypoint>), __p0)
    }

    /// java: @Override public int hashCode()
    pub fn hash_code_body(__this: &compat::Shared<dyn Waypoint>) -> i32 {
        return compat::hash_code(&(&compat::read(__this, |__t| __t.waypoint_base().pos.clone()).unwrap(), &((compat::read(__this, |__t| __t.waypoint_base().type_supplier.clone()).unwrap()))(), &compat::hash_code(&compat::read(__this, |__t| __t.waypoint_base().color_components.clone())), &compat::read(__this, |__t| __t.waypoint_base().alpha), &compat::read(__this, |__t| __t.waypoint_base().line_width), &compat::read(__this, |__t| __t.waypoint_base().through_walls), &compat::read(__this, |__t| __t.waypoint_base().enabled),));
    }

    /// java: @Override public boolean equals(Object obj)
    pub fn equals_body(__this: &compat::Shared<dyn Waypoint>, obj: &compat::Object) -> bool {
        // TODO(fabric2seer): super.equals() of java.lang.Object
        //   java: return super.equals(obj) || obj instanceof Waypoint other && pos.equals(other.pos) && typeSupplier.get() == other.typeSupplier.get() && Arrays.equals(colorComponents, other.colorComponents) && alpha == other.alpha && lineWidth == other.lineWidth && throughWalls == other.throughWalls && enabled == other.enabled;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl Waypoint for WaypointBase {
    fn waypoint_base(&self) -> &WaypointBase {
        &self
    }
    fn waypoint_base_mut(&mut self) -> &mut WaypointBase {
        &mut self
    }

}

impl Renderable for WaypointBase {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void extractRendering(PrimitiveCollector collector) (dispatch: the body for this class)
    fn extract_rendering(&self) -> fn(&compat::Shared<dyn Renderable>, &compat::Shared<dyn PrimitiveCollector>) {
        <dyn Waypoint>::extract_rendering_from::<Self>
    }

}

/// java: public enum Type implements StringRepresentable
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Type {
    Waypoint,
    OutlinedWaypoint,
    Highlight,
    OutlinedHighlight,
    Outline,
}

impl Type {
    pub const VALUES: [Type; 5] = [Type::Waypoint, Type::OutlinedWaypoint, Type::Highlight, Type::OutlinedHighlight, Type::Outline];

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
            Type::Waypoint => "WAYPOINT",
            Type::OutlinedWaypoint => "OUTLINED_WAYPOINT",
            Type::Highlight => "HIGHLIGHT",
            Type::OutlinedHighlight => "OUTLINED_HIGHLIGHT",
            Type::Outline => "OUTLINE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> Type {
        match name {
            "WAYPOINT" => Type::Waypoint,
            "OUTLINED_WAYPOINT" => Type::OutlinedWaypoint,
            "HIGHLIGHT" => Type::Highlight,
            "OUTLINED_HIGHLIGHT" => Type::OutlinedHighlight,
            "OUTLINE" => Type::Outline,
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
        //   java: return I18n.get("skyblocker.waypoints.type." + name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public Type withoutBeacon()
    pub fn without_beacon(&self) -> Type {
        return match *self {
            Type::Waypoint => HIGHLIGHT,
            Type::OutlinedWaypoint => OUTLINED_HIGHLIGHT,
            _ => *self,
        };
    }

}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Type::to_string(*self))
    }
}
