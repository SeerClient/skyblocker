//! Ported from de.hysky.skyblocker.Processor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::net::url::Url;

/// java: public static @Nullable File classesDir;
pub static CLASSES_DIR: compat::MainThread<Option<std::path::PathBuf>> = compat::MainThread::new(|| None);
/// java: public static @Nullable URLClassLoader compileClasspathLoader;
pub static COMPILE_CLASSPATH_LOADER: compat::MainThread<Option<crate::unresolved::URLClassLoader>> = compat::MainThread::new(|| None);

/// java: public class Processor implements Plugin<Project>
#[derive(Clone)]
pub struct Processor {}

impl Processor {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: private static Map<Path, ClassModel> readSkyblockerClasses()
    pub fn read_skyblocker_classes() -> std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel> {
        let mut skyblocker_classes: std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel> = std::collections::HashMap::new();
        // TODO(fabric2seer): static call ClassFile.of(..)
        //   java: ClassFile classFile = ClassFile.of();
        let mut class_file: crate::unresolved::ClassFile = todo!("fabric2seer: the initialiser of `classFile` (TODO above)");
        Processor::for_each_class_bi_consumer(Arc::new(move |path, _| {
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.parse(..)` on a ClassFile
                //   java: ClassModel model = classFile.parse(path);
                let mut model: crate::unresolved::ClassModel = todo!("fabric2seer: the initialiser of `model` (TODO above)");
                skyblocker_classes.insert(path.clone(), model.clone());
            }));
            if let Err(e) = __result {
                logger().error(&format!("Failed to parse class {}", path.display()));
            }
        }));
        return skyblocker_classes;
    }

    /// java: private static ClassHierarchyResolver skyblockerClassHierarchyResolver(Map<Path, ClassModel> classes)
    pub fn skyblocker_class_hierarchy_resolver(classes: &std::collections::HashMap<std::path::PathBuf, crate::unresolved::ClassModel>) -> crate::unresolved::ClassHierarchyResolver {
        let mut interfaces: Vec<crate::unresolved::ClassDesc> = Vec::new();
        let mut class_to_super_class: std::collections::HashMap<crate::unresolved::ClassDesc, crate::unresolved::ClassDesc> = std::collections::HashMap::new();
        for model in classes.values().cloned() {
            // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
            //   java: ClassDesc desc = model.thisClass().asSymbol();
            let mut desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `desc` (TODO above)");
            // TODO(fabric2seer): the condition: `.flags(..)` on a ClassModel
            //   java: model.flags(..).has(..)
            if todo!("fabric2seer: the condition (TODO above)") {
                interfaces.push(desc.clone());
            }
            // TODO(fabric2seer): the condition: `.isModuleInfo(..)` on a ClassModel
            //   java: !model.isModuleInfo() && !desc.equals(ConstantDescs.CD_Object)
            if todo!("fabric2seer: the condition (TODO above)") {
                // TODO(fabric2seer): `.superclass(..)` on a ClassModel
                //   java: classToSuperClass.put(desc, model.superclass().get().asSymbol());
            }
        }
        // TODO(fabric2seer): static call ClassHierarchyResolver.of(..)
        //   java: return ClassHierarchyResolver.of(interfaces, classToSuperClass);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static ClassHierarchyResolver compileClasspathHierarchyResolver(JavaCompile javaCompile)
    pub fn compile_classpath_hierarchy_resolver(java_compile: &crate::unresolved::JavaCompile) -> crate::unresolved::ClassHierarchyResolver {
        // TODO(fabric2seer): `.getClasspath(..)` on a JavaCompile
        //   java: URL[] urls = javaCompile.getClasspath().getFiles().stream()
        //   java: 				.map(File::toURI)
        //   java: 				.map(uri -> {
        //   java: 					try {
        //   java: 						return uri.toURL();
        //   java: 					} catch (Exception e) {
        //   java: 						throw new RuntimeException(e);
        //   java: 					}
        //   java: 				})
        //   java: 				.toArray(URL[]::new);
        let mut urls: Vec<Url> = todo!("fabric2seer: the initialiser of `urls` (TODO above)");
        // TODO(fabric2seer): `new URLClassLoader(..)`
        //   java: compileClasspathLoader = new URLClassLoader(urls, null);
        // TODO(fabric2seer): static call ClassHierarchyResolver.ofResourceParsing(..)
        //   java: return ClassHierarchyResolver.ofResourceParsing(compileClasspathLoader);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: public static void forEachClass(File directory, BiConsumer<Path, InputStream> consumer)
    pub fn for_each_class(directory: &std::path::PathBuf, consumer: Arc<dyn Fn(std::path::PathBuf, crate::unresolved::InputStream)>) {
        // java: try {..} catch (IOException e) - an exception here is a panic, caught by `catch_unwind`
        let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // TODO(fabric2seer): Files.walkFileTree
            //   java: Files.walkFileTree(directory.toPath(), new SimpleFileVisitor<>() {
            //   java: 				@Override
            //   java: 				public FileVisitResult visitFile(Path path, BasicFileAttributes attrs) {
            //   java: 					if (!path.toString().endsWith(".class")) return FileVisitResult.CONTINUE;
            //   java:
            //   java: 					try (InputStream inputStream = Files.newInputStream(path)) {
            //   java: 						consumer.accept(path, inputStream);
            //   java: 					} catch (IOException e) {
            //   java: 						LOGGER.error("Failed to run consumer on class {}", path, e);
            //   java: 					}
            //   java:
            //   java: 					return FileVisitResult.CONTINUE;
            //   java: 				}
            //   java: 			});
        }));
        if let Err(e) = __result {
            logger().error(&format!("Failed to walk classes"));
        }
    }

    /// java: public static void forEachClass(BiConsumer<Path, InputStream> consumer)
    pub fn for_each_class_bi_consumer(consumer: Arc<dyn Fn(std::path::PathBuf, crate::unresolved::InputStream)>) {
        Processor::for_each_class(CLASSES_DIR.get_mut().as_ref().unwrap(), consumer.clone());
    }

    /// java: public static void forEachClass(Consumer<InputStream> consumer)
    pub fn for_each_class_consumer(consumer: Arc<dyn Fn(crate::unresolved::InputStream)>) {
        Processor::for_each_class_bi_consumer(Arc::new(move |_, input_stream| { (consumer)(input_stream.clone()); }));
    }

    /// java: private static @Nullable File findClass(File directory, String className)
    pub fn find_class(directory: &std::path::PathBuf, mut class_name: String) -> Option<std::path::PathBuf> {
        if !(class_name.ends_with(".class")) {
            class_name.push_str(".class");
        }
        if !directory.is_dir() {
            panic!("IllegalArgumentException: {}", "Not a directory");
        }
        // TODO(fabric2seer): Path.listFiles
        //   java: for (File file : Objects.requireNonNull(directory.listFiles())) {
        //   java: 			if (file.isDirectory()) {
        //   java: 				File foundFile = findClass(file, className);
        //   java:
        //   java: 				if (foundFile != null) return foundFile;
        //   java: 			} else if (file.getName().equals(className)) {
        //   java: 				return file;
        //   java: 			}
        //   java: 		}
        return None;
    }

    /// java: public static @Nullable File findClass(String className)
    pub fn find_class_1(class_name: &str) -> Option<std::path::PathBuf> {
        return Processor::find_class(CLASSES_DIR.get_mut().as_ref().unwrap(), class_name.to_string());
    }

    /// java: public static void writeClass(Path classFilePath, byte[] classBytes)
    pub fn write_class(class_file_path: &std::path::PathBuf, class_bytes: &Vec<i8>) {
        {
            // TODO(fabric2seer): Files.newOutputStream
            //   java: OutputStream outputStream = Files.newOutputStream(..);
            let mut output_stream: crate::unresolved::OutputStream = todo!("fabric2seer: the initialiser of `outputStream` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.write(..)` on a OutputStream
                //   java: outputStream.write(classBytes);
            }));
            if let Err(e) = __result {
                panic!("RuntimeException: {}", compat::panic_message(&e));
            }
        }
    }

    /// java: public static boolean instanceOf(ClassDesc child, ClassDesc superClass)
    pub fn instance_of(child: &crate::unresolved::ClassDesc, super_class: &crate::unresolved::ClassDesc) -> bool {
        // TODO(fabric2seer): static call ClassFile.of(..)
        //   java: ClassFile context = ClassFile.of();
        let mut context: crate::unresolved::ClassFile = todo!("fabric2seer: the initialiser of `context` (TODO above)");
        let mut start: std::path::PathBuf = CLASSES_DIR.get_mut().as_ref().unwrap().clone();
        let mut sup: Option<crate::unresolved::ClassDesc> = Some(child.clone());
        while sup.is_some() {
            if sup.clone().unwrap() == super_class {
                return true;
            }
            // TODO(fabric2seer): `.descriptorString(..)` on a ClassDesc
            //   java: Path resolve = start.resolve(sup.descriptorString().replace(";", "").replaceFirst("L", "") + ".class");
            let mut resolve: std::path::PathBuf = todo!("fabric2seer: the initialiser of `resolve` (TODO above)");
            // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // TODO(fabric2seer): `.parse(..)` on a ClassFile
                //   java: ClassModel classModel = context.parse(resolve);
                let mut class_model: crate::unresolved::ClassModel = todo!("fabric2seer: the initialiser of `classModel` (TODO above)");
                // TODO(fabric2seer): `.superclass(..)` on a ClassModel
                //   java: sup = classModel.superclass().get().asSymbol();
            }));
            if let Err(e) = __result {
                logger().error(&format!("Failed to read class {}", resolve.display()));
                return false;
            }
        }
        return false;
    }

    /// java: public static void verifyClass(ClassFile context, ClassDesc desc, byte[] newClassBytes)
    pub fn verify_class(context: &crate::unresolved::ClassFile, desc: &crate::unresolved::ClassDesc, new_class_bytes: &Vec<i8>) {
        // TODO(fabric2seer): `.verify(..)` on a ClassFile
        //   java: List<VerifyError> verifyErrors = context.verify(newClassBytes);
        let mut verify_errors: Vec<crate::unresolved::VerifyError> = todo!("fabric2seer: the initialiser of `verifyErrors` (TODO above)");
        for error in verify_errors.iter() {
            // TODO(fabric2seer): `.packageName(..)` on a ClassDesc
            //   java: System.out.println("Verify error for '%s': %s".formatted(desc.packageName(), error));
        }
    }

    /// java: @Override public void apply(Project project)
    pub fn apply(&self, project: &crate::unresolved::Project) {
        // TODO(fabric2seer): `.getTasks(..)` on a Project
        //   java: project.getTasks().withType(JavaCompile.class).named("compileJava").get().doLast(task -> {
        //   java: 			JavaCompile javaCompile = (JavaCompile) task;
        //   java: 			classesDir = javaCompile.getDestinationDirectory().get().getAsFile();
        //   java:
        //   java: 			// Used for quickly extracting information from the mod's classes (so we don't reparse them a bunch of times)
        //   java: 			// NB: These class models MUST NOT be used when injecting into the class otherwise transformations will not stack!!!
        //   java: 			Map<Path, ClassModel> skyblockerClasses = readSkyblockerClasses();
        //   java:
        //   java: 			// The ClassFile API needs the complete hierarchy of classes in use to construct correct bytecode, so we
        //   java: 			// create the appropriate resolvers that provide such information.
        //   java: 			ClassHierarchyResolver skyblockerClassHierarchyResolver = skyblockerClassHierarchyResolver(skyblockerClasses);
        //   java: 			ClassHierarchyResolver compileClassPathHierarchyResolver = compileClasspathHierarchyResolver(javaCompile);
        //   java: 			ClassHierarchyResolver completeClassHierarchyResolver = skyblockerClassHierarchyResolver
        //   java: 					.orElse(compileClassPathHierarchyResolver)
        //   java: 					.orElse(ClassHierarchyResolver.defaultResolver());
        //   java: 			ClassFile context = ClassFile.of(ClassHierarchyResolverOption.of(completeClassHierarchyResolver));
        //   java:
        //   java: 			// Apply processors
        //   java: 			InitProcessor.apply(context, skyblockerClasses);
        //   java: 			ObjectProcessor.apply(context, skyblockerClasses);
        //   java: 			HudProcessor.apply(context, skyblockerClasses);
        //   java:
        //   java: 			// Close URL class loader
        //   java: 			if (compileClasspathLoader != null) {
        //   java: 				try {
        //   java: 					compileClasspathLoader.close();
        //   java: 				} catch (IOException e) {
        //   java: 					throw new RuntimeException(e);
        //   java: 				}
        //   java: 			}
        //   java: 		});
    }

}
