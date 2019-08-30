use com::{iunknown_gen_vtable, IUnknown, IUnknownVPtr, IUnknownVTable, IID_IUNKNOWN, CoClass, ComBox,};
use interface::{
    ianimal::{IAnimal, IAnimalVPtr, IAnimalVTable, IID_IANIMAL},
    ianimal_gen_vtable,
    icat::{ICat, ICatVPtr, ICatVTable, IID_ICAT},
    icat_gen_vtable,
    idomesticanimal::{
        IDomesticAnimal, IDomesticAnimalVPtr, IDomesticAnimalVTable, IID_IDOMESTIC_ANIMAL,
    },
    idomesticanimal_gen_vtable,
};

use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::{IsEqualGUID, IID},
        winerror::{E_NOINTERFACE, HRESULT, NOERROR},
    },
};

/// The implementation class
/// https://en.wikipedia.org/wiki/British_Shorthair
#[repr(C)]
pub struct BritishShortHairCat {
    food: u32,
}

impl IDomesticAnimal for BritishShortHairCat {
    fn train(&mut self) -> HRESULT {
        println!("Training...");
        NOERROR
    }
}

impl ICat for BritishShortHairCat {
    fn ignore_humans(&mut self) -> HRESULT {
        println!("Ignoring Humans...");
        NOERROR
    }
}

impl IAnimal for BritishShortHairCat {
    fn eat(&mut self) -> HRESULT {
        println!("Eating...");
        NOERROR
    }
}

pub struct BritishShortHairCatVPointerList {
    icat: ICatVPtr,
    idomesticanimal: IDomesticAnimalVPtr,
}

impl Drop for BritishShortHairCatVPointerList {
    fn drop(&mut self) {
        let _ = unsafe {
            Box::from_raw(self.icat as *mut ICatVTable);
            Box::from_raw(self.idomesticanimal as *mut IDomesticAnimalVTable);
        };
    }
}

impl CoClass for BritishShortHairCat {
    type VPointerList = BritishShortHairCatVPointerList;

    fn query_interface(vtbl_list: &Self::VPointerList, riid: *const IID, ppv: *mut *mut c_void) -> HRESULT {
        /* TODO: This should be the safe wrapper. You shouldn't need to write unsafe code here. */
        unsafe {
            let riid = &*riid;

            if IsEqualGUID(riid, &IID_IUNKNOWN)
                | IsEqualGUID(riid, &IID_ICAT)
                | IsEqualGUID(riid, &IID_IANIMAL)
            {
                *ppv = &vtbl_list.icat as *const _ as *mut c_void;
            } else if IsEqualGUID(riid, &IID_IDOMESTIC_ANIMAL) {
                *ppv = &vtbl_list.idomesticanimal as *const _ as *mut c_void;
            } else {
                println!("Returning NO INTERFACE.");
                *ppv = std::ptr::null_mut::<c_void>();
                return E_NOINTERFACE;
            }

            println!("Successful!.");
            NOERROR
        }
    }

    fn create_vtable_list() -> Self::VPointerList {
        let icat_vtable = icat_gen_vtable!(ComBox<BritishShortHairCat>, 0);
        let icat_vptr = Box::into_raw(Box::new(icat_vtable));
        let idomesticanimal_vtable = idomesticanimal_gen_vtable!(ComBox<BritishShortHairCat>, 1);
        let idomesticanimal_vptr = Box::into_raw(Box::new(idomesticanimal_vtable));

        BritishShortHairCatVPointerList {
            icat: icat_vptr,
            idomesticanimal: idomesticanimal_vptr,
        }
    }
}

impl BritishShortHairCat {
    pub(crate) fn new() -> BritishShortHairCat {
        BritishShortHairCat {
            food: 0,
        }
    }
}
