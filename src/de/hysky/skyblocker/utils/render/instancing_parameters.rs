//! Ported from de.hysky.skyblocker.utils.render.InstancingParameters by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::buffers::gpu_buffer_slice::GpuBufferSlice;

/// java: public static final InstancingParameters NONE = new InstancingParameters(1, null, null);
pub static NONE: compat::MainThread<InstancingParameters> = compat::MainThread::new(|| InstancingParameters::new(1, None, None));

/// java: public record InstancingParameters(int count, @Nullable String name, @Nullable GpuBufferSlice buffer)
#[derive(Clone, Debug, PartialEq)]
pub struct InstancingParameters {
    /// java: int count
    pub count: i32,
    /// java: String name
    pub name: Option<String>,
    /// java: GpuBufferSlice buffer
    pub buffer: Option<GpuBufferSlice>,
}

impl InstancingParameters {
    /// java: the canonical record constructor
    pub fn new(mut count: i32, mut name: Option<String>, mut buffer: Option<GpuBufferSlice>) -> Self {
        Self { count, name, buffer }
    }

    /// java: the record accessor `count()`
    pub fn count(&self) -> i32 {
        self.count
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// java: the record accessor `buffer()`
    pub fn buffer(&self) -> Option<GpuBufferSlice> {
        self.buffer.clone()
    }

}
