mod comoutptr;
mod comptr;
mod iclassfactory;
mod inproc;
mod iunknown;
mod runtime;
mod combox;

pub use comoutptr::ComOutPtr;
pub use comptr::ComPtr;
pub use iclassfactory::{IClassFactory, IClassFactoryVPtr, IClassFactoryVTable, IID_ICLASSFACTORY};
pub use inproc::*;
pub use iunknown::{IUnknown, IUnknownVPtr, IUnknownVTable, IID_IUNKNOWN};
pub use runtime::Runtime;
pub use combox::ComBox;

use winapi::{
    shared::{
        guiddef::{REFIID, IID,},
        winerror::HRESULT
    },
    ctypes::c_void,
};

pub fn failed(result: HRESULT) -> bool {
    result < 0
}

/// Structs implementing this trait must have the layout of a COM Interface Pointer.
/// For example, we assume safe conversion and usage of the struct as a `RawIUnknown`.
pub unsafe trait ComInterface {
    type VTable;
    const IID: IID;
}

pub trait CoClass {
    type VPointerList;
    fn create_vtable_list() -> Self::VPointerList;
    fn query_interface(vtbl_list: &Self::VPointerList, riid: REFIID, ppv: *mut *mut c_void) -> HRESULT;
}

// Export winapi for use by macros
#[doc(hidden)]
pub extern crate winapi as _winapi;
