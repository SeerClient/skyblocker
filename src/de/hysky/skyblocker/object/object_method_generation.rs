//! Ported from de.hysky.skyblocker.object.ObjectMethodGeneration by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::de::hysky::skyblocker::object::object_method_type::ObjectMethodType;
use crate::de::hysky::skyblocker::utils::field_reference::FieldReference;
use crate::de::hysky::skyblocker::utils::method_reference::MethodReference;

/// java: public record ObjectMethodGeneration(ClassDesc classDesc, MethodReference target, ObjectMethodType objectMethodType, List<FieldReference> fieldReferences, Optional<ClassDesc> superClassDesc)
pub struct ObjectMethodGeneration {
    /// java: ClassDesc classDesc
    pub class_desc: crate::unresolved::ClassDesc,
    /// java: MethodReference target
    pub target: MethodReference,
    /// java: ObjectMethodType objectMethodType
    pub object_method_type: ObjectMethodType,
    /// java: List<FieldReference> fieldReferences
    pub field_references: Vec<FieldReference>,
    /// java: Optional<ClassDesc> superClassDesc
    pub super_class_desc: Option<crate::unresolved::ClassDesc>,
}

impl ObjectMethodGeneration {
    /// java: the canonical record constructor
    pub fn new(mut class_desc: crate::unresolved::ClassDesc, mut target: MethodReference, mut object_method_type: ObjectMethodType, mut field_references: Vec<FieldReference>, mut super_class_desc: Option<crate::unresolved::ClassDesc>) -> Self {
        Self { class_desc, target, object_method_type, field_references, super_class_desc }
    }

    /// java: the record accessor `classDesc()`
    pub fn class_desc(&self) -> crate::unresolved::ClassDesc {
        self.class_desc.clone()
    }

    /// java: the record accessor `target()`
    pub fn target(&self) -> MethodReference {
        self.target.clone()
    }

    /// java: the record accessor `objectMethodType()`
    pub fn object_method_type(&self) -> ObjectMethodType {
        self.object_method_type
    }

    /// java: the record accessor `fieldReferences()`
    pub fn field_references(&self) -> Vec<FieldReference> {
        self.field_references.clone()
    }

    /// java: the record accessor `superClassDesc()`
    pub fn super_class_desc(&self) -> Option<crate::unresolved::ClassDesc> {
        self.super_class_desc.clone()
    }

}
