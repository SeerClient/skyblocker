//! Ported from de.hysky.skyblocker.hud.HudProcessor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::hud::hud_reader::HudReader;
use crate::de::hysky::skyblocker::processor::Processor;

/// java: public class HudProcessor
#[derive(Clone)]
pub struct HudProcessor {}

impl HudProcessor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void apply(ClassFile context, Map<Path, ClassModel> classes)
    pub fn apply(context: &crate::unresolved::ClassFile, classes: &std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel>) {
        let mut start: i64 = compat::millis();
        let mut registered_hud_widgets: std::collections::HashMap<crate::unresolved::ClassDesc, i32> = std::collections::HashMap::new();
        for entry in classes.iter().map(|(k, v)| (k.clone(), v.clone())) {
            HudReader::read_class(&entry.1, &mut registered_hud_widgets);
        }
        let mut widget_classes: Vec<crate::unresolved::ClassDesc> = registered_hud_widgets.keys().cloned().collect::<Vec<_>>();
        widget_classes.sort_by(|a, b| { let key = |__a0| registered_hud_widgets.get(__a0).cloned(); key(a).cmp(&key(b)) });
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut widgets_manager_class_file: std::path::PathBuf = Processor::find_class_1("WidgetManager.class").unwrap().clone();
            // TODO(fabric2seer): `.parse(..)` on a ClassFile
            //   java: byte[] classBytes = HudInjector.transformClass(context, context.parse(widgetsManagerClassFile), widgetClasses);
            let mut class_bytes: Vec<i8> = todo!("fabric2seer: the initialiser of `classBytes` (TODO above)");
            Processor::write_class(&widgets_manager_class_file, &class_bytes);
        }));
        if let Err(e) = __result {
            panic!("RuntimeException: {}", compat::panic_message(&e));
        }
        println!("{}", format!("Injecting widget instancing took: {}ms", (compat::millis()).wrapping_sub(start)));
    }

}
