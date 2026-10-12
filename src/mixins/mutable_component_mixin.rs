//! Ported from the Fabric mixin de.hysky.skyblocker.mixins.MutableComponentMixin by fabric2seer.
//! @Mixin(net.minecraft.network.chat.MutableComponent)

#[allow(unused_imports)]
use crate::prelude::*;

/// @WrapOperation getVisualOrderText
///   java: @WrapOperation(method = "getVisualOrderText", at = @At(value = "INVOKE", target = "Lnet/minecraft/locale/Language;getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;"))
/// target: net.minecraft.locale.Language#getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;  [exists]
/// within: net.minecraft.network.chat.MutableComponent#getVisualOrderText
/// Rust: net/minecraft/locale/language.rs:82 fn get_visual_order(&self, logical_order_text: &crate::net::minecraft::network::chat::formatted_text::FormattedText)
fn get_visual_order_text(ci: &mut CallbackInfo<'_>, original: &mut Original<'_>) {
    let Some(this) = compat::detach_ref(ci.this::<mcrust::net::minecraft::locale::language::Language>()) else { return; };
    let Some(formatted_text) = compat::detach_ref(ci.arg::<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>(0)) else { return; };
    // TODO(fabric2seer): `Language.getContents` has no Rust method in the port (looked for `get_contents` in Language and JavaSrc)
    if let Some(grid_contents) = this.get_contents().as_any().downcast_ref::<crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents>() {
        ci.set_return_value(crate::de::hysky::skyblocker::utils::render::text::grid_formatted_char_sequence::GridFormattedCharSequence::new(grid_contents.clone()));
        return;
    }
    // TODO(fabric2seer): `original.call(..)` of a method returning an object: call `original.call(ci)` and read `ci.return_value::<T>()` with the port's type
    //   java: return original.call(instance, formattedText);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.network.chat.MutableComponent#getVisualOrderText", "net.minecraft.locale.Language#getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;").priority(1000).wrap_operation(get_visual_order_text));
}
