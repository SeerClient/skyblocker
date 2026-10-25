//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientTooltipComponentMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue create
///   java: @ModifyReturnValue(method = "create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;", at = @At("TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent#create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/tooltip/client_tooltip_component.rs:84 fn create_text()
fn create(ci: &mut CallbackInfo<'_>) {
    use mcrust::net::minecraft::network::chat::component::Component as _;
    use mcrust::net::minecraft::network::chat::component_contents::ComponentContents;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    // `charSequence` is the call's argument 0. The port's sequence is the line's `FormattedText`: the game hands the
    // tooltip lines straight to `create` (Java runs `Component::getVisualOrderText` first, where
    // `MutableComponentMixin` makes a grid line a `GridFormattedCharSequence`), so the same test is made here on the
    // component: `charSequence instanceof GridFormattedCharSequence` <=> its contents are `GridComponent.Contents`.
    let grid_contents: Option<crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents> = match ci.arg::<FormattedText>(0) {
        Some(FormattedText::Component(component)) => match &**component.get_contents() {
            ComponentContents::Mod(contents) => contents.as_any().downcast_ref::<crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents>().cloned(),
            _ => None,
        },
        _ => None,
    };
    if let Some(grid_contents) = grid_contents {
        //   java: return new GridTooltipComponent(gridSequence);
        let grid_sequence = crate::de::hysky::skyblocker::utils::render::text::grid_formatted_char_sequence::GridFormattedCharSequence::new(grid_contents);
        let component = crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponent::new(grid_sequence);
        let line: Box<dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::ClientTooltipComponent> =
            Box::new(crate::de::hysky::skyblocker::utils::render::text::grid_tooltip_component::GridTooltipComponentLine(component));
        ci.set_return_value(line);
    }
    //   java: } else { return original; }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent#create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(create)));
}
