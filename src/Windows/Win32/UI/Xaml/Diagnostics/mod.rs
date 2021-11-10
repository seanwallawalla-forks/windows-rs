#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BaseValueSource(pub i32);
pub const BaseValueSourceUnknown: BaseValueSource = BaseValueSource(0i32);
pub const BaseValueSourceDefault: BaseValueSource = BaseValueSource(1i32);
pub const BaseValueSourceBuiltInStyle: BaseValueSource = BaseValueSource(2i32);
pub const BaseValueSourceStyle: BaseValueSource = BaseValueSource(3i32);
pub const BaseValueSourceLocal: BaseValueSource = BaseValueSource(4i32);
pub const Inherited: BaseValueSource = BaseValueSource(5i32);
pub const DefaultStyleTrigger: BaseValueSource = BaseValueSource(6i32);
pub const TemplateTrigger: BaseValueSource = BaseValueSource(7i32);
pub const StyleTrigger: BaseValueSource = BaseValueSource(8i32);
pub const ImplicitStyleReference: BaseValueSource = BaseValueSource(9i32);
pub const ParentTemplate: BaseValueSource = BaseValueSource(10i32);
pub const ParentTemplateTrigger: BaseValueSource = BaseValueSource(11i32);
pub const Animation: BaseValueSource = BaseValueSource(12i32);
pub const Coercion: BaseValueSource = BaseValueSource(13i32);
pub const BaseValueSourceVisualState: BaseValueSource = BaseValueSource(14i32);
impl ::core::convert::From<i32> for BaseValueSource {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BaseValueSource {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Graphics_Dxgi_Common`*"]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for BitmapDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for BitmapDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BitmapDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("AlphaMode", &self.AlphaMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for BitmapDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.AlphaMode == other.AlphaMode
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::runtime::Abi for BitmapDescription {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub MetadataBits: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl CollectionElementValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CollectionElementValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CollectionElementValue {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CollectionElementValue").field("Index", &self.Index).field("ValueType", &self.ValueType).field("Value", &self.Value).field("MetadataBits", &self.MetadataBits).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CollectionElementValue {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.ValueType == other.ValueType && self.Value == other.Value && self.MetadataBits == other.MetadataBits
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CollectionElementValue {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CollectionElementValue {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
pub const E_UNKNOWNTYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2144665560i32 as _);
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`, `Win32_System_Com`*"]
pub struct EnumType {
    pub Name: super::super::super::Foundation::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl EnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for EnumType {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for EnumType {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EnumType").field("Name", &self.Name).field("ValueInts", &self.ValueInts).field("ValueStrings", &self.ValueStrings).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for EnumType {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.ValueInts == other.ValueInts && self.ValueStrings == other.ValueStrings
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for EnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::runtime::Abi for EnumType {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitmapData(pub ::windows::runtime::IUnknown);
impl IBitmapData {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sourceoffsetinbytes), ::core::mem::transmute(maxbytestocopy), ::core::mem::transmute(pvbytes), ::core::mem::transmute(numberofbytescopied)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetStride(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetBitmapDescription(&self) -> ::windows::runtime::Result<BitmapDescription> {
        let mut result__: <BitmapDescription as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BitmapDescription>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetSourceBitmapDescription(&self) -> ::windows::runtime::Result<BitmapDescription> {
        let mut result__: <BitmapDescription as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<BitmapDescription>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IBitmapData {
    type Vtable = IBitmapData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3517140722, 51928, 17973, [163, 210, 252, 218, 141, 63, 60, 175]);
}
impl ::core::convert::From<IBitmapData> for ::windows::runtime::IUnknown {
    fn from(value: IBitmapData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitmapData> for ::windows::runtime::IUnknown {
    fn from(value: &IBitmapData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IBitmapData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IBitmapData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstride: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitmapdescription: *mut BitmapDescription) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitmapdescription: *mut BitmapDescription) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualTreeService(pub ::windows::runtime::IUnknown);
impl IVisualTreeService {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualTreeService {
    type Vtable = IVisualTreeService_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2777919770, 53631, 18619, [143, 102, 131, 145, 7, 49, 200, 165]);
}
impl ::core::convert::From<IVisualTreeService> for ::windows::runtime::IUnknown {
    fn from(value: IVisualTreeService) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualTreeService> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualTreeService) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualTreeService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualTreeService {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, child: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualTreeService2(pub ::windows::runtime::IUnknown);
impl IVisualTreeService2 {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyIndex<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, object: u64, propertyname: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), propertyname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(propertyindex), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcedictionary), ::core::mem::transmute(key), ::core::mem::transmute(newvalue)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::runtime::Result<IBitmapData> {
        let mut result__: <IBitmapData as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(options), ::core::mem::transmute(maxpixelwidth), ::core::mem::transmute(maxpixelheight), &mut result__).from_abi::<IBitmapData>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVisualTreeService2 {
    type Vtable = IVisualTreeService2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(319770934, 60483, 20321, [137, 199, 152, 1, 163, 109, 46, 149]);
}
impl ::core::convert::From<IVisualTreeService2> for ::windows::runtime::IUnknown {
    fn from(value: IVisualTreeService2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualTreeService2> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualTreeService2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualTreeService2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualTreeService2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVisualTreeService2> for IVisualTreeService {
    fn from(value: IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService2> for IVisualTreeService {
    fn from(value: &IVisualTreeService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService> for IVisualTreeService2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService> for &IVisualTreeService2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, child: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualTreeService3(pub ::windows::runtime::IUnknown);
impl IVisualTreeService3 {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyIndex<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, object: u64, propertyname: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), propertyname.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(propertyindex), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcedictionary), ::core::mem::transmute(key), ::core::mem::transmute(newvalue)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::runtime::Result<IBitmapData> {
        let mut result__: <IBitmapData as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(options), ::core::mem::transmute(maxpixelwidth), ::core::mem::transmute(maxpixelheight), &mut result__).from_abi::<IBitmapData>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn ResolveResource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, resourcecontext: u64, resourcename: Param1, resourcetype: ResourceType, propertyindex: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcecontext), resourcename.into_param().abi(), ::core::mem::transmute(resourcetype), ::core::mem::transmute(propertyindex)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetDictionaryItem<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, dictionaryhandle: u64, resourcename: Param1, resourceisimplicitstyle: Param2) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), resourcename.into_param().abi(), resourceisimplicitstyle.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), ::core::mem::transmute(resourcekey), ::core::mem::transmute(resourcehandle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), ::core::mem::transmute(resourcekey)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualTreeService3 {
    type Vtable = IVisualTreeService3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(242861792, 34208, 19432, [180, 26, 101, 92, 241, 253, 25, 189]);
}
impl ::core::convert::From<IVisualTreeService3> for ::windows::runtime::IUnknown {
    fn from(value: IVisualTreeService3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualTreeService3> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualTreeService3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService2> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService2> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService2> for &IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService2> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeService> for &IVisualTreeService3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeService> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, propertyindex: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, child: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parent: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourcecontext: u64, resourcename: super::super::super::Foundation::PWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionaryhandle: u64, resourcename: super::super::super::Foundation::PWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dictionaryhandle: u64, resourcekey: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualTreeServiceCallback(pub ::windows::runtime::IUnknown);
impl IVisualTreeServiceCallback {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn OnVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, ParentChildRelation>, Param1: ::windows::runtime::IntoParam<'a, VisualElement>>(&self, relation: Param0, element: Param1, mutationtype: VisualMutationType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), relation.into_param().abi(), element.into_param().abi(), ::core::mem::transmute(mutationtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualTreeServiceCallback {
    type Vtable = IVisualTreeServiceCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860157233, 32996, 20460, [143, 59, 85, 63, 135, 180, 150, 110]);
}
impl ::core::convert::From<IVisualTreeServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: IVisualTreeServiceCallback) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualTreeServiceCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualTreeServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualTreeServiceCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IVisualTreeServiceCallback2(pub ::windows::runtime::IUnknown);
impl IVisualTreeServiceCallback2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn OnVisualTreeChange<'a, Param0: ::windows::runtime::IntoParam<'a, ParentChildRelation>, Param1: ::windows::runtime::IntoParam<'a, VisualElement>>(&self, relation: Param0, element: Param1, mutationtype: VisualMutationType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), relation.into_param().abi(), element.into_param().abi(), ::core::mem::transmute(mutationtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn OnElementStateChanged<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, element: u64, elementstate: VisualElementState, context: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(element), ::core::mem::transmute(elementstate), context.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualTreeServiceCallback2 {
    type Vtable = IVisualTreeServiceCallback2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134843784, 44663, 17303, [185, 72, 95, 162, 219, 10, 25, 234]);
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for ::windows::runtime::IUnknown {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for ::windows::runtime::IUnknown {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback> for IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeServiceCallback> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IVisualTreeServiceCallback> for &IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IVisualTreeServiceCallback> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: u64, elementstate: VisualElementState, context: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IXamlDiagnostics(pub ::windows::runtime::IUnknown);
impl IXamlDiagnostics {
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetDispatcher(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetUiLayer(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetApplication(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetIInspectableFromHandle(&self, instancehandle: u64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn GetHandleFromIInspectable<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, pinstance: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pinstance.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn HitTest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, rect: Param0, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rect.into_param().abi(), ::core::mem::transmute(pcount), ::core::mem::transmute(ppinstancehandles)).ok()
    }
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
    pub unsafe fn RegisterInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, pinstance: Param0) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pinstance.into_param().abi(), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
    pub unsafe fn GetInitializationData(&self) -> ::windows::runtime::Result<super::super::super::Foundation::BSTR> {
        let mut result__: <super::super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IXamlDiagnostics {
    type Vtable = IXamlDiagnostics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(415883958, 16195, 16662, [159, 43, 255, 147, 93, 119, 112, 210]);
}
impl ::core::convert::From<IXamlDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: IXamlDiagnostics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IXamlDiagnostics> for ::windows::runtime::IUnknown {
    fn from(value: &IXamlDiagnostics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IXamlDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IXamlDiagnostics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDiagnostics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdispatcher: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplayer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppapplication: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instancehandle: u64, ppinstance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinstance: ::windows::runtime::RawPtr, phandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinstance: ::windows::runtime::RawPtr, pinstancehandle: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinitializationdata: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeXamlDiagnostic<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(endpointname: Param0, pid: u32, wszdllxamldiagnostics: Param2, wsztapdllname: Param3, tapclsid: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        InitializeXamlDiagnostic(endpointname.into_param().abi(), ::core::mem::transmute(pid), wszdllxamldiagnostics.into_param().abi(), wsztapdllname.into_param().abi(), tapclsid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeXamlDiagnosticsEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param5: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(
    endpointname: Param0,
    pid: u32,
    wszdllxamldiagnostics: Param2,
    wsztapdllname: Param3,
    tapclsid: Param4,
    wszinitializationdata: Param5,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows::runtime::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        InitializeXamlDiagnosticsEx(endpointname.into_param().abi(), ::core::mem::transmute(pid), wszdllxamldiagnostics.into_param().abi(), wsztapdllname.into_param().abi(), tapclsid.into_param().abi(), wszinitializationdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: MetadataBit = MetadataBit(0i32);
    pub const IsValueHandle: MetadataBit = MetadataBit(1i32);
    pub const IsPropertyReadOnly: MetadataBit = MetadataBit(2i32);
    pub const IsValueCollection: MetadataBit = MetadataBit(4i32);
    pub const IsValueCollectionReadOnly: MetadataBit = MetadataBit(8i32);
    pub const IsValueBindingExpression: MetadataBit = MetadataBit(16i32);
    pub const IsValueNull: MetadataBit = MetadataBit(32i32);
    pub const IsValueHandleAndEvaluatedValue: MetadataBit = MetadataBit(64i32);
}
impl ::core::convert::From<i32> for MetadataBit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MetadataBit {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
impl ParentChildRelation {}
impl ::core::default::Default for ParentChildRelation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for ParentChildRelation {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ParentChildRelation").field("Parent", &self.Parent).field("Child", &self.Child).field("ChildIndex", &self.ChildIndex).finish()
    }
}
impl ::core::cmp::PartialEq for ParentChildRelation {
    fn eq(&self, other: &Self) -> bool {
        self.Parent == other.Parent && self.Child == other.Child && self.ChildIndex == other.ChildIndex
    }
}
impl ::core::cmp::Eq for ParentChildRelation {}
unsafe impl ::windows::runtime::Abi for ParentChildRelation {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl PropertyChainSource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PropertyChainSource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PropertyChainSource {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PropertyChainSource").field("Handle", &self.Handle).field("TargetType", &self.TargetType).field("Name", &self.Name).field("Source", &self.Source).field("SrcInfo", &self.SrcInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PropertyChainSource {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.TargetType == other.TargetType && self.Name == other.Name && self.Source == other.Source && self.SrcInfo == other.SrcInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PropertyChainSource {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PropertyChainSource {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: super::super::super::Foundation::BSTR,
    pub DeclaringType: super::super::super::Foundation::BSTR,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub ItemType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub Overridden: super::super::super::Foundation::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: super::super::super::Foundation::BSTR,
    pub PropertyChainIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PropertyChainValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PropertyChainValue {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PropertyChainValue")
            .field("Index", &self.Index)
            .field("Type", &self.Type)
            .field("DeclaringType", &self.DeclaringType)
            .field("ValueType", &self.ValueType)
            .field("ItemType", &self.ItemType)
            .field("Value", &self.Value)
            .field("Overridden", &self.Overridden)
            .field("MetadataBits", &self.MetadataBits)
            .field("PropertyName", &self.PropertyName)
            .field("PropertyChainIndex", &self.PropertyChainIndex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PropertyChainValue {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Type == other.Type && self.DeclaringType == other.DeclaringType && self.ValueType == other.ValueType && self.ItemType == other.ItemType && self.Value == other.Value && self.Overridden == other.Overridden && self.MetadataBits == other.MetadataBits && self.PropertyName == other.PropertyName && self.PropertyChainIndex == other.PropertyChainIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PropertyChainValue {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct RenderTargetBitmapOptions(pub i32);
pub const RenderTarget: RenderTargetBitmapOptions = RenderTargetBitmapOptions(0i32);
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = RenderTargetBitmapOptions(1i32);
impl ::core::convert::From<i32> for RenderTargetBitmapOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RenderTargetBitmapOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ResourceType(pub i32);
pub const ResourceTypeStatic: ResourceType = ResourceType(0i32);
pub const ResourceTypeTheme: ResourceType = ResourceType(1i32);
impl ::core::convert::From<i32> for ResourceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ResourceType {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
pub struct SourceInfo {
    pub FileName: super::super::super::Foundation::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SourceInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SourceInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SourceInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SourceInfo").field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("ColumnNumber", &self.ColumnNumber).field("CharPosition", &self.CharPosition).field("Hash", &self.Hash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.ColumnNumber == other.ColumnNumber && self.CharPosition == other.CharPosition && self.Hash == other.Hash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SourceInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SourceInfo {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[derive(:: core :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`, `Win32_Foundation`*"]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub NumChildren: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VisualElement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VisualElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VisualElement {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("VisualElement").field("Handle", &self.Handle).field("SrcInfo", &self.SrcInfo).field("Type", &self.Type).field("Name", &self.Name).field("NumChildren", &self.NumChildren).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VisualElement {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.SrcInfo == other.SrcInfo && self.Type == other.Type && self.Name == other.Name && self.NumChildren == other.NumChildren
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VisualElement {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VisualElement {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisualElementState(pub i32);
pub const ErrorResolved: VisualElementState = VisualElementState(0i32);
pub const ErrorResourceNotFound: VisualElementState = VisualElementState(1i32);
pub const ErrorInvalidResource: VisualElementState = VisualElementState(2i32);
impl ::core::convert::From<i32> for VisualElementState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualElementState {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_Xaml_Diagnostics`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisualMutationType(pub i32);
pub const Add: VisualMutationType = VisualMutationType(0i32);
pub const Remove: VisualMutationType = VisualMutationType(1i32);
impl ::core::convert::From<i32> for VisualMutationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualMutationType {
    type Abi = Self;
}