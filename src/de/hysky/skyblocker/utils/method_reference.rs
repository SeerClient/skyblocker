//! Ported from de.hysky.skyblocker.utils.MethodReference by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record MethodReference(ClassDesc classDesc, String methodName, MethodTypeDesc typeDesc, boolean itf)
pub struct MethodReference {
    /// java: ClassDesc classDesc
    pub class_desc: crate::unresolved::ClassDesc,
    /// java: String methodName
    pub method_name: String,
    /// java: MethodTypeDesc typeDesc
    pub type_desc: crate::unresolved::MethodTypeDesc,
    /// java: boolean itf
    pub itf: bool,
}

impl MethodReference {
    /// java: the canonical record constructor
    pub fn new(mut class_desc: crate::unresolved::ClassDesc, mut method_name: String, mut type_desc: crate::unresolved::MethodTypeDesc, mut itf: bool) -> Self {
        Self { class_desc, method_name, type_desc, itf }
    }

    /// java: the record accessor `classDesc()`
    pub fn class_desc(&self) -> &crate::unresolved::ClassDesc {
        &self.class_desc
    }

    /// java: the record accessor `methodName()`
    pub fn method_name(&self) -> String {
        self.method_name.clone()
    }

    /// java: the record accessor `typeDesc()`
    pub fn type_desc(&self) -> &crate::unresolved::MethodTypeDesc {
        &self.type_desc
    }

    /// java: the record accessor `itf()`
    pub fn itf(&self) -> bool {
        self.itf
    }

    /// java: public static MethodReference fromModel(MethodModel methodModel)
    pub fn from_model(method_model: &crate::unresolved::MethodModel) -> MethodReference {
        // TODO(fabric2seer): `.parent(..)` on a MethodModel
        //   java: ClassModel parentClassModel = methodModel.parent().get();
        let mut parent_class_model: crate::unresolved::ClassModel = todo!("fabric2seer: the initialiser of `parentClassModel` (TODO above)");
        // TODO(fabric2seer): `.thisClass(..)` on a ClassModel
        //   java: ClassDesc classDesc = parentClassModel.thisClass().asSymbol();
        let mut class_desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `classDesc` (TODO above)");
        // TODO(fabric2seer): `.methodName(..)` on a MethodModel
        //   java: String methodName = methodModel.methodName().stringValue();
        let mut method_name: String = todo!("fabric2seer: the initialiser of `methodName` (TODO above)");
        // TODO(fabric2seer): `.methodTypeSymbol(..)` on a MethodModel
        //   java: MethodTypeDesc typeDesc = methodModel.methodTypeSymbol();
        let mut type_desc: crate::unresolved::MethodTypeDesc = todo!("fabric2seer: the initialiser of `typeDesc` (TODO above)");
        // TODO(fabric2seer): `.flags(..)` on a ClassModel
        //   java: boolean itf = parentClassModel.flags().has(AccessFlag.INTERFACE);
        let mut itf: bool = todo!("fabric2seer: the initialiser of `itf` (TODO above)");
        return MethodReference::new(class_desc, method_name, type_desc, itf);
    }

    /// java: public boolean matches(MethodModel methodModel)
    pub fn matches(&self, method_model: &crate::unresolved::MethodModel) -> bool {
        // TODO(fabric2seer): `.parent(..)` on a MethodModel
        //   java: return methodModel.parent()
        //   java: 				.map(classModel -> classModel.thisClass().asSymbol())
        //   java: 				.map(this.classDesc::equals).orElse(true) &&
        //   java: 				this.methodName.equals(methodModel.methodName().stringValue()) &&
        //   java: 				this.typeDesc.equals(methodModel.methodTypeSymbol()) &&
        //   java: 				this.itf == methodModel.parent().map(classModel -> classModel.flags().has(AccessFlag.INTERFACE)).orElse(true);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
