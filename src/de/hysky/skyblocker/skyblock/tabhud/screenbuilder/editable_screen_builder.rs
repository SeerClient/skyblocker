//! Ported from de.hysky.skyblocker.skyblock.tabhud.screenbuilder.EditableScreenBuilder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder::LayerBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::layer_builder_editor::LayerBuilderEditor;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilder;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::screen_builder::ScreenBuilderBase;
use crate::de::hysky::skyblocker::skyblock::tabhud::screenbuilder::widget_manager::ScreenLayer;
use crate::de::hysky::skyblocker::skyblock::tabhud::widget::hud_widget::HudWidget;

/// java: public class EditableScreenBuilder extends ScreenBuilder
#[derive(Clone)]
pub struct EditableScreenBuilder {
    /// java: extends ScreenBuilder
    pub base: ScreenBuilderBase,
}

impl std::ops::Deref for EditableScreenBuilder {
    type Target = ScreenBuilderBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for EditableScreenBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl EditableScreenBuilder {
    /// java: public EditableScreenBuilder()
    pub fn new() -> compat::Shared<Self> {
        let (_, __this) = ScreenBuilderBase::new_0_in(move |__base| {
            let __h = compat::shared(Self {
                base: __base,
            });
            ((__h.clone() as compat::Shared<dyn ScreenBuilder>), __h)
        });
        __this
    }

    /// java: public LayerBuilderEditor getEditor(WidgetManager.ScreenLayer layer)
    pub fn get_editor(__this: &compat::Shared<Self>, mut layer: ScreenLayer) -> LayerBuilderEditor {
        return LayerBuilderEditor::new(({ let __f = compat::dispatch(__this, |__t| __t.get()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ScreenBuilder>), layer) }));
    }

    /// java: public EditableLayer getLayer(WidgetManager.ScreenLayer layer)
    pub fn get_layer(__this: &compat::Shared<Self>, mut layer: ScreenLayer) -> EditableLayer {
        return EditableLayer::new(({ let __f = compat::dispatch(__this, |__t| __t.get()); __f(&compat::view(__this, |a| a as compat::Shared<dyn ScreenBuilder>), layer) }), EditableScreenBuilder::get_editor(__this, layer));
    }

}

impl ScreenBuilder for EditableScreenBuilder {
    fn screen_builder_base(&self) -> &ScreenBuilderBase {
        &self.base
    }
    fn screen_builder_base_mut(&mut self) -> &mut ScreenBuilderBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public record EditableLayer(LayerBuilder builder, LayerBuilderEditor editor)
#[derive(Clone, Debug)]
pub struct EditableLayer {
    /// java: LayerBuilder builder
    pub builder: compat::Shared<dyn LayerBuilder>,
    /// java: LayerBuilderEditor editor
    pub editor: LayerBuilderEditor,
}

impl EditableLayer {
    /// java: the canonical record constructor
    pub fn new(mut builder: compat::Shared<dyn LayerBuilder>, mut editor: LayerBuilderEditor) -> Self {
        Self { builder, editor }
    }

    /// java: the record accessor `builder()`
    pub fn builder(&self) -> compat::Shared<dyn LayerBuilder> {
        self.builder.clone()
    }

    /// java: the record accessor `editor()`
    pub fn editor(&self) -> LayerBuilderEditor {
        self.editor.clone()
    }

    /// java: public void update()
    pub fn update(&mut self) {
        ({ let __f = compat::dispatch(&self.builder, |__t| __t.update()); __f(&self.builder) });
        for w in ({ let __f = compat::dispatch(&self.builder, |__t| __t.get_rendered()); __f(&self.builder) }) {
            ({ let __r = &compat::read(&w, |__t| __t.widget.clone()); let __f = compat::dispatch(__r, |__t| __t.update_config_preview()); __f(__r) });
        }
    }

}
