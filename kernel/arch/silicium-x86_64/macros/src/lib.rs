use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStatic};

/// A macro that can be used on static variables to make them per-CPU. This macro will wrap
/// the variable in a [`PerCpu`] struct, which will allow each CPU to have its own copy of
/// the variable. For more information, see the [`PerCpu`] documentation.
///
/// # Example
/// ```rust
/// #[per_cpu]
/// pub static COUNTER: usize = 0;
///
/// fn main() {
///     COUNTER.set(42);
/// }
/// ```
#[proc_macro_attribute]
pub fn per_cpu(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut var = parse_macro_input!(item as ItemStatic);

    let old_type = var.ty.clone();
    let old_init = var.expr.clone();
    let new_type = syn::parse_quote!(arch::percpu::PerCpu<#old_type>);
    let new_init = syn::parse_quote!(unsafe { arch::percpu::PerCpu::new(#old_init) });

    var.ty = Box::new(new_type);
    var.expr = Box::new(new_init);
    var.attrs
        .push(syn::parse_quote!(#[link_section = ".percpu"]));

    TokenStream::from(quote::quote!(#var))
}
