use crate::CoClass;

use winapi::{
    shared::{
        guiddef::{REFIID,},
        winerror::{HRESULT, S_OK,},
    },
    ctypes::c_void,
};
use std::sync::atomic::{ AtomicU32, Ordering };

/// Wrapper struct that intercepts and handles IUnknown methods on the COM Object.
#[repr(C)]
pub struct ComBox<T: CoClass> {
    vtable_list: T::VPointerList,
    ref_count: AtomicU32,
    value: T,
}

impl<T: CoClass> ComBox<T> {
    pub fn new(value: T) -> ComBox<T> {
        ComBox {
            vtable_list: T::create_vtable_list(),
            ref_count: AtomicU32::new(0),
            value,
        }
    }

    // Delegates to the CoClass's query_interface implementation,
    // Passing along the vpointers.
    pub unsafe fn combox_query_interface(
        &mut self,
        riid: REFIID,
        out: *mut *mut c_void,
    ) -> HRESULT {
        let hr = T::query_interface(&self.vtable_list, riid, out);
        self.combox_add_ref();

        hr
    }

    /// Increments the reference count for the underlying COM Object.
    ///
    /// Returns the reference count after the increment.
    pub fn combox_add_ref(&mut self) -> u32 {
        let previous_value = self.ref_count.fetch_add(1, Ordering::Relaxed);
        (previous_value + 1)
    }

    /// Gets the reference count of the underlying COM Object.
    pub fn get_ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::Relaxed)
    }

    /// Decrements the reference count. Destroys the object if the count reaches
    /// zero.
    ///
    /// Returns the reference count after the release.
    pub fn combox_release(&mut self) -> u32 {
        // Ensure we're not releasing an interface that has no references.
        //
        // Note: If the interface has no references, it has already been
        // dropped. As a result we can't guarantee that it's ref_count stays
        // as zero as the memory could have been reallocated for something else.
        //
        // However this is still an effective check in the case where the client
        // attempts to release a com pointer twice and the memory hasn't been
        // reused.
        //
        // It might not be deterministic, but in the cases where it triggers
        // it's way better than the access violation error we'd otherwise get.
        if self.ref_count.load(Ordering::Relaxed) == 0 {
            panic!("Attempt to release pointer with no references.");
        }

        // Decrease the ref count and store a copy of it. We'll need a local
        // copy for a return value in case we end up dropping the ComBox
        // instance. after the drop referencing *this would be undeterministic.
        let previous_value = self.ref_count.fetch_sub(1, Ordering::Relaxed);
        let rc = previous_value - 1;

        // If that was the last reference we can drop self. Do this by giving
        // it back to a box and then dropping the box. This should reverse the
        // allocation we did by boxing the value in the first place.
        if rc == 0 {
            drop(self);
        }
        rc
    }
}

impl<T> std::ops::Deref for ComBox<T>
where
    T: CoClass,
{
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> std::ops::DerefMut for ComBox<T>
where
    T: CoClass,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
