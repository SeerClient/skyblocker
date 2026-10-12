//! Ported from de.hysky.skyblocker.object.ObjectMethodInjector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::object::object_method_generation::ObjectMethodGeneration;
use crate::de::hysky::skyblocker::object::object_method_type::ObjectMethodType;

/// java: public class ObjectMethodInjector
#[derive(Clone)]
pub struct ObjectMethodInjector {}

impl ObjectMethodInjector {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static byte[] transformClass(ClassFile context, ClassModel classModel, List<ObjectMethodGeneration> objectMethodsToGenerate)
    pub fn transform_class(context: &crate::unresolved::ClassFile, class_model: &crate::unresolved::ClassModel, object_methods_to_generate: &Vec<ObjectMethodGeneration>) -> Vec<i8> {
        let mut methods_to_transform: Arc<dyn Fn(crate::unresolved::MethodModel) -> bool> = Arc::new(move |method_model| object_methods_to_generate.iter().cloned().map(|__a0| __a0.target()).any(|method_reference| method_reference.matches(&method_model)));
        // TODO(fabric2seer): `.transformClass(..)` on a ClassFile
        //   java: byte[] newBytes = context.transformClass(classModel, (classBuilder, classElement) -> {
        //   java: 			if (classElement instanceof MethodModel methodModel && methodsToTransform.test(methodModel)) {
        //   java: 				// Find what method we are meant to generate
        //   java: 				ObjectMethodGeneration objectMethodToGenerate = objectMethodsToGenerate.stream()
        //   java: 						.filter(generation -> generation.target().matches(methodModel))
        //   java: 						.findFirst()
        //   java: 						.orElseThrow(() -> new IllegalStateException("Could not find generation for: " + methodModel.toDebugString()));
        //   java:
        //   java: 				// Transform the method
        //   java: 				classBuilder.transformMethod(methodModel, (methodBuilder, methodElement) -> {
        //   java: 					switch (methodElement) {
        //   java: 						case AccessFlags accessFlags -> {
        //   java: 							// Remove the native flag if present
        //   java: 							if (accessFlags.has(AccessFlag.NATIVE)) {
        //   java: 								// Copy the access flags set
        //   java: 								Set<AccessFlag> newAccessFlags = new HashSet<>(accessFlags.flags());
        //   java: 								newAccessFlags.remove(AccessFlag.NATIVE);
        //   java: 								methodBuilder.withFlags(newAccessFlags.toArray(AccessFlag[]::new));
        //   java:
        //   java: 								// If the method was declared native, it won't have a code attribute so we now need to add one
        //   java: 								// since our block below that looks for a CodeModel will not run
        //   java: 								methodBuilder.withCode(codeBuilder -> {
        //   java: 									loadLocalVariables(objectMethodToGenerate, codeBuilder);
        //   java: 									buildInvokeDynamic(objectMethodToGenerate, codeBuilder);
        //   java: 									addSuperClassComparisons(objectMethodToGenerate, codeBuilder);
        //   java: 								});
        //   java: 							}
        //   java: 						}
        //   java:
        //   java: 						// This will only run if the object method was not declared as native
        //   java: 						case CodeModel _ -> {
        //   java: 							methodBuilder.withCode(codeBuilder -> {
        //   java: 								loadLocalVariables(objectMethodToGenerate, codeBuilder);
        //   java: 								buildInvokeDynamic(objectMethodToGenerate, codeBuilder);
        //   java: 								addSuperClassComparisons(objectMethodToGenerate, codeBuilder);
        //   java: 							});
        //   java: 						}
        //   java:
        //   java: 						default -> {
        //   java: 							methodBuilder.with(methodElement);
        //   java: 						}
        //   java: 					}
        //   java: 				});
        //   java: 			} else {
        //   java: 				classBuilder.with(classElement);
        //   java: 			}
        //   java: 		});
        let mut new_bytes: Vec<i8> = todo!("fabric2seer: the initialiser of `newBytes` (TODO above)");
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: Processor.verifyClass(context, classModel.thisClass().asSymbol(), newBytes);
        return new_bytes;
    }

    /// java: private static void loadLocalVariables(ObjectMethodGeneration objectMethodToGenerate, CodeBuilder codeBuilder)
    pub fn load_local_variables(object_method_to_generate: &ObjectMethodGeneration, code_builder: &crate::unresolved::CodeBuilder) {
        // TODO(fabric2seer): `.aload(..)` on a CodeBuilder
        //   java: codeBuilder.aload(0);
        if object_method_to_generate.object_method_type() == ObjectMethodType::Equals {
            // TODO(fabric2seer): `.aload(..)` on a CodeBuilder
            //   java: codeBuilder.aload(1);
        }
    }

