use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn qty(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as LitStr);
    let expr = lit.value();
    match eng_unit_expr::evaluate(&expr) {
        Ok(q) => {
            let m = q.signature.m;
            let l = q.signature.l;
            let t = q.signature.t;
            let th = q.signature.th;
            let n = q.signature.n;
            let value_si = q.value_si;
            quote! {
                ::eng_core::units::typed::ExprInput::new(
                    #value_si,
                    ::eng_core::units::typed::DimensionSignature::new(#m, #l, #t, #th, #n),
                )
            }
            .into()
        }
        Err(e) => syn::Error::new(lit.span(), e.to_string())
            .to_compile_error()
            .into(),
    }
}
