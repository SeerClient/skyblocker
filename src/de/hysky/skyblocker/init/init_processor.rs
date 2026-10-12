//! Ported from de.hysky.skyblocker.init.InitProcessor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::init::init_reader::InitReader;
use crate::de::hysky::skyblocker::processor::Processor;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

/// java: public class InitProcessor
#[derive(Clone)]
pub struct InitProcessor {}

impl InitProcessor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void apply(ClassFile context, Map<Path, ClassModel> classes)
    pub fn apply(context: &crate::unresolved::ClassFile, classes: &std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel>) {
        let mut start: i64 = compat::millis();
        let mut method_signatures: std::collections::HashMap<MethodReference, i32> = std::collections::HashMap::new();
        for entry in classes.iter().map(|(k, v)| (k.clone(), v.clone())) {
            InitReader::read_class(&entry.1, &mut method_signatures);
        }
        // TODO(fabric2seer): static field Map.Entry
        //   java: List<MethodReference> sortedMethodSignatures = methodSignatures.entrySet()
        //   java: 				.stream()
        //   java: 				.sorted(Map.Entry.<MethodReference, Integer>comparingByValue().thenComparing(entry -> entry.getKey().classDesc().packageName()))
        //   java: 				.map(Map.Entry::getKey)
        //   java: 				.toList();
        let mut sorted_method_signatures: Vec<MethodReference> = todo!("fabric2seer: the initialiser of `sortedMethodSignatures` (TODO above)");
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut main_class_file: std::path::PathBuf = Processor::find_class_1("SkyblockerMod.class").unwrap().clone();
            // TODO(fabric2seer): `.parse(..)` on a ClassFile
            //   java: byte[] classBytes = InitInjector.transformClass(context, context.parse(mainClassFile), sortedMethodSignatures);
            let mut class_bytes: Vec<i8> = todo!("fabric2seer: the initialiser of `classBytes` (TODO above)");
            Processor::write_class(&main_class_file, &class_bytes);
        }));
        if let Err(e) = __result {
            panic!("RuntimeException: {}", compat::panic_message(&e));
        }
        println!("{}", format!("Injecting init methods took: {}ms", (compat::millis()).wrapping_sub(start)));
    }

}
