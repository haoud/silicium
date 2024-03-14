use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn, ItemStatic};

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
    let new_type = syn::parse_quote!(crate::arch::percpu::PerCpu<#old_type>);
    let new_init = syn::parse_quote!(unsafe { crate::arch::percpu::PerCpu::new(#old_init) });

    var.ty = Box::new(new_type);
    var.expr = Box::new(new_init);
    var.attrs
        .push(syn::parse_quote!(#[link_section = ".percpu"]));

    TokenStream::from(quote::quote!(#var))
}

/// A macro to indicate that a function is only used during the initialization of the kernel.
/// This macro will this attribute are put in a separate .init section. When the kernel has
/// been initialized, this section will be discarded and the memory will be freed, allowing
/// the kernel to reduce its memory footprint.
///
/// # Safety
/// If an function with this attribute is called after the kernel has been initialized, the
/// behavior is undefined and will probably cause a kernel panic.
#[proc_macro_attribute]
pub fn init(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    input_fn
        .attrs
        .push(syn::parse_quote!(#[link_section = ".init"]));

    input_fn.sig.unsafety = Some(syn::parse_quote!(unsafe));

    TokenStream::from(quote::quote!(
        #input_fn
    ))
}

/// A macro to indicate that a function is atomic. Therefor, it will deny
/// the use of async/await, and interruptions / preemption will be disabled
/// to enforce atomicity. Furthermore, the function will increase an global
/// counter (only if the `enforce_atomic` feature is enabled) to detect if
/// the atomicity is violated.
#[proc_macro_attribute]
pub fn atomic(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Compile error if the function is async
    if input_fn.sig.asyncness.is_some() {
        return syn::Error::new_spanned(
            input_fn.sig.fn_token,
            "async functions are not allowed in atomic functions",
        )
        .to_compile_error()
        .into();
    }

    // Wrap the function body in a critical section by disabling
    // interruptions, preemption and sleep
    let body = input_fn.block;
    input_fn.block = Box::new(syn::parse_quote!({
        crate::arch::hal::irq::without(|| {
            crate::preempt::without(|| {
                crate::sys::sleep::without(|| {
                    #body
                })
            })
        })
    }));

    TokenStream::from(quote::quote!(
        #input_fn
    ))
}
/// A macro that indicate that a function may sleep. This macro will
/// check at the beginning of the function if the atomicity is violated
/// and will panic if it is the case.
#[proc_macro_attribute]
pub fn may_sleep(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    // Wrap the function body in a critical section by disabling
    // interruptions and preemption
    let body = input_fn.block;
    input_fn.block = Box::new(syn::parse_quote!({
        cfg_if::cfg_if! {
            if #[cfg(feature = "enforce_atomicity")] {
                if !crate::sys::sleep::enabled() {
                    panic!("#[may_sleep] function called in atomic context");
                }
            }
        }

        #body
    }));

    TokenStream::from(quote::quote!(
        #input_fn
    ))
}
