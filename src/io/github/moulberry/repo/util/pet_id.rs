//! Ported from io.github.moulberry.repo.util.PetId (neurepoparser 1.12.0).
//!
//! java: @Documented @Target({PARAMETER, FIELD, LOCAL_VARIABLE, METHOD, TYPE_USE}) @Retention(CLASS) public @interface PetId
//!
//! Indicates that this string is a pet id. Unlike `NEUId`, this pet id does not include the `;RARITY` postfix. So a
//! `PetId` `String` would be `"BEE"`, while a `NEUId` `String` would be `"BEE;2"`.
//!
//! A class-retention annotation has no run-time behaviour; the port names it as a documenting type alias.

/// A pet id (`@PetId String`), e.g. `"BEE"`.
pub type PetId = String;
