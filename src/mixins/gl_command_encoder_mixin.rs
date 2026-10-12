//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.GlCommandEncoderMixin by fabric2seer.
//! @Mixin(com.mojang.blaze3d.opengl.GlCommandEncoder)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapWithCondition skyblocker$replaceBufferData
///   java: @WrapWithCondition(method = "writeToBuffer", at = @At(value = "INVOKE", target = "Lcom/mojang/blaze3d/opengl/DirectStateAccess;bufferSubData(IJLjava/nio/ByteBuffer;I)V"))
/// target: com.mojang.blaze3d.opengl.DirectStateAccess#bufferSubData  [exists]
/// within: com.mojang.blaze3d.opengl.GlCommandEncoder#writeToBuffer
/// Rust: com/mojang/blaze3d/opengl/direct_state_access.rs:149 fn buffer_sub_data(&self, buffer: u32, offset: i64, data: &[u8], usage: i32)
fn skyblocker_replace_buffer_data(ci: &CallbackInfo<'_>) -> bool {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::com::mojang::blaze3d::opengl::direct_state_access::DirectStateAccess>()) else { return true; };
    let buffer: u32 = ci.arg::<u32>(0).copied().unwrap_or_default();
    let offset: i64 = ci.arg::<i64>(1).copied().unwrap_or_default();
    let Some(data) = compat::detach_ref(ci.arg::<[u8]>(2)) else { return true; };
    let usage: i32 = ci.arg::<i32>(3).copied().unwrap_or_default();
    // TODO(fabric2seer): the condition: `slice`: this argument is not visible to the injector (see the notes above the handler)
    //   java: offset == 0 && slice.length() == slice.buffer().size()
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `slice`: this argument is not visible to the injector (see the notes above the handler)
        //   java: ((DirectStateAccessInvoker) manager).invokeBufferData(buffer, data, slice.buffer().usage());
        return false;
    }
    return true;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("com.mojang.blaze3d.opengl.GlCommandEncoder#writeToBuffer", "com.mojang.blaze3d.opengl.DirectStateAccess#bufferSubData").priority(1000).wrap_with_condition(skyblocker_replace_buffer_data));
}
