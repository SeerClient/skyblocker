//! Ported from de.hysky.skyblocker.init.InitReader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc INIT_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.Init");
/// java: private static final ClassDesc INIT_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.Init");
pub static INIT_ANNOTATION_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `INIT_ANNOTATION_DESC` - see the TODO above"));

/// java: public class InitReader
#[derive(Clone)]
pub struct InitReader {}

impl InitReader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void readClass(ClassModel classModel, Map<MethodReference, Integer> initMethodReferences)
    pub fn read_class(class_model: &crate::unresolved::ClassModel, init_method_references: &mut std::collections::HashMap<MethodReference, i32>) {
        // TODO(fabric2seer): `.methods(..)` on a ClassModel
        //   java: for (MethodModel methodModel : classModel.methods()) {
        //   java: 			Optional<RuntimeInvisibleAnnotationsAttribute> runtimeInvisibleAnnotationsOpt = methodModel.findAttribute(Attributes.runtimeInvisibleAnnotations());
        //   java:
        //   java: 			if (runtimeInvisibleAnnotationsOpt.isPresent()) {
        //   java: 				RuntimeInvisibleAnnotationsAttribute runtimeInvisibleAnnotations = runtimeInvisibleAnnotationsOpt.get();
        //   java:
        //   java: 				for (Annotation annotation : runtimeInvisibleAnnotations.annotations()) {
        //   java: 					if (annotation.classSymbol().equals(INIT_ANNOTATION_DESC)) {
        //   java: 						// Ensure that this is a valid init method
        //   java: 						checkInitMethod(methodModel);
        //   java:
        //   java: 						AnnotationValue priorityValue = annotation.elements().stream()
        //   java: 								.filter(annotationElement -> annotationElement.name().equalsString("priority"))
        //   java: 								.map(AnnotationElement::value)
        //   java: 								.findAny()
        //   java: 								.orElse(null);
        //   java: 						int priority = priorityValue instanceof AnnotationValue.OfInt intValue ? intValue.intValue() : 0;
        //   java: 						MethodReference methodReference = MethodReference.fromModel(methodModel);
        //   java:
        //   java: 						initMethodReferences.put(methodReference, priority);
        //   java: 					}
        //   java: 				}
        //   java: 			}
        //   java: 		}
    }

    /// java: private static void checkInitMethod(MethodModel methodModel)
    pub fn check_init_method(method_model: &crate::unresolved::MethodModel) {
        // TODO(fabric2seer): the condition: `.flags(..)` on a MethodModel
        //   java: !methodModel.flags().has(AccessFlag.PUBLIC)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.toDebugString(..)` on a MethodModel
            //   java: throw new IllegalStateException(methodModel.toDebugString() + ": Initializer methods must be public");
        }
        // TODO(fabric2seer): the condition: `.flags(..)` on a MethodModel
        //   java: !methodModel.flags().has(AccessFlag.STATIC)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.toDebugString(..)` on a MethodModel
            //   java: throw new IllegalStateException(methodModel.toDebugString() + ": Initializer methods must be static");
        }
        // TODO(fabric2seer): `.methodTypeSymbol(..)` on a MethodModel
        //   java: MethodTypeDesc typeDesc = methodModel.methodTypeSymbol();
        let mut type_desc: crate::unresolved::MethodTypeDesc = todo!("fabric2seer: the initialiser of `typeDesc` (TODO above)");
        // TODO(fabric2seer): the condition: `.parameterCount(..)` on a MethodTypeDesc
        //   java: typeDesc.parameterCount() > 0 || !typeDesc.returnType().equals(ConstantDescs.CD_void)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.toDebugString(..)` on a MethodModel
            //   java: throw new IllegalStateException(methodModel.toDebugString() + ": Initializer methods must have no arguments and a void return type");
        }
    }

}
