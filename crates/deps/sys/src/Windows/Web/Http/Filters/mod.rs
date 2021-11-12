#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCacheControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: Self = Self(0i32);
    pub const MostRecent: Self = Self(1i32);
    pub const OnlyFromCache: Self = Self(2i32);
    pub const NoCache: Self = Self(3i32);
}
#[repr(transparent)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCache: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCookies: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpCacheControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);