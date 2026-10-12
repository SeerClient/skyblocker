//! Ported from de.hysky.skyblocker.utils.command.argumenttypes.blockpos.ClientPosArgument by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::core::block_pos::BlockPos;
use mcrust::net::minecraft::world::phys::vec2::Vec2;
use mcrust::net::minecraft::world::phys::vec3::Vec3;

/// java: public interface ClientPosArgument
pub trait ClientPosArgument: compat::DynAny + Send {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Vec3 toAbsolutePos(FabricClientCommandSource source); (dispatch: the body for this class)
    fn to_absolute_pos(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec3;

    /// java: Vec2 toAbsoluteRotation(FabricClientCommandSource source); (dispatch: the body for this class)
    fn to_absolute_rotation(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> Vec2;

    /// java: default BlockPos toAbsoluteBlockPos(FabricClientCommandSource source) (dispatch: the body for this class)
    fn to_absolute_block_pos(&self) -> fn(&compat::Shared<dyn ClientPosArgument>, &crate::unresolved::FabricClientCommandSource) -> BlockPos {
        <dyn ClientPosArgument>::to_absolute_block_pos_body
    }

    /// java: boolean isXRelative(); (dispatch: the body for this class)
    fn is_x_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool;

    /// java: boolean isYRelative(); (dispatch: the body for this class)
    fn is_y_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool;

    /// java: boolean isZRelative(); (dispatch: the body for this class)
    fn is_z_relative(&self) -> fn(&compat::Shared<dyn ClientPosArgument>) -> bool;

}

impl dyn ClientPosArgument {
    /// java: default BlockPos toAbsoluteBlockPos(FabricClientCommandSource source)
    pub fn to_absolute_block_pos_body(__this: &compat::Shared<dyn ClientPosArgument>, source: &crate::unresolved::FabricClientCommandSource) -> BlockPos {
        return BlockPos::containing_position(&({ let __f = compat::dispatch(__this, |__t| __t.to_absolute_pos()); __f(__this, source) }));
    }

}
