//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.blockpos.LookingClientPosArgument by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::command::argumenttypes::blockpos::client_pos_argument::ClientPosArgument;
use mcrust::com::mojang::brigadier::string_reader::StringReader;
use mcrust::net::minecraft::util::mth;
use mcrust::net::minecraft::world::phys::vec2::Vec2;
use mcrust::net::minecraft::world::phys::vec3::Vec3;
use mcrust::com::mojang::brigadier::immutable_string_reader::ImmutableStringReader as _;

/// java: public class LookingClientPosArgument implements ClientPosArgument
#[derive(Clone)]
pub struct LookingClientPosArgument {
    /// java: private final double x;
    pub x: f64,
    /// java: private final double y;
    pub y: f64,
    /// java: private final double z;
    pub z: f64,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl LookingClientPosArgument {
    /// java: public LookingClientPosArgument(double x, double y, double z)
    pub fn new(mut x: f64, mut y: f64, mut z: f64) -> compat::Shared<Self> {
        let __init_x = x;
        let __init_y = y;
        let __init_z = z;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            x: __init_x,
            y: __init_y,
            z: __init_z,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: public static LookingClientPosArgument parse(StringReader reader) throws CommandSyntaxException
    pub fn parse(reader: &mut StringReader) -> compat::Shared<LookingClientPosArgument> {
        let mut i: i32 = reader.get_cursor();
        let mut d: f64 = LookingClientPosArgument::read_coordinate(reader, i);
        if reader.can_read() && reader.peek() == ' ' {
            reader.skip();
            let mut e: f64 = LookingClientPosArgument::read_coordinate(reader, i);
            if reader.can_read() && reader.peek() == ' ' {
                reader.skip();
                let mut f: f64 = LookingClientPosArgument::read_coordinate(reader, i);
                return LookingClientPosArgument::new(d, e, f);
            } else {
                reader.set_cursor(i);
                panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
            }
        } else {
            reader.set_cursor(i);
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_not_complete().create_with_context(reader));
        }
    }

    /// java: private static double readCoordinate(StringReader reader, int startingCursorPos) throws CommandSyntaxException
    pub fn read_coordinate(reader: &mut StringReader, mut starting_cursor_pos: i32) -> f64 {
        if !reader.can_read() {
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::world_coordinate::error_expected_double().create_with_context(reader));
        } else if reader.peek() != '^' {
            reader.set_cursor(starting_cursor_pos);
            panic!("{:?}", mcrust::net::minecraft::commands::arguments::coordinates::vec3_argument::error_mixed_type().create_with_context(reader));
        } else {
            reader.skip();
            return (if reader.can_read() && reader.peek() != ' ' { reader.read_double().unwrap() } else { 0.0 });
        }
    }

    /// java: public boolean equals(Object o)
    pub fn equals(&self, o: &compat::Object) -> bool {
        if std::ptr::addr_eq(std::sync::Arc::as_ptr(self), std::sync::Arc::as_ptr(o)) {
            return true;
        }
        // TODO(fabric2seer): instanceof with a pattern binding outside an `if` condition
        //   java: return o instanceof LookingClientPosArgument lookingPosArgument
        //   java: 				&& this.x == lookingPosArgument.x && this.y == lookingPosArgument.y && this.z == lookingPosArgument.z;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public int hashCode()
    pub fn hash_code(&self) -> i32 {
        return compat::hash_code(&(&self.x, &self.y, &self.z,));
    }

}

impl ClientPosArgument for LookingClientPosArgument {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Vec3 toAbsolutePos(FabricClientCommandSource source) (dispatch: the body for this class)
    fn to_absolute_pos(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        LookingClientPosArgument::to_absolute_pos_dyn
    }

    /// java: @Override public Vec2 toAbsoluteRotation(FabricClientCommandSource source) (dispatch: the body for this class)
    fn to_absolute_rotation(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        LookingClientPosArgument::to_absolute_rotation_dyn
    }

    /// java: @Override public boolean isXRelative() (dispatch: the body for this class)
    fn is_x_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        LookingClientPosArgument::is_x_relative_dyn
    }

