//! Ported from de.hysky.skyblocker.utils.render.TexturePool by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::gpu_format::GpuFormat;
use mcrust::com::mojang::blaze3d::systems::render_system;
use mcrust::com::mojang::blaze3d::textures::gpu_texture::GpuTexture;
use mcrust::com::mojang::blaze3d::textures::gpu_texture_view::GpuTextureView;
use std::rc::Rc;

//// java: public record TexturePool(String name, int size, @GpuTexture.Usage int usage, GpuFormat format, @Nullable GpuTexture[] textures, @Nullable GpuTextureView[] textureViews, boolean[] usedSlots) implements AutoCloseable
///
/// The three arrays are the record's own objects, written through its accessors (`this.textures()[i] = ..`): cells
/// here, so the record's `&self` methods write them as Java does.
pub struct TexturePool {
    /// java: String name
    pub name: String,
    /// java: int size
    pub size: i32,
    /// java: int usage
    pub usage: i32,
    /// java: GpuFormat format
    pub format: GpuFormat,
    /// java: @Nullable GpuTexture[] textures
    pub textures: std::cell::RefCell<Vec<Option<Rc<dyn GpuTexture>>>>,
    /// java: @Nullable GpuTextureView[] textureViews
    pub texture_views: std::cell::RefCell<Vec<Option<Rc<dyn GpuTextureView>>>>,
    /// java: boolean[] usedSlots
    pub used_slots: std::cell::RefCell<Vec<bool>>,
}

impl TexturePool {
    /// java: the canonical record constructor
    pub fn new(mut name: String, mut size: i32, mut usage: i32, mut format: GpuFormat, mut textures: Vec<Option<Rc<dyn GpuTexture>>>, mut texture_views: Vec<Option<Rc<dyn GpuTextureView>>>, mut used_slots: Vec<bool>) -> Self {
        // TODO(fabric2seer): method reference `::clearUnusedTextures` to a method this tool cannot see
        //   java: LevelRenderEvents.START_MAIN.register(this::clearUnusedTextures);
        Self {
            name,
            size,
            usage,
            format,
            textures: std::cell::RefCell::new(textures),
            texture_views: std::cell::RefCell::new(texture_views),
            used_slots: std::cell::RefCell::new(used_slots),
        }
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

    /// java: the record accessor `textures()` - the array itself
    pub fn textures(&self) -> &std::cell::RefCell<Vec<Option<Rc<dyn GpuTexture>>>> {
        &self.textures
    }

    /// java: the record accessor `textureViews()` - the array itself
    pub fn texture_views(&self) -> &std::cell::RefCell<Vec<Option<Rc<dyn GpuTextureView>>>> {
        &self.texture_views
    }

    /// java: the record accessor `usedSlots()` - the array itself
    pub fn used_slots(&self) -> &std::cell::RefCell<Vec<bool>> {
        &self.used_slots
    }

    /// java: public static TexturePool create(String name, int size, @GpuTexture.Usage int usage, GpuFormat format)
    pub fn create(name: &str, mut size: i32, mut usage: i32, format: &GpuFormat) -> TexturePool {
        // java: new GpuTexture[size], new GpuTextureView[size], new boolean[size]
        return TexturePool::new(name.to_string(), size, usage, *format, (0..(size) as usize).map(|_| None).collect::<Vec<_>>(), (0..(size) as usize).map(|_| None).collect::<Vec<_>>(), vec![false; (size) as usize]);
    }

    /// java: private void clearUnusedTextures(LevelTerrainRenderContext context)
    pub fn clear_unused_textures(&self, context: &crate::unresolved::LevelTerrainRenderContext) {
        // Close textures if they were unused for a frame
        for i in 0..self.size() {
            if !(self.used_slots().borrow()[(i) as usize]) {
                let mut texture: Option<Rc<dyn GpuTexture>> = self.textures().borrow()[(i) as usize].clone();
                let mut texture_view: Option<Rc<dyn GpuTextureView>> = self.texture_views().borrow()[(i) as usize].clone();
                if texture.is_some() && texture_view.is_some() {
                    texture.as_ref().unwrap().close();
                    texture_view.as_ref().unwrap().close();
                    self.textures().borrow_mut()[(i) as usize] = None;
                    self.texture_views().borrow_mut()[(i) as usize] = None;
                }
            }
        }
        self.used_slots().borrow_mut().fill(false);
    }

    /// java: public int getNextAvailableIndex(int requiredWidth, int requiredHeight)
    pub fn get_next_available_index(&self, mut required_width: i32, mut required_height: i32) -> i32 {
        for i in 0..self.size() {
            if !(self.used_slots().borrow()[(i) as usize]) {
                let mut texture: Option<Rc<dyn GpuTexture>> = self.textures().borrow()[(i) as usize].clone();
                let mut texture_view: Option<Rc<dyn GpuTextureView>> = self.texture_views().borrow()[(i) as usize].clone();
                let mut texture_exists: bool = (texture.is_some() && texture_view.is_some());
                let mut texture_sizes_match: bool = (texture_exists && texture.as_ref().unwrap().get_width(0) == required_width && texture.as_ref().unwrap().get_height(0) == required_height);
                if texture_sizes_match {
                    self.used_slots().borrow_mut()[(i) as usize] = true;
                    return i;
                } else if !texture_exists {
                    let mut new_texture: Rc<dyn GpuTexture> = render_system::get_device().create_texture(Some(&format!("{} {}", self.name(), i)), self.usage(), self.format(), required_width, required_height, 1, 1);
                    let mut new_texture_view: Rc<dyn GpuTextureView> = render_system::get_device().create_texture_view(new_texture.clone());
                    self.textures().borrow_mut()[(i) as usize] = Some(new_texture);
                    self.texture_views().borrow_mut()[(i) as usize] = Some(new_texture_view);
                    self.used_slots().borrow_mut()[(i) as usize] = true;
                    return i;
                }
            }
        }
        panic!("UnsupportedOperationException: {}", "Trying to use too many textures than are allocated for the pool");
    }

    /// java: public GpuTexture getTexture(int index)
    pub fn get_texture(&self, mut index: i32) -> Rc<dyn GpuTexture> {
        // java: return Objects.requireNonNull(this.textures()[index]);
        return self.textures().borrow()[(index) as usize].clone().expect("NullPointerException: texture");
    }

    /// java: public GpuTextureView getTextureView(int index)
    pub fn get_texture_view(&self, mut index: i32) -> Rc<dyn GpuTextureView> {
        // java: return Objects.requireNonNull(this.textureViews()[index]);
        return self.texture_views().borrow()[(index) as usize].clone().expect("NullPointerException: texture view");
    }

    /// java: @Override public void close()
    pub fn close(&self) {
        for texture in self.textures().borrow().iter() {
            if let Some(texture) = texture {
                texture.close();
            }
        }
        for texture_view in self.texture_views().borrow().iter() {
            if let Some(texture_view) = texture_view {
                texture_view.close();
            }
        }
        self.used_slots().borrow_mut().fill(false);
    }

}
