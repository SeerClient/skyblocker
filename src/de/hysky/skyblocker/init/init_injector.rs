//! Ported from de.hysky.skyblocker.init.InitInjector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final MethodReference INIT_METHOD_REFERENCE = new MethodReference(ClassDesc.of("de.hysky.skyblocker.SkyblockerMod"), "init", MethodTypeDesc.of(ConstantDescs.CD_void), false);
/// java: private static final MethodReference INIT_METHOD_REFERENCE = new MethodReference(ClassDesc.of("de.hysky.skyblocker.SkyblockerMod"), "init", MethodTypeDesc.of(ConstantDescs.CD_void), false);
pub static INIT_METHOD_REFERENCE: compat::MainThread<MethodReference> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `INIT_METHOD_REFERENCE` - see the TODO above"));

/// java: public class InitInjector
#[derive(Clone)]
pub struct InitInjector {}

impl InitInjector {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static byte[] transformClass(ClassFile context, ClassModel classModel, List<MethodReference> initMethodReferences)
    pub fn transform_class(context: &crate::unresolved::ClassFile, class_model: &crate::unresolved::ClassModel, init_method_references: &Vec<MethodReference>) -> Vec<i8> {
        // TODO(fabric2seer): `.transformClass(..)` on a ClassFile
        //   java: byte[] newBytes = context.transformClass(classModel, ClassTransform.transformingMethods(INIT_METHOD_REFERENCE::matches, (methodBuilder, methodElement) -> {
        //   java: 			if (methodElement instanceof CodeModel) {
        //   java: 				methodBuilder.withCode(codeBuilder -> {
        //   java: 					for (MethodReference methodReference : initMethodReferences) {
        //   java: 						codeBuilder.invokestatic(methodReference.classDesc(), methodReference.methodName(), methodReference.typeDesc(), methodReference.itf());
        //   java: 					}
        //   java:
        //   java: 					codeBuilder.return_();
        //   java: 				});
        //   java: 			} else {
        //   java: 				methodBuilder.with(methodElement);
        //   java: 			}
        //   java: 		}));
        let mut new_bytes: Vec<i8> = todo!("fabric2seer: the initialiser of `newBytes` (TODO above)");
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: Processor.verifyClass(context, classModel.thisClass().asSymbol(), newBytes);
        return new_bytes;
    }

}
