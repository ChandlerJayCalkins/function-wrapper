# function-wrapper
Rust attribute macro library that makes it easy to wrap functions in code that runs before and / or after a function executes.

This function

```rust
#[wrap]
fn hello() -> bool
{
	println!("Hello there!");
	println!("This is some code.");
	true
}
```

which is being wrapped by this attribute

```rust
use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::parse_macro_input;
use quote::quote;

// Adds print statements before and after a function executes.
#[proc_macro_attribute]
pub fn wrap(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	// Parse input as a WrappedFn object from the function-wrapper crate.
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

```rust
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

If only pre-code is added, a wrapper closure and extra return expression won't be added since they are unecessary in this case.
If only post-code is added, the wrapper closure and return expression will still need to be added.
