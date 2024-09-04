# function-wrapper
Rust attribute macro library that makes it easy to wrap functions in code that runs before and / or after a function executes.

This function

```rs
#[wrap]
fn hello() -> bool
{
	println!("Hello there!");
	println!("This is some code.");
	true
}
```

which is being wrapped by this attribute that adds a print statement before and after the function

```rs
use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro_attribute]
pub fn wrap_test(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	// Parse input as a WrappedFn object.
	let mut function = parse_macro_input!(item as WrappedFn);
	// Put a print statement in the code that gets run before the function.
	function.set_pre_code(quote!{ println!("Hi at the start :)"); });
	// Put a print statement in the code that gets run after the function.
	function.set_post_code(quote!{ println!("Hi at the end :)"); });
	// Convert the function into a TokenStream and return it.
	proc_macro2::TokenStream::from(function).into()
}
```

will turn into this after being compiled.

```rs
fn hello() -> bool
{
	println!("Hi at the start :)");
	let mut wrapper = ||
	{
		println!("Hello there!");
		println!("This is some code.");
		true
	};
	let result = wrapper();
	println!("Hi at the end :)");
	result
}
```
