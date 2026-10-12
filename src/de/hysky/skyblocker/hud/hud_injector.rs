//! Ported from de.hysky.skyblocker.hud.HudInjector by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

// TODO(fabric2seer): static call ClassDesc.of(..)
//   java: private static final ClassDesc WIDGETS_MANAGER_DESC = ClassDesc.of("de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager");
/// java: private static final ClassDesc WIDGETS_MANAGER_DESC = ClassDesc.of("de.hysky.skyblocker.skyblock.tabhud.screenbuilder.WidgetManager");
pub static WIDGETS_MANAGER_DESC: compat::MainThread<crate::unresolved::ClassDesc> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `WIDGETS_MANAGER_DESC` - see the TODO above"));
// TODO(fabric2seer): static call MethodTypeDesc.of(..)
//   java: private static final MethodReference INIT_WIDGETS_METHOD_REFERENCE = new MethodReference(WIDGETS_MANAGER_DESC, "instantiateWidgets", MethodTypeDesc.of(ConstantDescs.CD_void), false);
/// java: private static final MethodReference INIT_WIDGETS_METHOD_REFERENCE = new MethodReference(WIDGETS_MANAGER_DESC, "instantiateWidgets", MethodTypeDesc.of(ConstantDescs.CD_void), false);
pub static INIT_WIDGETS_METHOD_REFERENCE: compat::MainThread<MethodReference> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `INIT_WIDGETS_METHOD_REFERENCE` - see the TODO above"));

/// java: public class HudInjector
#[derive(Clone)]
pub struct HudInjector {}

impl HudInjector {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static byte[] transformClass(ClassFile context, ClassModel classModel, List<ClassDesc> widgetClasses)
    pub fn transform_class(context: &crate::unresolved::ClassFile, class_model: &crate::unresolved::ClassModel, widget_classes: &Vec<crate::unresolved::ClassDesc>) -> Vec<i8> {
        // TODO(fabric2seer): `.transformClass(..)` on a ClassFile
        //   java: byte[] newBytes = context.transformClass(classModel, ClassTransform.transformingMethods(INIT_WIDGETS_METHOD_REFERENCE::matches, (methodBuilder, methodElement) -> {
        //   java: 			if (methodElement instanceof CodeModel) {
        //   java: 				methodBuilder.withCode(codeBuilder -> {
        //   java: 					for (ClassDesc widgetClass : widgetClasses) {
        //   java: 						codeBuilder.new_(widgetClass);
        //   java: 						codeBuilder.dup();
        //   java: 						codeBuilder.invokespecial(widgetClass, "<init>", MethodTypeDesc.of(ConstantDescs.CD_void));
        //   java: 						codeBuilder.invokestatic(WIDGETS_MANAGER_DESC, "addWidgetInstance", MethodTypeDesc.of(ConstantDescs.CD_void, ClassDesc.of("de.hysky.skyblocker.skyblock.tabhud.widget.HudWidget")));
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
