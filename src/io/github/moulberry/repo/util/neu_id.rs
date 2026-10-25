//! Ported from io.github.moulberry.repo.util.NEUId (neurepoparser 1.12.0).
//!
//! java: @Documented @Target({PARAMETER, FIELD, LOCAL_VARIABLE, METHOD, TYPE_USE}) @Retention(CLASS) public @interface NEUId
//!
//! Indicates that this string is a neu internalname / neu id. When applied to a method, indicates the return type is a
//! neu id. When applied to a parameter, field, or local variable indicated that the variable type is a neu id.
//!
//! A class-retention annotation has no run-time behaviour; Rust has no type-use annotations, so the port names it as
//! a type alias that documents the same thing where it is written.

/// A NEU id (`@NEUId String`), e.g. `"BEE;2"`.
pub type NEUId = String;
