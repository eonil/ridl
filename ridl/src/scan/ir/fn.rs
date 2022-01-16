#![cfg(test)]

use proc_macro2::TokenStream;
use quote::quote;

#[test]
fn scan_trait_fns() {
    let a = quote! {
        trait Service1 {
            #[rest(GET,"/service1/func1")]
            fn func1(&self, input:Func1Input) -> Func1Output;
        }
    };
    let b = syn::parse2::<syn::File>(a).unwrap();
    assert_eq!(b.items.len(), 1);
    let c = &b.items[0];
    let d = if let syn::Item::Trait(d) = c { d } else { panic!() };
    scan_trait::scan_trait_fns(d).unwrap();
}

mod scan_trait {
    use crate::scan::Result;
    use crate::scan::err;
    pub fn scan_trait_fns(d:&syn::ItemTrait) -> Result<Vec<ScannedTraitFn>> {
        assert_eq!(d.items.len(), 1);
        let e = &d.items[0];
        let f = if let syn::TraitItem::Method(f) = e { f } else { panic!() };
        let h = f.sig.inputs;
    }
    struct ScannedTraitFn {
        trait_name: String,
        method_name: String,
        input_type_name: String,
        output_type_name: String,
    }
    fn scan_input_type_name(arg:&syn::FnArg) -> Result<String> {
        match arg {
            syn::FnArg::Receiver(_) => return err(arg, "`self` is not supported"),
            syn::FnArg::Typed(arg) => &*arg.ty
        }
        if let syn::FnArg::Typed(arg) = arg { arg } else { return err(arg, "") }
    }
    fn scan_type_ident_name(ty:&syn::Type) -> Result<String> {
        crate::scan
    }
}
