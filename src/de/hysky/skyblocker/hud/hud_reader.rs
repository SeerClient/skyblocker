//! Ported from de.hysky.skyblocker.hud.HudReader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc REGISTER_WIDGET_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.RegisterWidget");
/// java: private static final ClassDesc REGISTER_WIDGET_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.RegisterWidget");
pub static REGISTER_WIDGET_ANNOTATION_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `REGISTER_WIDGET_ANNOTATION_DESC` - see the TODO above"));
// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc HUD_WIDGET_DESC = ClassDesc.of("de.hysky.skyblocker.skyblock.tabhud.widget.HudWidget");
/// java: private static final ClassDesc HUD_WIDGET_DESC = ClassDesc.of("de.hysky.skyblocker.skyblock.tabhud.widget.HudWidget");
pub static HUD_WIDGET_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `HUD_WIDGET_DESC` - see the TODO above"));
// TODO(fabric2seer): static call MethodTypeDesc.of(..)
//   java: private static final MethodTypeDesc DEFAULT_CONSTRUCTOR_DESC = MethodTypeDesc.of(ConstantDescs.CD_void);
/// java: private static final MethodTypeDesc DEFAULT_CONSTRUCTOR_DESC = MethodTypeDesc.of(ConstantDescs.CD_void);
pub static DEFAULT_CONSTRUCTOR_DESC: compat::MainThread<crate::unresolved::MethodTypeDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `DEFAULT_CONSTRUCTOR_DESC` - see the TODO above"));

/// java: public class HudReader
#[derive(Clone)]
pub struct HudReader {}

impl HudReader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void readClass(ClassModel classModel, Map<ClassDesc, Integer> registeredHudWidgets)
    pub fn read_class(class_model: &crate::unresolved::ClassModel, registered_hud_widgets: &mut std::collections::HashMap<crate::unresolved::ClassDesc, i32>) {
        // TODO(fabric2seer): `.findAttribute(..)` on a ClassModel
        //   java: Optional<RuntimeInvisibleAnnotationsAttribute> runtimeInvisibleAnnotationsOpt = classModel.findAttribute(Attributes.runtimeInvisibleAnnotations());
        let mut runtime_invisible_annotations_opt: Option<crate::unresolved::RuntimeInvisibleAnnotationsAttribute> = todo!("fabric2seer: the initialiser of `runtimeInvisibleAnnotationsOpt` (TODO above)");
        if runtime_invisible_annotations_opt.is_some() {
            let mut runtime_invisible_annotations: crate::unresolved::RuntimeInvisibleAnnotationsAttribute = runtime_invisible_annotations_opt.clone().unwrap();
            // TODO(fabric2seer): `.annotations(..)` on a RuntimeInvisibleAnnotationsAttribute
            //   java: for (Annotation annotation : runtimeInvisibleAnnotations.annotations()) {
            //   java: 				if (annotation.classSymbol().equals(REGISTER_WIDGET_ANNOTATION_DESC)) {
            //   java: 					checkSuperClass(classModel);
            //   java: 					checkForDefaultConstructor(classModel);
            //   java:
            //   java: 					AnnotationValue priorityValue = annotation.elements().stream()
            //   java: 							.filter(annotationElement -> annotationElement.name().equalsString("priority"))
            //   java: 							.map(AnnotationElement::value)
            //   java: 							.findAny()
            //   java: 							.orElse(null);
            //   java: 					int priority = priorityValue instanceof AnnotationValue.OfInt intValue ? intValue.intValue() : 0;
            //   java:
            //   java: 					registeredHudWidgets.put(classModel.thisClass().asSymbol(), priority);
            //   java: 				}
            //   java: 			}
        }
    }

    /// java: private static void checkSuperClass(ClassModel classModel)
    pub fn check_super_class(class_model: &crate::unresolved::ClassModel) {
        // TODO(fabric2seer): the condition: `.superclass(..)` on a ClassModel
        //   java: classModel.superclass().isPresent() && Processor.instanceOf(classModel.thisClass().asSymbol(), HUD_WIDGET_DESC)
        if todo!("fabric2seer: the condition (TODO above)") {
            return;
        }
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: throw new IllegalStateException("Class " + classModel.thisClass().asSymbol().packageName() + "is annotated with @RegisterWidget but does not extend HudWidget!");
    }

    /// java: private static void checkForDefaultConstructor(ClassModel classModel)
    pub fn check_for_default_constructor(class_model: &crate::unresolved::ClassModel) {
        // TODO(fabric2seer): `.methods(..)` on a ClassModel
        //   java: for (MethodModel methodModel : classModel.methods()) {
        //   java: 			if (methodModel.flags().has(AccessFlag.PUBLIC) && methodModel.methodName().equalsString("<init>") && methodModel.methodTypeSymbol().equals(DEFAULT_CONSTRUCTOR_DESC)) {
        //   java: 				return;
        //   java: 			}
        //   java: 		}
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: throw new IllegalStateException("Did not find a public, no-args constructor found for " + classModel.thisClass().asSymbol().displayName() + "!");
    }

}
