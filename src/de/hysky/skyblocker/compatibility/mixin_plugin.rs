//! Ported from de.hysky.skyblocker.compatibility.MixinPlugin by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: private static final boolean ADAPTIVE_TOOLTIPS_LOADED = FabricLoader.getInstance().isModLoaded("adaptive_tooltips");
pub static ADAPTIVE_TOOLTIPS_LOADED: LazyLock<bool> = LazyLock::new(|| mcrust::mcrust::modding::loader::is_mod_loaded("adaptive_tooltips"));

/// java: public class MixinPlugin implements IMixinConfigPlugin
#[derive(Clone)]
pub struct MixinPlugin {}

impl MixinPlugin {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @Override public void onLoad(String mixinPackage)
    pub fn on_load(&self, mixin_package: &str) {
    }

    /// java: @Override public String getRefMapperConfig()
    pub fn get_ref_mapper_config(&self) -> Option<String> {
        return None;
    }

    /// java: @Override public boolean shouldApplyMixin(String targetClassName, String mixinClassName)
    pub fn should_apply_mixin(&self, target_class_name: &str, mixin_class_name: &str) -> bool {
        //   java: return switch (mixinClassName) {
        //   java: 			case String s when s.endsWith("ScrollTrackerMixin") -> ADAPTIVE_TOOLTIPS_LOADED;
        //   java: 			case String s when s.endsWith("GlCommandEncoderMixin") -> SystemUtils.IS_OS_MAC && "aarch64".equalsIgnoreCase(SystemUtils.OS_ARCH);
        //   java: 			default -> true;
        //   java: 		};
        // (`SystemUtils.IS_OS_MAC` / `OS_ARCH` are the JVM's `os.name` / `os.arch`; the process's own target here.)
        let _ = target_class_name;
        if mixin_class_name.ends_with("ScrollTrackerMixin") {
            return *ADAPTIVE_TOOLTIPS_LOADED;
        }
        if mixin_class_name.ends_with("GlCommandEncoderMixin") {
            return std::env::consts::OS == "macos" && compat::equals_ignore_case("aarch64", std::env::consts::ARCH);
        }
        return true;
    }

    /// java: @Override public void acceptTargets(Set<String> myTargets, Set<String> otherTargets)
    pub fn accept_targets(&self, my_targets: &std::collections::HashSet<String>, other_targets: &std::collections::HashSet<String>) {
    }

    /// java: @Override public List<String> getMixins()
    pub fn get_mixins(&self) -> Option<Vec<String>> {
        return None;
    }

    /// java: @Override public void preApply(String targetClassName, ClassNode targetClass, String mixinClassName, IMixinInfo mixinInfo)
    pub fn pre_apply(&self, target_class_name: &str, target_class: &crate::unresolved::ClassNode, mixin_class_name: &str, mixin_info: &crate::unresolved::IMixinInfo) {
    }

    /// java: @Override public void postApply(String targetClassName, ClassNode targetClass, String mixinClassName, IMixinInfo mixinInfo)
    pub fn post_apply(&self, target_class_name: &str, target_class: &crate::unresolved::ClassNode, mixin_class_name: &str, mixin_info: &crate::unresolved::IMixinInfo) {
    }

}
