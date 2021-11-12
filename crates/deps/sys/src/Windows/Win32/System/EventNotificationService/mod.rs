#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableA(lpszdestination: super::super::Foundation::PSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableW(lpszdestination: super::super::Foundation::PWSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
}
pub const CONNECTION_AOL: u32 = 4u32;
#[repr(transparent)]
pub struct ISensLogon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensLogon2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensOnNow(pub *mut ::core::ffi::c_void);
pub const NETWORK_ALIVE_AOL: u32 = 4u32;
pub const NETWORK_ALIVE_INTERNET: u32 = 8u32;
pub const NETWORK_ALIVE_LAN: u32 = 1u32;
pub const NETWORK_ALIVE_WAN: u32 = 2u32;
#[repr(C)]
pub struct QOCINFO(i32);
#[repr(C)]
pub struct SENS(i32);
pub const SENSGUID_EVENTCLASS_LOGON: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477296, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_LOGON2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477328, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_NETWORK: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477280, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_EVENTCLASS_ONNOW: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3583477312, data2: 23455, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_PUBLISHER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1609440214, data2: 23451, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_LCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3549661872, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
pub const SENSGUID_SUBSCRIBER_WININET: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3549661877, data2: 23453, data3: 4561, data4: [141, 210, 0, 170, 0, 74, 189, 94] };
#[repr(transparent)]
pub struct SENS_CONNECTION_TYPE(pub u32);
pub const CONNECTION_LAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(0u32);
pub const CONNECTION_WAN: SENS_CONNECTION_TYPE = SENS_CONNECTION_TYPE(1u32);
#[repr(C)]
pub struct SENS_QOCINFO(i32);