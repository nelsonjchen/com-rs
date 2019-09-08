use proc_macro2::TokenStream as HelperTokenStream;
use quote::{quote, format_ident,};
use syn::{ItemStruct, Ident,};

// impl BritishShortHairCat {
//     fn allocate(init_struct: InitBritishShortHairCat) -> Box<BritishShortHairCat> {
//         {
//             ::std::io::_print(::std::fmt::Arguments::new_v1(
//                 &["Allocating new VTable for ", "\n"],
//                 &match (&"BritishShortHairCat",) {
//                     (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
//                 },
//             ));
//         };
//         let icat_vtable =
//             <dyn ICat as ::com::ProductionComInterface<BritishShortHairCat>>::vtable::<
//                 ::com::offset::Zero,
//             >();
//         let __icatvptr = Box::into_raw(Box::new(icat_vtable));
//         let idomesticanimal_vtable = <dyn IDomesticAnimal as ::com::ProductionComInterface<
//             BritishShortHairCat,
//         >>::vtable::<::com::offset::One>();
//         let __idomesticanimalvptr = Box::into_raw(Box::new(idomesticanimal_vtable));
//         let out = BritishShortHairCat {
//             __icatvptr,
//             __idomesticanimalvptr,
//             __refcnt: 0,
//             __init_struct: init_struct,
//         };
//         Box::new(out)
//     }
//     pub fn get_class_object() -> Box<BritishShortHairCatClassFactory> {
//         <BritishShortHairCatClassFactory>::new()
//     }
// }

pub fn generate(base_itf_idents: &[Ident], struct_item: &ItemStruct) -> HelperTokenStream {
    let init_ident = &struct_item.ident;
    let real_ident = macro_utils::get_real_ident(&struct_item.ident);

    // Allocate stuff
    let mut offset_count: usize = 0;
    let base_inits = base_itf_idents.iter().map(|base| {
        let vtable_var_ident = format_ident!("{}_vtable", base.to_string().to_lowercase());
        let vptr_field_ident = macro_utils::get_vptr_field_ident(&base);

        let out = quote!(
            let #vtable_var_ident = com::vtable!(#real_ident: #base, #offset_count);
            let #vptr_field_ident = Box::into_raw(Box::new(#vtable_var_ident));
        );

        offset_count += 1;
        out
    });
    let base_fields = base_itf_idents.iter().map(|base| {
        let vptr_field_ident = macro_utils::get_vptr_field_ident(base);
        quote!(#vptr_field_ident)
    });
    let ref_count_ident = macro_utils::get_ref_count_ident();
    let inner_init_field_ident = macro_utils::get_inner_init_field_ident();

    // GetClassObject stuff
    let class_factory_ident = macro_utils::get_class_factory_ident(&real_ident);

    quote!(
        impl #real_ident {
            fn allocate(init_struct: #init_ident) -> Box<#real_ident> {
                println!("Allocating new VTable for {}", stringify!(#real_ident));
                #(#base_inits)*
                let out = #real_ident {
                    #(#base_fields,)*
                    #ref_count_ident: 0,
                    #inner_init_field_ident: init_struct
                };
                Box::new(out)
            }

            pub fn get_class_object() -> Box<#class_factory_ident> {
                <#class_factory_ident>::new()
            }
        }
    )
}