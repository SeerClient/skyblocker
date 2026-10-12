//! Ported from de.hysky.skyblocker.object.ObjectMethodReader by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::object::object_method_generation::ObjectMethodGeneration;
use crate::de::hysky::skyblocker::object::object_method_type::ObjectMethodType;
use crate::de::hysky::skyblocker::utils::field_reference::FieldReference;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc EQUALS_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenEquals");
/// java: private static final ClassDesc EQUALS_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenEquals");
pub static EQUALS_ANNOTATION_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `EQUALS_ANNOTATION_DESC` - see the TODO above"));
// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc HASH_CODE_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenHashCode");
/// java: private static final ClassDesc HASH_CODE_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenHashCode");
pub static HASH_CODE_ANNOTATION_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `HASH_CODE_ANNOTATION_DESC` - see the TODO above"));
// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc TO_STRING_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenToString");
/// java: private static final ClassDesc TO_STRING_ANNOTATION_DESC = ClassDesc.of("de.hysky.skyblocker.annotations.GenToString");
pub static TO_STRING_ANNOTATION_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TO_STRING_ANNOTATION_DESC` - see the TODO above"));
// TODO(fabric2seer): static call MethodTypeDesc.of(..)
//   java: private static final MethodTypeDesc EQUALS_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_boolean, ConstantDescs.CD_Object);
/// java: private static final MethodTypeDesc EQUALS_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_boolean, ConstantDescs.CD_Object);
pub static EQUALS_METHOD_DESC: compat::MainThread<crate::unresolved::MethodTypeDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `EQUALS_METHOD_DESC` - see the TODO above"));
// TODO(fabric2seer): static call MethodTypeDesc.of(..)
//   java: private static final MethodTypeDesc HASH_CODE_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_int);
/// java: private static final MethodTypeDesc HASH_CODE_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_int);
pub static HASH_CODE_METHOD_DESC: compat::MainThread<crate::unresolved::MethodTypeDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `HASH_CODE_METHOD_DESC` - see the TODO above"));
// TODO(fabric2seer): static call MethodTypeDesc.of(..)
//   java: private static final MethodTypeDesc TO_STRING_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_String);
/// java: private static final MethodTypeDesc TO_STRING_METHOD_DESC = MethodTypeDesc.of(ConstantDescs.CD_String);
pub static TO_STRING_METHOD_DESC: compat::MainThread<crate::unresolved::MethodTypeDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `TO_STRING_METHOD_DESC` - see the TODO above"));

/// java: public class ObjectMethodReader
#[derive(Clone)]
pub struct ObjectMethodReader {}

impl ObjectMethodReader {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static void readClass(ClassModel classModel, Consumer<List<ObjectMethodGeneration>> mapAdder)
    pub fn read_class(class_model: &crate::unresolved::ClassModel, map_adder: Arc<dyn Fn(Vec<ObjectMethodGeneration>)>) {
        let mut target_methods: std::collections::HashMap<MethodReference, (ObjectMethodType, bool)> = std::collections::HashMap::new();
        // TODO(fabric2seer): `.methods(..)` on a ClassModel
        //   java: for (MethodModel methodModel : classModel.methods()) {
        //   java: 			Optional<RuntimeInvisibleAnnotationsAttribute> runtimeInvisibleAnnotationsOpt = methodModel.findAttribute(Attributes.runtimeInvisibleAnnotations());
        //   java:
        //   java: 			if (runtimeInvisibleAnnotationsOpt.isPresent()) {
        //   java: 				RuntimeInvisibleAnnotationsAttribute runtimeInvisibleAnnotations = runtimeInvisibleAnnotationsOpt.get();
        //   java:
        //   java: 				for (Annotation annotation : runtimeInvisibleAnnotations.annotations()) {
        //   java: 					ClassDesc annotationDesc = annotation.classSymbol();
        //   java:
        //   java: 					ObjectMethodType objectMethodType = switch (annotationDesc) {
        //   java: 						case ClassDesc _ when annotationDesc.equals(EQUALS_ANNOTATION_DESC) -> ObjectMethodType.EQUALS;
        //   java: 						case ClassDesc _ when annotationDesc.equals(HASH_CODE_ANNOTATION_DESC) -> ObjectMethodType.HASH_CODE;
        //   java: 						case ClassDesc _ when annotationDesc.equals(TO_STRING_ANNOTATION_DESC) -> ObjectMethodType.TO_STRING;
        //   java: 						default -> null;
        //   java: 					};
        //   java:
        //   java: 					if (objectMethodType != null) {
        //   java: 						checkMethodDesc(methodModel, objectMethodType);
        //   java:
        //   java: 						if (methodModel.flags().has(AccessFlag.ABSTRACT)) {
        //   java: 							throw new IllegalStateException("Methods that generate an Object method must not be abstract! Use the native modifier if you do not want a stub method body.");
        //   java: 						}
        //   java:
        //   java: 						AnnotationValue includeSuperValue = annotation.elements().stream()
        //   java: 								.filter(annotationElement -> annotationElement.name().equalsString("includeSuper"))
        //   java: 								.map(AnnotationElement::value)
        //   java: 								.findAny()
        //   java: 								.orElse(null);
        //   java: 						boolean includeSuper = includeSuperValue instanceof AnnotationValue.OfBoolean booleanValue && booleanValue.booleanValue();
        //   java:
        //   java: 						targetMethods.put(MethodReference.fromModel(methodModel), Pair.of(objectMethodType, includeSuper));
        //   java: 					}
        //   java: 				}
        //   java: 			}
        //   java: 		}
        if target_methods.is_empty() {
            return;
        }
        // TODO(fabric2seer): `.fields(..)` on a ClassModel
        //   java: List<FieldReference> fieldReferences = classModel.fields().stream()
        //   java: 				.filter(fieldModel -> !fieldModel.flags().has(AccessFlag.STATIC) && !fieldModel.flags().has(AccessFlag.TRANSIENT))
        //   java: 				.map(FieldReference::fromModel)
        //   java: 				.toList();
        let mut field_references: Vec<FieldReference> = todo!("fabric2seer: the initialiser of `fieldReferences` (TODO above)");
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: ClassDesc classDesc = classModel.thisClass().asSymbol();
        let mut class_desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `classDesc` (TODO above)");
        // TODO(fabric2seer): `.superclass(..)` on a ClassModel
        //   java: ClassDesc superClassDesc = classModel.superclass().map(ClassEntry::asSymbol).orElse(null);
        let mut super_class_desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `superClassDesc` (TODO above)");
        let mut methods_to_generate: Vec<ObjectMethodGeneration> = target_methods.iter().map(|(k, v)| (k.clone(), v.clone())).map(|entry| ObjectMethodGeneration::new(class_desc.clone(), entry.0.clone(), entry.1.0, field_references.clone(), (if entry.1.1 { Some(super_class_desc.clone()) } else { None }))).collect::<Vec<_>>();
        (map_adder)(methods_to_generate.clone());
    }

    /// java: private static void checkMethodDesc(MethodModel methodModel, ObjectMethodType objectMethodType)
    pub fn check_method_desc(method_model: &crate::unresolved::MethodModel, mut object_method_type: ObjectMethodType) {
        let mut required_descriptor: crate::unresolved::MethodTypeDesc = match object_method_type {
            ObjectMethodType::Equals => EQUALS_METHOD_DESC.get_mut().clone(),
            ObjectMethodType::HashCode => HASH_CODE_METHOD_DESC.get_mut().clone(),
            ObjectMethodType::ToString => TO_STRING_METHOD_DESC.get_mut().clone(),
            _ => unreachable!("switch expression: no case matched"),
        };
        // TODO(fabric2seer): the condition: `.methodTypeSymbol(..)` on a MethodModel
        //   java: !methodModel.methodTypeSymbol().equals(requiredDescriptor)
        if todo!("fabric2seer: the condition (TODO above)") {
            // TODO(fabric2seer): `.methodName(..)` on a MethodModel
            //   java: throw new RuntimeException(String.format("Method '%s' has a mismatched descriptor! Expected: '%s'.", methodModel.methodName().stringValue(), requiredDescriptor.descriptorString()));
        }
    }

}
