//! Ported from de.hysky.skyblocker.utils.render.AbstractUniformTexelBuffer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::renderer::state::level::camera_render_state::CameraRenderState;
use mcrust::net::minecraft::util::argb;

/// java: private static final boolean IS_LITTLE_ENDIAN = ByteOrder.nativeOrder() == ByteOrder.LITTLE_ENDIAN;
pub static IS_LITTLE_ENDIAN: LazyLock<bool> = LazyLock::new(|| cfg!(target_endian = "little"));

/// java: `java.lang.foreign.MemorySegment` as these writers use it - the bytes an `Arena` allocated, which the caller
/// uploads (`asByteBuffer()`); `set(ValueLayout.JAVA_FLOAT | JAVA_INT, long offset, value)` writes in the layouts'
/// byte order, the native one, and a write past the end is Java's `IndexOutOfBoundsException` (a panic here).
pub trait MemorySegmentBytes {
    /// the segment's bytes
    fn segment_bytes(&mut self) -> &mut [u8];

    /// java: `segment.set(ValueLayout.JAVA_FLOAT, offset, value)`
    fn set_java_float(&mut self, offset: i64, value: f32) {
        let start = offset as usize;
        self.segment_bytes()[start..start + 4].copy_from_slice(&value.to_ne_bytes());
    }

    /// java: `segment.set(ValueLayout.JAVA_INT, offset, value)`
    fn set_java_int(&mut self, offset: i64, value: i32) {
        let start = offset as usize;
        self.segment_bytes()[start..start + 4].copy_from_slice(&value.to_ne_bytes());
    }
}

impl MemorySegmentBytes for [u8] {
    fn segment_bytes(&mut self) -> &mut [u8] {
        self
    }
}

impl MemorySegmentBytes for Vec<u8> {
    fn segment_bytes(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

/// The converter's `MemorySegment` placeholder holds no bytes: it is the zero-length segment (every `set` on it is out
/// of bounds, as on Java's `MemorySegment.NULL`).
impl MemorySegmentBytes for crate::unresolved::MemorySegment {
    fn segment_bytes(&mut self) -> &mut [u8] {
        &mut []
    }
}

/// java: public abstract class AbstractUniformTexelBuffer<T>
#[derive(Clone)]
pub struct AbstractUniformTexelBufferBase<T> {
    /// java: private final int instanceBytes;
    pub instance_bytes: i32,
    /// the class's type parameter (`T`: the state it writes)
    pub _states: std::marker::PhantomData<fn() -> T>,
}

impl<T: Clone + 'static> AbstractUniformTexelBufferBase<T> {
    /// java: protected AbstractUniformTexelBuffer(int instanceBytes)
    pub fn new(mut instance_bytes: i32) -> Self {
        let __init_instance_bytes = instance_bytes;
        let mut this = Self {
            instance_bytes: __init_instance_bytes,
            _states: std::marker::PhantomData,
        };
        this
    }

    /// java: protected static final int toNativeRgba(int colour)
    pub fn to_native_rgba(mut colour: i32) -> i32 {
        let mut abgr: i32 = argb::to_abgr(colour);
        // java: return IS_LITTLE_ENDIAN ? abgr : Integer.reverseBytes(abgr);
        return if *IS_LITTLE_ENDIAN { abgr } else { abgr.swap_bytes() };
    }

}

/// java: the virtual methods of AbstractUniformTexelBuffer (implemented by AbstractUniformTexelBufferBase and every subclass)
pub trait AbstractUniformTexelBuffer<T> {
    fn abstract_uniform_texel_buffer_base(&self) -> &AbstractUniformTexelBufferBase<T>;
    fn abstract_uniform_texel_buffer_base_mut(&mut self) -> &mut AbstractUniformTexelBufferBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public final int calculateRequiredSize(int count)
    fn calculate_required_size(&self, mut count: i32) -> i32 {
        return count.wrapping_mul(self.abstract_uniform_texel_buffer_base().instance_bytes);
    }

    /// java: public final void writeToBuffer(List<T> states, CameraRenderState camera, MemorySegment buffer)
    fn write_to_buffer<S: MemorySegmentBytes + ?Sized>(&self, states: &Vec<T>, camera: &CameraRenderState, buffer: &mut S) {
        for i in 0..(states.len() as i32) {
            self.write_to_buffer_4(i, &states[(i) as usize], camera, buffer);
        }
    }

    /// java: protected abstract void writeToBuffer(int index, T state, CameraRenderState camera, MemorySegment buffer);
    fn write_to_buffer_4<S: MemorySegmentBytes + ?Sized>(&self, index: i32, state: &T, camera: &CameraRenderState, buffer: &mut S);

}
