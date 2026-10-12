//! Ported from de.hysky.skyblocker.utils.FieldReference by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record FieldReference(ClassDesc classDesc, String fieldName, ClassDesc typeDesc)
pub struct FieldReference {
    /// java: ClassDesc classDesc
    pub class_desc: crate::unresolved::ClassDesc,
    /// java: String fieldName
    pub field_name: String,
    /// java: ClassDesc typeDesc
    pub type_desc: crate::unresolved::ClassDesc,
}

impl FieldReference {
    /// java: the canonical record constructor
    pub fn new(mut class_desc: crate::unresolved::ClassDesc, mut field_name: String, mut type_desc: crate::unresolved::ClassDesc) -> Self {
        Self { class_desc, field_name, type_desc }
    }

    /// java: the record accessor `classDesc()`
    pub fn class_desc(&self) -> crate::unresolved::ClassDesc {
        self.class_desc.clone()
    }

    /// java: the record accessor `fieldName()`
    pub fn field_name(&self) -> String {
        self.field_name.clone()
    }

    /// java: the record accessor `typeDesc()`
    pub fn type_desc(&self) -> crate::unresolved::ClassDesc {
        self.type_desc.clone()
    }

    /// java: public static FieldReference fromModel(FieldModel fieldModel)
    pub fn from_model(field_model: &crate::unresolved::FieldModel) -> FieldReference {
        // TODO(fabric2seer): `.parent(..)` on a FieldModel
        //   java: ClassDesc classDesc = fieldModel.parent().get().thisClass().asSymbol();
        let mut class_desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `classDesc` (TODO above)");
        // TODO(fabric2seer): `.fieldName(..)` on a FieldModel
        //   java: String fieldName = fieldModel.fieldName().stringValue();
        let mut field_name: String = todo!("fabric2seer: the initialiser of `fieldName` (TODO above)");
        // TODO(fabric2seer): `.fieldTypeSymbol(..)` on a FieldModel
        //   java: ClassDesc typeDesc = fieldModel.fieldTypeSymbol();
        let mut type_desc: crate::unresolved::ClassDesc = todo!("fabric2seer: the initialiser of `typeDesc` (TODO above)");
        return FieldReference::new(class_desc.clone(), field_name.clone(), type_desc.clone());
    }

    /// java: public boolean matches(FieldModel fieldModel)
    pub fn matches(&self, field_model: &crate::unresolved::FieldModel) -> bool {
        // TODO(fabric2seer): `.parent(..)` on a FieldModel
        //   java: return fieldModel.parent()
        //   java: 				.map(classModel -> classModel.thisClass().asSymbol())
        //   java: 				.map(this.classDesc::equals).orElse(true) &&
        //   java: 				this.fieldName.equals(fieldModel.fieldName().stringValue()) &&
        //   java: 				this.typeDesc.equals(fieldModel.fieldTypeSymbol());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
