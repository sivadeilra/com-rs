mod comoutptr;
mod comptr;
mod iclassfactory;
mod inproc;
mod iunknown;
mod runtime;

pub use comoutptr::ComOutPtr;
pub use comptr::ComPtr;
pub use iclassfactory::{
    IClassFactory, IClassFactoryVPtr, IClassFactoryVTable, IID_ICLASS_FACTORY,
};
pub use inproc::*;
pub use iunknown::{IUnknown, IUnknownVPtr, IUnknownVTable, IID_IUNKNOWN};
pub use runtime::Runtime;

use winapi::shared::{guiddef::IID, winerror::HRESULT};

pub fn failed(result: HRESULT) -> bool {
    result < 0
}

/// Structs implementing this trait must have the layout of a COM Interface Pointer.
/// For example, we assume safe conversion and usage of the struct as a `RawIUnknown`.
pub unsafe trait ComInterface {
    type VTable;
    const IID: IID;
}

// Export winapi for use by macros
#[doc(hidden)]
pub extern crate winapi as _winapi;

#[doc(hidden)]
pub use com_interface_attribute::*;

// this allows for the crate to refer to itself as `com` to keep macros consistent
// whether they are used by some other crate or internally
#[doc(hidden)]
extern crate self as com;
