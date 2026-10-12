//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.blockpos.DefaultClientPosArgument by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_pos_argument::ClientPosArgument;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::net::minecraft::commands::arguments::coordinates::world_coordinate::WorldCoordinate;
use mcrust::net::minecraft::world::phys::vec2::Vec2;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public class DefaultClientPosArgument implements ClientPosArgument
#[derive(Clone)]
pub struct DefaultClientPosArgument {
    /// java: private final WorldCoordinate x;
    pub x: WorldCoordinate,
    /// java: private final WorldCoordinate y;
    pub y: WorldCoordinate,
    /// java: private final WorldCoordinate z;
    pub z: WorldCoordinate,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl DefaultClientPosArgument {
    /// java: public DefaultClientPosArgument(WorldCoordinate x, WorldCoordinate y, WorldCoordinate z)
    pub fn new(mut x: WorldCoordinate, mut y: WorldCoordinate, mut z: WorldCoordinate) -> compat::Shared<Self> {
        let __init_x = x.clone();
        let __init_y = y.clone();
        let __init_z = z.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            x: __init_x,
            y: __init_y,
            z: __init_z,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public static DefaultClientPosArgument parse(StringReader reader) throws CommandSyntaxException
    pub fn parse(reader: &mut StringReader) -> compat::Shared<DefaultClientPosArgument> {
        let mut i: i32 = reader.get_cursor();
        let mut coordinate_argument: WorldCoordinate = WorldCoordinate::parse_int(reader).unwrap();
        if reader.can_read() && reader.peek() == ' ' {
            reader.skip();
            let mut coordinate_argument2: WorldCoordinate = WorldCoordinate::parse_int(reader).unwrap();
            if reader.can_read() && reader.peek() == ' ' {
                reader.skip();
                let mut coordinate_argument3: WorldCoordinate = WorldCoordinate::parse_int(reader).unwrap();
                return DefaultClientPosArgument::new(coordinate_argument.clone(), coordinate_argument2.clone(), coordinate_argument3.clone());
            } else {
                reader.set_cursor(i);
                panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
            }
        } else {
            reader.set_cursor(i);
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
        }
    }

    /// java: public static DefaultClientPosArgument parse(StringReader reader, boolean centerIntegers) throws CommandSyntaxException
    pub fn parse_2(reader: &mut StringReader, mut center_integers: bool) -> compat::Shared<DefaultClientPosArgument> {
        let mut i: i32 = reader.get_cursor();
        let mut coordinate_argument: WorldCoordinate = WorldCoordinate::parse_double(reader, center_integers).unwrap();
        if reader.can_read() && reader.peek() == ' ' {
            reader.skip();
            let mut coordinate_argument2: WorldCoordinate = WorldCoordinate::parse_double(reader, false).unwrap();
            if reader.can_read() && reader.peek() == ' ' {
                reader.skip();
                let mut coordinate_argument3: WorldCoordinate = WorldCoordinate::parse_double(reader, center_integers).unwrap();
                return DefaultClientPosArgument::new(coordinate_argument.clone(), coordinate_argument2.clone(), coordinate_argument3.clone());
            } else {
                reader.set_cursor(i);
                panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
            }
        } else {
            reader.set_cursor(i);
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
        }
    }

    /// java: public static DefaultClientPosArgument absolute(double x, double y, double z)
    pub fn absolute(mut x: f64, mut y: f64, mut z: f64) -> compat::Shared<DefaultClientPosArgument> {
        return DefaultClientPosArgument::new(WorldCoordinate::new(false, x), WorldCoordinate::new(false, y), WorldCoordinate::new(false, z));
    }

    /// java: public static DefaultClientPosArgument absolute(Vec2 vec)
    pub fn absolute_1(vec: &Vec2) -> compat::Shared<DefaultClientPosArgument> {
        return DefaultClientPosArgument::new(WorldCoordinate::new(false, vec.x), WorldCoordinate::new(false, vec.y), WorldCoordinate::new(true, 0.0));
    }

    /// java: public static DefaultClientPosArgument zero()
    pub fn zero() -> compat::Shared<DefaultClientPosArgument> {
        return DefaultClientPosArgument::new(WorldCoordinate::new(true, 0.0), WorldCoordinate::new(true, 0.0), WorldCoordinate::new(true, 0.0));
    }

    /// java: public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(self), std::sync::Arc::as_ptr(o)) {
            return true;
        }
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return o instanceof DefaultClientPosArgument defaultPosArgument &&
        //   java: 				this.x.equals(defaultPosArgument.x) && this.y.equals(defaultPosArgument.y) && this.z.equals(defaultPosArgument.z);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public int hashCode()
    pub fn hash_code(&self) -> i32 {
        // TODO(fabric2seer): `WorldCoordinate.hashCode` has no Rust method in the port (looked for `hash_code` in WorldCoordinate and JavaSrc)
        // rust: let mut i: i32 = self.x.hash_code();
        let mut i: i32 = todo!("fabric2seer: the initialiser of `i` (TODO above)");
        // TODO(fabric2seer): `WorldCoordinate.hashCode` has no Rust method in the port (looked for `hash_code` in WorldCoordinate and JavaSrc)
        // rust: i = (31i32.wrapping_mul(i)).wrapping_add(self.y.hash_code());
        // TODO(fabric2seer): `WorldCoordinate.hashCode` has no Rust method in the port (looked for `hash_code` in WorldCoordinate and JavaSrc)
        // rust: return (31i32.wrapping_mul(i)).wrapping_add(self.z.hash_code());
        return todo!("fabric2seer: see the TODO above");
    }

}

impl ClientPosArgument for DefaultClientPosArgument {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Vec3 toAbsolutePos(FabricClientCommandSource source) (dispatch: the body for this class)
    fn to_absolute_pos(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        DefaultClientPosArgument::to_absolute_pos_dyn
    }

