# COM

[![Build Status](https://dev.azure.com/microsoft-rust/com-rs/_apis/build/status/microsoft.com-rs?branchName=master)](https://dev.azure.com/microsoft-rust/com-rs/_build/latest?definitionId=1&branchName=master)

A one stop shop for all things related to [COM](https://docs.microsoft.com/en-us/windows/win32/com/component-object-model--com--portal) programming in Rust.

This library exposes various macros to the user for both producing and consuming COM components in an idiomatic manner.

# Frequently asked questions

**Is there IDL support?**

As a foundation, we are attempting to create a library that doesn't necessarily rely on having an IDL file. However, it is in the pipeline for future improvements. We will have a command-line tool that will parse the IDL into the required macros.

**How do I use custom data types?**

**Is there out-of-process COM support?**

Currently, we only support production of in-process COM components. Also, production of a COM component can only be in the DLL format. There will be plans to enable out-of-process COM production as well as producing in the .EXE format.

# Usage

## Defining a COM interface

To both consume or produce a COM component through an interface, you will first need to generate the Rust representation of said interface. The `com_interface` macro is the main tool for automatically generating this Rust representation.

```rust
#[com_interface(00000000-0000-0000-C000-000000000046)]
pub trait IUnknown {
    fn query_interface(
        &mut self,
        riid: winapi::shared::guiddef::REFIID,
        ppv: *mut *mut winapi::ctypes::c_void
    ) -> winapi::shared::winerror::HRESULT;
    fn add_ref(&mut self) -> u32;
    fn release(&mut self) -> u32;
}

#[com_interface(EFF8970E-C50F-45E0-9284-291CE5A6F771)]
pub trait IAnimal: IUnknown {
    fn eat(&mut self) -> HRESULT;
}

```

Short explanation: This generates the VTable layout for IUnknown and implements the trait on ComPtr so that it dereferences the correct function pointer entry within the VTable.

## Consuming a COM component

Interaction with COM components are always through an Interface Pointer (a pointer to a pointer to a VTable). We represent such an Interface Pointer with the `ComPtr` struct, which helps manage the lifetime of the COM component through IUnknown methods.

```rust
// Initialises the COM library
let runtime = match Runtime::new() {
    Ok(runtime) => runtime,
    Err(hr) => {
        println!("Failed to initialize COM Library: {}", hr);
        return;
    }
};

// Get a COM instance's interface pointer, by specifying
// - The CLSID of the COM component
// - The interface of the COM component that you want
// runtime.create_instance returns a ComPtr<dyn IAnimal> in this case.
let mut cat = match runtime.create_instance::<dyn IAnimal>(&CLSID_CAT_CLASS) {
    Ok(cat) => cat,
    Err(e) => {
        println!("Failed to get a cat, {:x}", e);
        return;
    }
};

// All IAnimal methods will be defined on ComPtr<T: IAnimal>
cat.eat();
```

## Producing a COM component

Producing a COM component is relatively complicated compared to consumption, due to the many features available that we must support. Here, we will walk you through producing one of our examples, the `BritishShortHairCat`.

1. Define an Init struct containing all the user fields you want.
- Apply the `#[derive(CoClass)]` macro to wrap the Init struct in a COM-compatible struct. The name of the Init struct **MUST** start with "Init".
- You can then use the helper attribute `#[com_implements(...)]` to indicate inheritance of any COM interfaces. The order of interfaces declared is important, as the generated vpointers are going to be in that order.

```rust
#[repr(C)]
#[derive(CoClass)]
#[com_implements(ICat, IDomesticAnimal)]
pub struct InitBritishShortHairCat {
    num_owners: u32,
}
```

2. Implement the necessary traits on the actual COM struct (in this case, `BritishShortHairCat`).

```rust
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
```

3. You will have to define a constructor with the signature. This provides us with a standard constructor to your COM component. Within this constructor, you need to
```rust
fn new() -> Box<name_of_com_struct>
```
- Initialise all user fields
- Call the provided name_of_com_struct::allocate() function, which has the signature
```rust
fn allocate(init_name_of_com_struct) -> Box<name_of_com_struct>
```

```rust
impl BritishShortHairCat {
    pub(crate) fn new() -> Box<BritishShortHairCat> {
        let init = InitBritishShortHairCat { num_owners: 20 };
        BritishShortHairCat::allocate(init)
    }
}
```

# Advanced COM

## Aggregation

COM allows you to aggregate other COM objects. This means exposing their interfaces as your own, allowing code reuse.

If you plan to use aggregation, then we assume you are somewhat familiar with the inner workings of COM. This explanation assumes the same.

We will walk you through producing a `WindowsFileManager`, which aggregates another COM object, the `LocalFileManager`.

1. Define an **AGGREGABLE** com class. Here we use the `#[derive(AggrCoClass)]` macro instead of the `CoClass` one.

```rust
#[repr(C)]
#[derive(AggrCoClass)]
#[com_implements(ILocalFileManager)]
pub struct InitLocalFileManager {
    user_field: u32,
}

impl ILocalFileManager for LocalFileManager {
    fn delete_local(&mut self) -> HRESULT {
        println!("Deleting Locally...");
        NOERROR
    }
}

impl LocalFileManager {
    pub(crate) fn new() -> Box<LocalFileManager> {
        let init = InitLocalFileManager { user_field: 2 };
        LocalFileManager::allocate(init)
    }
}
```

2. Define the class that will aggregate `LocalFileManager`. This can be aggregable or not. The constructor is left out here to explain it separately.
- You are responsible for instantiating your aggregates (reasoning explained later).
- In order for us to generate the correct QueryInterface implementation, you need to inform us the field storing the aggregate's IUnknown, as well as the base interfaces exposed by that aggregate. To do this, you mark the field using the `#[aggr(...)]` helper attribute.

```rust
#[derive(CoClass)]
#[com_implements(IFileManager)]
#[repr(C)]
pub struct InitWindowsFileManager {
    #[aggr(ILocalFileManager)]
    lfm_iunknown: *mut IUnknownVPtr,
}

impl Drop for InitWindowsFileManager {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping init struct..");
            let mut lfm_iunknown: ComPtr<dyn IUnknown> =
                ComPtr::new(self.lfm_iunknown as *mut c_void);
            lfm_iunknown.release();
            forget(lfm_iunknown);
        };
    }
}

impl IFileManager for WindowsFileManager {
    fn delete_all(&mut self) -> HRESULT {
        println!("Deleting all by delegating to Local and Remote File Managers...");
        NOERROR
    }
}
```

3. Define the class constructor. Here, we chose to instantiate the aggregate in the constructor. You could choose to instantiate it whenever you want. As part of aggregation rules, you need to pass `WindowsFileManager's` IUnknown vpointer to the aggregated object. Since we hide this implementation detail from users, we expose them through a set of hidden functions.

```rust
impl WindowsFileManager {
    pub(crate) fn new() -> Box<WindowsFileManager> {
        let init = InitWindowsFileManager {
            lfm_iunknown: std::ptr::null_mut::<IUnknownVPtr>(),
        };

        let mut wfm = WindowsFileManager::allocate(init);

        // Instantiate object to aggregate
        // TODO: Should change to use safe ComPtr methods instead.
        let mut unknown_file_manager = std::ptr::null_mut::<c_void>();
        let hr = unsafe {
            CoCreateInstance(
                &CLSID_LOCAL_FILE_MANAGER_CLASS as REFCLSID,
                &*wfm as *const _ as winapi::um::unknwnbase::LPUNKNOWN,
                CLSCTX_INPROC_SERVER,
                &IID_IUNKNOWN as REFIID,
                &mut unknown_file_manager as *mut LPVOID,
            )
        };
        if failed(hr) {
            println!("Failed to instantiate aggregate! Error: {:x}", hr as u32);
            panic!();
        }

        wfm.lfm_iunknown = unknown_file_manager as *mut IUnknownVPtr;

        wfm
    }
}

```