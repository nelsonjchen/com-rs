use crate::british_short_hair_cat::BritishShortHairCat;
use com::{
    IClassFactory, IClassFactoryVPtr, IClassFactoryVTable, IUnknown, IUnknownVPtr, IUnknownVTable,
    IID_ICLASSFACTORY, IID_IUNKNOWN, ComBox, CoClass,
};
use interface::icat_class::{ICatClassVPtr, ICatClassVTable, IID_ICAT_CLASS};

use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::{IsEqualGUID, IID, REFIID},
        minwindef::BOOL,
        winerror::{CLASS_E_NOAGGREGATION, E_NOINTERFACE, HRESULT, NOERROR, S_OK},
    },
};

#[repr(C)]
pub struct BritishShortHairCatClass {

}

impl IClassFactory for BritishShortHairCatClass {
    fn create_instance(
        &mut self,
        aggr: *mut IUnknownVPtr,
        riid: REFIID,
        ppv: *mut *mut c_void,
    ) -> HRESULT {
        println!("Creating instance...");
        if !aggr.is_null() {
            return CLASS_E_NOAGGREGATION;
        }

        let mut cat_combox = Box::new(ComBox::new(BritishShortHairCat::new()));
        cat_combox.combox_add_ref();
        let hr = unsafe { cat_combox.combox_query_interface(riid, ppv) };
        cat_combox.combox_release();
        Box::into_raw(cat_combox);

        hr
    }

    fn lock_server(&mut self, _increment: BOOL) -> HRESULT {
        println!("LockServer called");
        S_OK
    }
}

unsafe extern "stdcall" fn query_interface(
    this: *mut IUnknownVPtr,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    println!("Querying interface on CatClass...");
    let this = this as *mut ComBox<BritishShortHairCatClass>;
    (*this).combox_query_interface(riid, ppv)
}

unsafe extern "stdcall" fn add_ref(this: *mut IUnknownVPtr) -> u32 {
    println!("Adding ref...");
    let this = this as *mut ComBox<BritishShortHairCatClass>;
    (*this).combox_add_ref()
}

// TODO: This could potentially be null or pointing to some invalid memory
unsafe extern "stdcall" fn release(this: *mut IUnknownVPtr) -> u32 {
    println!("Releasing...");
    let this = this as *mut ComBox<BritishShortHairCatClass>;
    (*this).combox_release()
}

unsafe extern "stdcall" fn create_instance(
    this: *mut IClassFactoryVPtr,
    aggregate: *mut IUnknownVPtr,
    riid: *const IID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    let this = this as *mut BritishShortHairCatClass;
    (*this).create_instance(aggregate, riid, ppv)
}

unsafe extern "stdcall" fn lock_server(this: *mut IClassFactoryVPtr, increment: BOOL) -> HRESULT {
    let this = this as *mut BritishShortHairCatClass;
    (*this).lock_server(increment)
}

pub struct BritishShortHairCatClassVPointerList {
    icatclass: ICatClassVPtr,
}

impl Drop for BritishShortHairCatClassVPointerList {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.icatclass as *mut ICatClassVTable) };
    }
}

impl CoClass for BritishShortHairCatClass {
    type VPointerList = BritishShortHairCatClassVPointerList;

    fn query_interface(vtbl_list: &Self::VPointerList, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        /* TODO: This should be the safe wrapper. You shouldn't need to write unsafe code here. */
        unsafe {
            let riid = &*riid;
            if IsEqualGUID(riid, &IID_IUNKNOWN)
                || IsEqualGUID(riid, &IID_ICLASSFACTORY)
                || IsEqualGUID(riid, &IID_ICAT_CLASS)
            {
                *ppv = &vtbl_list.icatclass as *const _ as *mut c_void;
                NOERROR
            } else {
                E_NOINTERFACE
            }
        }
    }

    fn create_vtable_list() -> Self::VPointerList {
        println!("Allocating new Vtable for CatClass...");
        let iunknown = IUnknownVTable {
            QueryInterface: query_interface,
            Release: release,
            AddRef: add_ref,
        };
        let iclassfactory = IClassFactoryVTable {
            base: iunknown,
            CreateInstance: create_instance,
            LockServer: lock_server,
        };
        let icatclass = ICatClassVTable {
            base: iclassfactory,
        };
        let vptr = Box::into_raw(Box::new(icatclass));

        BritishShortHairCatClassVPointerList {
            icatclass: vptr,
        }
    }
}

impl BritishShortHairCatClass {
    pub(crate) fn new() -> BritishShortHairCatClass {
        BritishShortHairCatClass {}
    }
}
