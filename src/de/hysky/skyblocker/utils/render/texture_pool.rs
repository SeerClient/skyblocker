//! Ported from de.hysky.skyblocker.utils.render.TexturePool by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::gpu_format::GpuFormat;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::textures::gpu_texture::GpuTexture;
use mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView;

/// java: public record TexturePool(String name, int size, @GpuTexture.Usage int usage, GpuFormat format, @Nullable GpuTexture[] textures, @Nullable GpuTextureView[] textureViews, boolean[] usedSlots) implements AutoCloseable
pub struct TexturePool {
    /// java: String name
    pub name: String,
    /// java: int size
    pub size: i32,
    /// java: int usage
    pub usage: i32,
    /// java: GpuFormat format
    pub format: GpuFormat,
    /// java: GpuTexture[] textures
    pub textures: Option<Vec<Box<dyn GpuTexture>>>,
    /// java: GpuTextureView[] textureViews
    pub texture_views: Option<Vec<Box<dyn GpuTextureView>>>,
    /// java: boolean[] usedSlots
    pub used_slots: Vec<bool>,
}

impl TexturePool {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut size: i32, mut usage: i32, mut format: GpuFormat, mut textures: Option<Vec<Box<dyn GpuTexture>>>, mut texture_views: Option<Vec<Box<dyn GpuTextureView>>>, mut used_slots: Vec<bool>) -> Self {
        // TODO(fabric2seer): method reference `::clearUnusedTextures` to a method this tool cannot see
        //   java: LevelRenderEvents.START_MAIN.register(this::clearUnusedTextures);
        Self { name, size, usage, format, textures, texture_views, used_slots }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// java: the record accessor `size()`
    pub fn size(&self) -> i32 {
        self.size
    }

    /// java: the record accessor `usage()`
    pub fn usage(&self) -> i32 {
        self.usage
    }

    /// java: the record accessor `format()`
    pub fn format(&self) -> GpuFormat {
        self.format
    }

    /// java: the record accessor `textures()`
    pub fn textures(&self) -> Option<Vec<Box<dyn GpuTexture>>> {
        self.textures.clone()
    }

    /// java: the record accessor `textureViews()`
    pub fn texture_views(&self) -> Option<Vec<Box<dyn GpuTextureView>>> {
        self.texture_views.clone()
    }

    /// java: the record accessor `usedSlots()`
    pub fn used_slots(&self) -> Vec<bool> {
        self.used_slots.clone()
    }

    /// java: public static TexturePool create(String name, int size, @GpuTexture.Usage int usage, GpuFormat format)
    pub fn create(name: &str, mut size: i32, mut usage: i32, format: &GpuFormat) -> TexturePool {
        return TexturePool::new(name.to_string(), size, usage, *format, Some((0..(size) as usize).map(|_| Default::default()).collect::<Vec<_>>()), Some((0..(size) as usize).map(|_| Default::default()).collect::<Vec<_>>()), vec![false; (size) as usize]);
    }

    /// java: private void clearUnusedTextures(LevelTerrainRenderContext context)
    pub fn clear_unused_textures(&self, context: &crate::unresolved::LevelTerrainRenderContext) {
        for i in 0..self.size() {
            if !(self.used_slots()[(i) as usize]) {
                let mut texture: Option<Box<dyn GpuTexture>> = Some(self.textures().unwrap()[(i) as usize].clone());
                let mut texture_view: Option<Box<dyn GpuTextureView>> = Some(self.texture_views().unwrap()[(i) as usize].clone());
                if texture.is_some() && texture_view.is_some() {
                    texture.as_ref().unwrap().close();
                    <_ as mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView>::close(&texture_view.as_ref().unwrap());
                    self.textures().unwrap()[(i) as usize] = None;
                    self.texture_views().unwrap()[(i) as usize] = None;
                }
            }
        }
        self.used_slots().fill(false);
    }

    /// java: public int getNextAvailableIndex(int requiredWidth, int requiredHeight)
    pub fn get_next_available_index(&self, mut required_width: i32, mut required_height: i32) -> i32 {
        for i in 0..self.size() {
            if !(self.used_slots()[(i) as usize]) {
                let mut texture: Option<Box<dyn GpuTexture>> = Some(self.textures().unwrap()[(i) as usize].clone());
                let mut texture_view: Option<Box<dyn GpuTextureView>> = Some(self.texture_views().unwrap()[(i) as usize].clone());
                let mut texture_exists: bool = (texture.is_some() && texture_view.is_some());
                let mut texture_sizes_match: bool = (texture_exists && texture.as_ref().unwrap().get_width(0) == required_width && texture.as_ref().unwrap().get_height(0) == required_height);
                if texture_sizes_match {
                    self.used_slots()[(i) as usize] = true;
                    return i;
                } else if !texture_exists {
                    let mut new_texture: Box<dyn GpuTexture> = render_system::get_device().create_texture(Some(&format!("{} {}", self.name(), i)), self.usage(), self.format(), required_width, required_height, 1, 1);
                    let mut new_texture_view: Box<dyn GpuTextureView> = render_system::get_device().create_texture_view(new_texture.clone());
                    self.textures().unwrap()[(i) as usize] = new_texture.clone();
                    self.texture_views().unwrap()[(i) as usize] = new_texture_view.clone();
                    self.used_slots()[(i) as usize] = true;
                    return i;
                }
            }
        }
        panic!("UnsupportedOperationException: {}", "Trying to use too many textures than are allocated for the pool");
    }

    /// java: public GpuTexture getTexture(int index)
    pub fn get_texture(&self, mut index: i32) -> Box<dyn GpuTexture> {
        return self.textures().unwrap()[(index) as usize].clone();
    }

    /// java: public GpuTextureView getTextureView(int index)
    pub fn get_texture_view(&self, mut index: i32) -> Box<dyn GpuTextureView> {
        return self.texture_views().unwrap()[(index) as usize].clone();
    }

    /// java: @Override public void close()
    pub fn close(&self) {
        for texture in self.textures().unwrap().iter_mut() {
            if true /* java: texture != null: never null here */ {
                texture.close();
            }
        }
        for texture_view in self.texture_views().unwrap().iter_mut() {
            if true /* java: texture_view != null: never null here */ {
                <_ as mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView>::close(texture_view);
            }
        }
        self.used_slots().fill(false);
    }

}
