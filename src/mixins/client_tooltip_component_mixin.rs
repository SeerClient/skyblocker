//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.ClientTooltipComponentMixin by fabric2seer.
//! @Mixin(net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent)

#[allow(unused_imports)]
use crate::prelude::*;

/// @ModifyReturnValue create
///   java: @ModifyReturnValue(method = "create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;", at = @At("TAIL"))
/// target: net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent#create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;  [exists]
/// Rust: net/minecraft/client/gui/screens/inventory/tooltip/client_tooltip_component.rs:84 fn create_text()
fn create(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): read the return value typed: ci.return_value::<T>() (ClientTooltipComponent)
    // TODO(fabric2seer): unknown name `charSequence`
    //   java: if (charSequence instanceof GridFormattedCharSequence gridSequence) {
    //   java: 			return new GridTooltipComponent(gridSequence);
    //   java: 		} else {
    //   java: 			return original;
    //   java: 		}
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::register_injector("net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent#create(Lnet/minecraft/util/FormattedCharSequence;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;", InjectOptions::at(At::Return).kind(InjectorKind::ModifyReturnValue).priority(1000), Arc::new(create)));
}
