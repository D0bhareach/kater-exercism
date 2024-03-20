use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Planet)]
pub fn planet_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_macro(&ast)
}

const EARCH_YEAR_SECONDS: f64 = 31557600f64;
const VENUS_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 0.61518726f64;
const MERCURY_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 0.2408467f64;
const MARS_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 1.8808158f64;
const JUPITER_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 11.862615f64;
const SATURN_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 29.447498f64;
const URANUS_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 84.016846f64;
const NEPTUNE_COOFICIENT: f64 = EARCH_YEAR_SECONDS * 164.79132f64;

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let planet = &ast.ident;
    let pl_st = planet.to_string();
    let coof = match pl_st.as_str() {
        // orbital period cooficient
        "Venus" => VENUS_COOFICIENT,
        "Mercury" => MERCURY_COOFICIENT,
        "Mars" => MARS_COOFICIENT,
        "Jupiter" => JUPITER_COOFICIENT,
        "Saturn" => SATURN_COOFICIENT,
        "Uranus" => URANUS_COOFICIENT,
        "Neptune" => NEPTUNE_COOFICIENT,
        &_ => EARCH_YEAR_SECONDS,
    };
    let gen = quote! {
        impl Planet for #planet {
            fn years_during(d: &Duration) -> f64 {
                let s = d.seconds / #coof;
                (s * 100.0).round() / 100.0
                }
        }
    };
    gen.into()
}
