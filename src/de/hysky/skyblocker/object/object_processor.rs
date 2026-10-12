//! Ported from de.hysky.skyblocker.object.ObjectProcessor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::object::object_method_generation::ObjectMethodGeneration;
use crate::de::hysky::skyblocker::object::object_method_reader::ObjectMethodReader;
use crate::de::hysky::skyblocker::processor::Processor;

/// java: public class ObjectProcessor
#[derive(Clone)]
pub struct ObjectProcessor {}

impl ObjectProcessor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void apply(ClassFile context, Map<Path, ClassModel> classes)
    pub fn apply(context: &crate::unresolved::ClassFile, classes: &std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel>) {
        let mut start: i64 = compat::millis();
        let mut methods_to_generate: std::collections::HashMap<std::path::PathBuf, Vec<ObjectMethodGeneration>> = std::collections::HashMap::new();
        for entry in classes.iter().map(|(k, v)| (k.clone(), v.clone())) {
            ObjectMethodReader::read_class(&entry.1, Arc::new(move |list| { methods_to_generate.insert(entry.0.clone(), list.clone()); }));
        }
        for entry in methods_to_generate.iter().map(|(k, v)| (k.clone(), v.clone())) {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.parse(..)` on a ClassFile
                //   java: byte[] classBytes = ObjectMethodInjector.transformClass(context, context.parse(entry.getKey()), entry.getValue());
                let mut class_bytes: Vec<i8> = todo!("fabric2seer: the initialiser of `classBytes` (TODO above)");
                Processor::write_class(&entry.0, &class_bytes);
            }));
            if let Err(e) = __result {
                panic!("RuntimeException: {}", compat::panic_message(&e));
            }
        }
        println!("{}", format!("Injecting object methods took: {}ms", (compat::millis()).wrapping_sub(start)));
    }

}
