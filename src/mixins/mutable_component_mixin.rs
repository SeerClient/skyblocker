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
    use mcrust::net::minecraft::network::chat::component::Component as _;
    use mcrust::net::minecraft::network::chat::component_contents::ComponentContents;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    // `getContents()` is the contents of the component `getVisualOrderText` runs on; `:97-98` hands that component to
    // `getVisualOrder` as the call's `formattedText` (argument 0, `FormattedText::from(self.clone())`).
    let grid_contents: Option<crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents> = match ci.arg::<FormattedText>(0) {
        Some(FormattedText::Component(component)) => match &**component.get_contents() {
            ComponentContents::Mod(contents) => contents.as_any().downcast_ref::<crate::de::hysky::skyblocker::utils::render::text::grid_component::Contents>().cloned(),
            _ => None,
        },
        _ => None,
    };
    if let Some(grid_contents) = grid_contents {
        let sequence: mcrust::net::minecraft::util::formatted_char_sequence::Seq = std::sync::Arc::new(crate::de::hysky::skyblocker::utils::render::text::grid_formatted_char_sequence::GridFormattedCharSequence::new(grid_contents));
        ci.set_return_value(sequence);
        return;
    }
    // java: return original.call(instance, formattedText);
    original.call(ci);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    report(hook::call_site("net.minecraft.network.chat.MutableComponent#getVisualOrderText", "net.minecraft.locale.Language#getVisualOrder(Lnet/minecraft/network/chat/FormattedText;)Lnet/minecraft/util/FormattedCharSequence;").priority(1000).wrap_operation(get_visual_order_text));
}