    /// java: @Override public boolean isYRelative() (dispatch: the body for this class)
    fn is_y_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        LookingClientPosArgument::is_y_relative_dyn
    }

    /// java: @Override public boolean isZRelative() (dispatch: the body for this class)
    fn is_z_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool {
        LookingClientPosArgument::is_z_relative_dyn
    }

}

impl LookingClientPosArgument {
    /// java: @Override public Vec3 toAbsolutePos(FabricClientCommandSource source)
    pub fn to_absolute_pos_impl(__this: &compat::Shared<Self>, source: &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        // TODO(fabric2seer): `.getRotation(..)` on a FabricClientCommandSource
        //   java: Vec2 vec2f = source.getRotation();
        let mut vec2f: Vec2 = todo!("fabric2seer: the initialiser of `vec2f` (TODO above)");
        // TODO(fabric2seer): `.getPlayer(..)` on a FabricClientCommandSource
        //   java: Vec3 vec3d = source.getPlayer().position();
        let mut vec3d: Vec3 = todo!("fabric2seer: the initialiser of `vec3d` (TODO above)");
        let mut f: f32 = mth::cos((((vec2f.y + 90.0f32) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut g: f32 = mth::sin((((vec2f.y + 90.0f32) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut h: f32 = mth::cos((((-vec2f.x) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut i: f32 = mth::sin((((-vec2f.x) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut j: f32 = mth::cos(((((-vec2f.x) + 90.0f32) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut k: f32 = mth::sin(((((-vec2f.x) + 90.0f32) * (((std::f64::consts::PI / 180.0)) as f32))) as f64);
        let mut vec3d2: Vec3 = Vec3::new(((f * h)) as f64, (i) as f64, ((g * h)) as f64);
        let mut vec3d3: Vec3 = Vec3::new(((f * j)) as f64, (k) as f64, ((g * j)) as f64);
        let mut vec3d4: Vec3 = vec3d2.cross(&vec3d3).scale(-1.0);
        let mut d: f64 = (((vec3d2.x * compat::read(__this, |__t| __t.z)) + (vec3d3.x * compat::read(__this, |__t| __t.y))) + (vec3d4.x * compat::read(__this, |__t| __t.x)));
        let mut e: f64 = (((vec3d2.y * compat::read(__this, |__t| __t.z)) + (vec3d3.y * compat::read(__this, |__t| __t.y))) + (vec3d4.y * compat::read(__this, |__t| __t.x)));
        let mut l: f64 = (((vec3d2.z * compat::read(__this, |__t| __t.z)) + (vec3d3.z * compat::read(__this, |__t| __t.y))) + (vec3d4.z * compat::read(__this, |__t| __t.x)));
        return Vec3::new((vec3d.x + d), (vec3d.y + e), (vec3d.z + l));
    }

    /// LookingClientPosArgument.toAbsolutePos as the dispatch of ClientPosArgument.toAbsolutePos
    pub fn to_absolute_pos_dyn(__this: &compat::Shared<dyn ClientPosArgument>, __p0: &crate::unresolved::FabricClientCommandSource) -> Vec3 {
        Self::to_absolute_pos_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Vec2 toAbsoluteRotation(FabricClientCommandSource source)
    pub fn to_absolute_rotation_impl(__this: &compat::Shared<Self>, source: &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        return mcrust::net::minecraft::world::phys::vec2::ZERO;
    }

    /// LookingClientPosArgument.toAbsoluteRotation as the dispatch of ClientPosArgument.toAbsoluteRotation
    pub fn to_absolute_rotation_dyn(__this: &compat::Shared<dyn ClientPosArgument>, __p0: &crate::unresolved::FabricClientCommandSource) -> Vec2 {
        Self::to_absolute_rotation_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public boolean isXRelative()
    pub fn is_x_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// LookingClientPosArgument.isXRelative as the dispatch of ClientPosArgument.isXRelative
    pub fn is_x_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_x_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean isYRelative()
    pub fn is_y_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// LookingClientPosArgument.isYRelative as the dispatch of ClientPosArgument.isYRelative
    pub fn is_y_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_y_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean isZRelative()
    pub fn is_z_relative_impl(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// LookingClientPosArgument.isZRelative as the dispatch of ClientPosArgument.isZRelative
    pub fn is_z_relative_dyn(__this: &compat::Shared<dyn ClientPosArgument>) -> bool {
        Self::is_z_relative_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
