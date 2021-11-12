#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ContentAccessRestrictionLevel(pub i32);
impl ContentAccessRestrictionLevel {
    pub const Allow: Self = Self(0i32);
    pub const Warn: Self = Self(1i32);
    pub const Block: Self = Self(2i32);
    pub const Hide: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentRestrictionsBrowsePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRatedContentRestrictionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatedContentCategory(pub i32);
impl RatedContentCategory {
    pub const General: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Game: Self = Self(2i32);
    pub const Movie: Self = Self(3i32);
    pub const Television: Self = Self(4i32);
    pub const Music: Self = Self(5i32);
}
#[repr(transparent)]
pub struct RatedContentDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RatedContentRestrictions(pub *mut ::core::ffi::c_void);