    /// java: private static void buildInvokeDynamic(ObjectMethodGeneration objectMethodToGenerate, CodeBuilder codeBuilder)
    pub fn build_invoke_dynamic(object_method_to_generate: &ObjectMethodGeneration, code_builder: &crate::unresolved::CodeBuilder) {
        let mut bootstrap_args_builder: Vec<crate::unresolved::ConstantDesc> = Vec::new();
        bootstrap_args_builder.push(object_method_to_generate.class_desc());
        bootstrap_args_builder.push(object_method_to_generate.field_references().into_iter().map(|__a0| __a0.field_name()).map(|s| s.to_string()).collect::<Vec<String>>().join(";"));
        // TODO(fabric2seer): static call MethodHandleDesc.of(..)
        //   java: bootstrapArgsBuilder.addAll(objectMethodToGenerate.fieldReferences().stream()
        //   java: 				.map(fieldReference -> MethodHandleDesc.of(
        //   java: 						DirectMethodHandleDesc.Kind.GETTER,
        //   java: 						objectMethodToGenerate.classDesc(),
        //   java: 						fieldReference.fieldName(),
        //   java: 						fieldReference.typeDesc().descriptorString()))
        //   java: 				.toList());
        // TODO(fabric2seer): static call MethodHandleDesc.of(..)
        //   java: DirectMethodHandleDesc bootstrapMethod = MethodHandleDesc.of(
        //   java: 				DirectMethodHandleDesc.Kind.STATIC,
        //   java: 				ClassDesc.of("java.lang.runtime.ObjectMethods"),
        //   java: 				"bootstrap",
        //   java: 				MethodTypeDesc.of(
        //   java: 						ConstantDescs.CD_Object,
        //   java: 						ConstantDescs.CD_MethodHandles_Lookup,
        //   java: 						ConstantDescs.CD_String,
        //   java: 						ClassDesc.of("java.lang.invoke.TypeDescriptor"),
        //   java: 						ConstantDescs.CD_Class,
        //   java: 						ConstantDescs.CD_String,
        //   java: 						ConstantDescs.CD_MethodHandle.arrayType()).descriptorString());
        let mut bootstrap_method: crate::unresolved::DirectMethodHandleDesc = todo!("fabric2seer: the initialiser of `bootstrapMethod` (TODO above)");
        // TODO(fabric2seer): static call MethodTypeDesc.of(..)
        //   java: MethodTypeDesc invocationType = switch (objectMethodToGenerate.objectMethodType()) {
        //   java: 			case ObjectMethodType.EQUALS -> MethodTypeDesc.of(ConstantDescs.CD_boolean, objectMethodToGenerate.classDesc(), ConstantDescs.CD_Object);
        //   java: 			case ObjectMethodType.HASH_CODE -> MethodTypeDesc.of(ConstantDescs.CD_int, objectMethodToGenerate.classDesc());
        //   java: 			case ObjectMethodType.TO_STRING -> MethodTypeDesc.of(ConstantDescs.CD_String, objectMethodToGenerate.classDesc());
        //   java: 		};
        let mut invocation_type: crate::unresolved::MethodTypeDesc = todo!("fabric2seer: the initialiser of `invocationType` (TODO above)");
        let mut bootstrap_args: Vec<crate::unresolved::ConstantDesc> = bootstrap_args_builder.clone();
        // TODO(fabric2seer): static call DynamicCallSiteDesc.of(..)
        //   java: DynamicCallSiteDesc callSite = DynamicCallSiteDesc.of(bootstrapMethod, objectMethodToGenerate.objectMethodType().methodName, invocationType, bootstrapArgs);
        let mut call_site: crate::unresolved::DynamicCallSiteDesc = todo!("fabric2seer: the initialiser of `callSite` (TODO above)");
        // TODO(fabric2seer): `.invokedynamic(..)` on a CodeBuilder
        //   java: codeBuilder.invokedynamic(callSite);
    }

    /// java: private static void addSuperClassComparisons(ObjectMethodGeneration objectMethodToGenerate, CodeBuilder codeBuilder)
    pub fn add_super_class_comparisons(object_method_to_generate: &ObjectMethodGeneration, code_builder: &crate::unresolved::CodeBuilder) {
        if object_method_to_generate.super_class_desc().is_some() {
            match object_method_to_generate.object_method_type() {
                ObjectMethodType::Equals => {
                    // TODO(fabric2seer): `.ifThenElse(..)` on a CodeBuilder
                    //   java: codeBuilder.ifThenElse(trueCodeBuilder -> {
                    //   java: 						trueCodeBuilder.aload(0);
                    //   java: 						trueCodeBuilder.aload(1);
                    //   java: 						trueCodeBuilder.invokespecial(objectMethodToGenerate.superClassDesc().get(), "equals", MethodTypeDesc.of(ConstantDescs.CD_boolean, ConstantDescs.CD_Object), false);
                    //   java: 						trueCodeBuilder.ireturn();
                    //   java: 					}, falseCodeBuilder -> {
                    //   java: 						falseCodeBuilder.iconst_0();
                    //   java: 						falseCodeBuilder.ireturn();
                    //   java: 					});
                }
                ObjectMethodType::HashCode => {
                    // TODO(fabric2seer): `.bipush(..)` on a CodeBuilder
                    //   java: codeBuilder.bipush(31);
                    // TODO(fabric2seer): `.imul(..)` on a CodeBuilder
                    //   java: codeBuilder.imul();
                    // TODO(fabric2seer): `.aload(..)` on a CodeBuilder
                    //   java: codeBuilder.aload(0);
                    // TODO(fabric2seer): `.invokespecial(..)` on a CodeBuilder
                    //   java: codeBuilder.invokespecial(objectMethodToGenerate.superClassDesc().get(), "hashCode", MethodTypeDesc.of(ConstantDescs.CD_int), false);
                    // TODO(fabric2seer): `.iadd(..)` on a CodeBuilder
                    //   java: codeBuilder.iadd();
                    // TODO(fabric2seer): `.ireturn(..)` on a CodeBuilder
                    //   java: codeBuilder.ireturn();
                }
                ObjectMethodType::ToString => {
                    panic!("UnsupportedOperationException: {}", "Cannot compute toString values for superclasses!");
                }
                _ => {}
            }
        } else {
            match object_method_to_generate.object_method_type() {
                ObjectMethodType::Equals | ObjectMethodType::HashCode => {
                    // TODO(fabric2seer): `.ireturn(..)` on a CodeBuilder
                    //   java: codeBuilder.ireturn();
                }
                ObjectMethodType::ToString => {
                    // TODO(fabric2seer): `.areturn(..)` on a CodeBuilder
                    //   java: codeBuilder.areturn();
                }
                _ => {}
            }
        }
    }

}
