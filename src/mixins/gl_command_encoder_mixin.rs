//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.GlCommandEncoderMixin by fabric2seer.
//! @Mixin(com.mojang.blaze3d.opengl.GlCommandEncoder)

#[allow(unused_imports)]
use crate::prelude::*;

thread_local! {
    /// `@Local(name = "slice") GpuBufferSlice slice` - `writeToBuffer`'s own parameter, recorded by a HEAD injector on
    /// it (a call-site handler sees only the CALL's arguments): `slice.length() == slice.buffer().size()`.
    static SLICE_IS_WHOLE_BUFFER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// HEAD of `writeToBuffer(slice, data)`: records what the call-site handler reads of `slice`.
fn record_slice(ci: &mut CallbackInfo<'_>) {
    use mcrust::com::mojang::blaze3d::buffers::gpu_buffer::GpuBuffer as _;
    let whole = ci.arg::<mcrust::com::mojang::blaze3d::buffers::gpu_buffer_slice::GpuBufferSlice>(0).is_some_and(|slice| slice.length() == slice.buffer().size());
    SLICE_IS_WHOLE_BUFFER.with(|cell| cell.set(whole));
}

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
    //   java: offset == 0 && slice.length() == slice.buffer().size()
    if offset == 0 && SLICE_IS_WHOLE_BUFFER.with(|cell| cell.get()) {
        //   java: ((DirectStateAccessInvoker) manager).invokeBufferData(buffer, data, slice.buffer().usage());
        // `slice.buffer().usage()` is the call's own `usage` argument (`buffer.usage()`, GlCommandEncoder:262).
        this.buffer_data(buffer, data, usage);
        return false;
    }
    return true;
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("com.mojang.blaze3d.opengl.GlCommandEncoder#writeToBuffer", InjectOptions::at(At::Head).kind(InjectorKind::Inject).priority(1000), Arc::new(record_slice)));
    report(hook::call_site("com.mojang.blaze3d.opengl.GlCommandEncoder#writeToBuffer", "com.mojang.blaze3d.opengl.DirectStateAccess#bufferSubData").priority(1000).wrap_with_condition(skyblocker_replace_buffer_data));
}