    /// java: @Override public Vec2 toAbsoluteRotation(FabricClientCommandSource source) (dispatch: the body for this class)
    fn to_absolute_rotation(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        DefaultClientPosArgument::to_absolute_rotation_dyn
    }

    /// java: @Override public boolean isXRelative() (dispatch: the body for this class)
    fn is_x_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        DefaultClientPosArgument::is_x_relative_dyn
    }

    /// java: @Override public boolean isYRelative() (dispatch: the body for this class)
    fn is_y_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        DefaultClientPosArgument::is_y_relative_dyn
    }

    /// java: @Override public boolean isZRelative() (dispatch: the body for this class)
    fn is_z_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        DefaultClientPosArgument::is_z_relative_dyn
    }

}

impl DefaultClientPosArgument {
    /// java: @Override public Vec3 toAbsolutePos(FabricClientCommandSource source)
    pub fn to_absolute_pos_impl(__this: &compat::Shared<Self>, source: &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        // TODO(fabric2seer): `.getPosition(..)` on a FabricClientCommandSource
        //   java: Vec3 vec3d = source.getPosition();
        let mut vec3d: Vec3 = todo!("fabric2seer: the initialiser of `vec3d` (TODO above)");
        return Vec3::new(__this.lock().borrow().x.get(vec3d.x), __this.lock().borrow().y.get(vec3d.y), __this.lock().borrow().z.get(vec3d.z));
    }

    /// DefaultClientPosArgument.toAbsolutePos as the dispatch of ClientPosArgument.toAbsolutePos
    pub fn to_absolute_pos_dyn(__this: &compat::Shared<dyn ClientPosArgument>, __p0: &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        Self::to_absolute_pos_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Vec2 toAbsoluteRotation(FabricClientCommandSource source)
    pub fn to_absolute_rotation_impl(__this: &compat::Shared<Self>, source: &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        // TODO(fabric2seer): `.getRotation(..)` on a FabricClientCommandSource
        //   java: Vec2 vec2f = source.getRotation();
        let mut vec2f: Vec2 = todo!("fabric2seer: the initialiser of `vec2f` (TODO above)");
        return Vec2::new((({ let (__q0,) = ((vec2f.x) as f64,); __this.lock().borrow().x.get(__q0) }) as f32), (({ let (__q0,) = ((vec2f.y) as f64,); __this.lock().borrow().y.get(__q0) }) as f32));
    }

    /// DefaultClientPosArgument.toAbsoluteRotation as the dispatch of ClientPosArgument.toAbsoluteRotation
    pub fn to_absolute_rotation_dyn(__this: &compat::Shared<dyn ClientPosArgument>, __p0: &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        Self::to_absolute_rotation_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isXRelative()
    pub fn is_x_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().x.is_relative();
    }

    /// DefaultClientPosArgument.isXRelative as the dispatch of ClientPosArgument.isXRelative
    pub fn is_x_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_x_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean isYRelative()
    pub fn is_y_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().y.is_relative();
    }

    /// DefaultClientPosArgument.isYRelative as the dispatch of ClientPosArgument.isYRelative
    pub fn is_y_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_y_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean isZRelative()
    pub fn is_z_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return __this.lock().borrow().z.is_relative();
    }

    /// DefaultClientPosArgument.isZRelative as the dispatch of ClientPosArgument.isZRelative
    pub fn is_z_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_z_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
