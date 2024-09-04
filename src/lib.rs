//! Rust attribute macro library that makes it easy to wrap functions in code that runs before and / or after a function executes.
//!
//! Repository: <https://github.com/ChandlerJayCalkins/function-wrapper>
//!
//! This function
//!
//! ```rs
//! #[wrap]
//! fn hello() -> bool
//! {
//! 	println!("Hello there!");
//! 	println!("This is some code.");
//! 	true
//! }
//! ```
//!
//! which is being wrapped by this attribute
//!
//! ```rs
//! use function_wrapper::WrappedFn;
//! extern crate proc_macro;
//! extern crate proc_macro2;
//! use syn::parse_macro_input;
//! use quote::quote;
//!
//! // Adds print statements before and after a function executes.
//! #[proc_macro_attribute]
//! pub fn wrap(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
//! {
//! 	// Parse input as a WrappedFn object from the function-wrapper crate.
//! 	let mut function = parse_macro_input!(item as WrappedFn);
//! 	// Put a print statement in the code that gets run before the function.
//! 	function.set_pre_code(quote!{ println!("Hi at the start :)"); });
//! 	// Put a print statement in the code that gets run after the function.
//! 	function.set_post_code(quote!{ println!("Hi at the end :)"); });
//! 	// Convert the function into a TokenStream and return it.
//! 	proc_macro2::TokenStream::from(function).into()
//! }
//! ```
//!
//! will turn into this after being compiled.
//!
//! ```rs
//! fn hello() -> bool
//! {
//! 	println!("Hi at the start :)");
//! 	let mut wrapper = ||
//! 	{
//! 		println!("Hello there!");
//! 		println!("This is some code.");
//! 		true
//! 	};
//! 	let result = wrapper();
//! 	println!("Hi at the end :)");
//! 	result
//! }
//! ```

use proc_macro2::{TokenStream, Span};
use syn::{ItemFn, Ident, /* Type, ReturnType */};
use syn::parse::{Parse, ParseStream};
use syn;
use quote::quote;
use core::iter::Extend;

/// String table of error messages
const ERROR_STRS: [&str; 1] =
[
	// Error message for when no tokens are given to parse in the `syn::parse()` method.
	"expected function"
];

// /// Contains the type variants that wrapped function can return.
// #[derive(Clone)]
// pub enum WrappedFnOutput
// {
// 	/// No return type was given. Usually represented as `()`.
// 	Default,
// 	/// All other explicitly written types. Contains a `syn::Type` as the internal value.
// 	Type(Type)
// }

/// Function that can have code inserted before and after the rest of the function executes.
/// Can be constructed with `syn::parse()` and other variations of parsing from the `syn` crate.
///
/// Example:
///
/// ```rs
/// let mut function = parse_macro_input!(token_stream as WrappedFn);
/// ```
///
/// Code that runs before the function can be set using the `set_pre_code()` method.
///
/// Example:
///
/// ```rs
/// function.set_pre_code(quote!{ println!("Hi at the start :)"); });
/// ```
///
/// Code that runs after the function can be set using the `set_post_code()` method.
///
/// Example:
///
/// ```rs
/// function.set_post_code(quote!{ println!("Hi at the end :)"); });
/// ```
#[derive(Clone)]
pub struct WrappedFn
{
	/// Contains code that gets run before the rest of the function.
	pub pre_code: Option<TokenStream>,
	/// `syn::ItemFn` that contains all of the data of the original function, including the code inside, the function signature, any attributes, etc.
	pub function: ItemFn,
	// /// The arguments to the function.
	// pub args: Vec<FnArgData>,
	// /// Return type.
	// pub output: WrappedFnOutput,
	/// Identifier token for the closure that wraps all of the original code from the wrapped function. It is `wrapper` by default.
	pub wrapper_ident: Ident,
	/// Identifier token for the variable that holds the return value of the wrapped function. It is `result` by default.
	pub result_ident: Ident,
	/// Contains code that gets run after the rest of the function.
	pub post_code: Option<TokenStream>
}

impl WrappedFn
{
	/// Sets the code that gets run before the rest of the function executes.
	pub fn set_pre_code(&mut self, pre_code: TokenStream)
	{
		self.pre_code = Some(pre_code);
	}

	/// Sets the code that gets run after the rest of the function executes.
	pub fn set_post_code(&mut self, post_code: TokenStream)
	{
		self.post_code = Some(post_code);
	}
}

/// Main way to construct a `WrappedFn`.
/// Can be constructed using `syn::parse_macro_input` like this:
///
/// ```rs
/// let mut function = parse_macro_input!(token_stream as WrappedFn);
/// ```
impl Parse for WrappedFn
{
	/// Constructs a WrappedFn from a `syn::ParseStream`.
	fn parse(input: ParseStream) -> syn::Result<Self>
	{
		// If no tokens were given to parse, throw an error
		if input.is_empty()
		{
			return Err(syn::Error::new(input.span(), ERROR_STRS[0]))
		}
		// Attempt to parse the input tokens as a function
		let function: ItemFn = input.parse()?;
		// Get the return type
		// let output = match function.sig.output.clone()
		// {
		// 	// If a return type was explicitly given, extract it
		// 	ReturnType::Type(_, o) => WrappedFnOutput::Type(*o),
		// 	// If no return type was given, use the default return type variant (usually represented as ())
		// 	ReturnType::Default => WrappedFnOutput::Default
		// };
		// Construct a WrappedFn to return
		let wrapped_function = Self
		{
			pre_code: None,
			function: function,
			// output: output,
			wrapper_ident: Ident::new("wrapper", Span::call_site()),
			result_ident: Ident::new("result", Span::call_site()),
			post_code: None
		};
		Ok(wrapped_function)
	}
}

/// Allows `WrappedFn`s to be converted into tokenstreams for easy use in procedural attribute macros.
impl From<WrappedFn> for TokenStream
{
	/// Converts a `WrappedFn` into a `proc_macro2::TokenStream`.
	fn from(function: WrappedFn) -> Self
	{
		// Determine whether the function has code that gets run before / after the rest of the function or not
		match (function.pre_code, function.post_code)
		{
			// If the function has some code to get run both before and after the function
			(Some(pre_code), Some(post_code)) =>
			{
				// Get the code code block from the function that was given
				let wrapped_code = &function.function.block;
				// Get the identifier token for the closure that wraps the function's original code
				let wrapper_ident = &function.wrapper_ident;
				// Get the identifier token for the variable that holds the result of running the function's original code
				let result_ident = &function.result_ident;
				// Wrap the code in a closure, get the result of running that closure, and turn all of it into a TokenStream
				let wrapper_code = quote!
				{
					let mut #wrapper_ident = || #wrapped_code ;
					let #result_ident = #wrapper_ident ();
				};
				// Get a TokenStream of the return line
				let return_line = quote!{ #result_ident };
				// Create a TokenStream where everything will get combined, starting with the code that gets run before the rest of the function
				let mut function_block = pre_code;
				// Add the wrapped code that came with the function
				function_block.extend(wrapper_code);
				// Add the code that runs after the rest of the function
				function_block.extend(post_code);
				// Add the line that returns the return value
				function_block.extend(return_line);
				// Wrap all of this code inside curly braces
				let function_block = quote!{ { #function_block } };
				// Get a new ItemFn object
				let mut function = function.function.clone();
				// Set the code inside the function to the new code
				function.block = syn::parse(function_block.into()).unwrap();
				// Convert the function back to a TokenStream and return it
				quote!(#function)
			},
			// If the function has some code to get run before the function but not after
			(Some(pre_code), None) =>
			{
				// Get the code code block from the function that was given
				let code = &function.function.block;
				// Turn it into a TokenStream
				let code = quote! { #code };
				// Create a TokenStream where everything will get combined, starting with the code that gets run before the rest of the function
				let mut function_block = TokenStream::new();
				// Add the code that gets run before the rest of the function
				function_block.extend(pre_code);
				// Add the code that came with the function
				function_block.extend(code);
				// Wrap all of this code inside curly braces
				let function_block = quote!{ { #function_block } };
				// Get a new ItemFn object
				let mut function = function.function.clone();
				// Set the code inside the function to the new code
				function.block = syn::parse(function_block.into()).unwrap();
				// Convert the function back to a TokenStream and return it
				quote!(#function)
			},
			// If the function has some code to get run after the function but not before
			(None, Some(post_code)) =>
			{
				// Get the code code block from the function that was given
				let wrapped_code = &function.function.block;
				// Get the identifier token for the closure that wraps the function's original code
				let wrapper_ident = &function.wrapper_ident;
				// Get the identifier token for the variable that holds the result of running the function's original code
				let result_ident = &function.result_ident;
				// Wrap the code in a closure, get the result of running that closure, and turn all of it into a TokenStream
				let wrapper_code = quote!
				{
					let mut #wrapper_ident = || #wrapped_code ;
					let #result_ident = #wrapper_ident ();
				};
				// Get a TokenStream of the return line
				let return_line = quote!{ #result_ident };
				// Create a TokenStream where everything will get combined, starting with the wrapped code that came with the function
				let mut function_block = wrapper_code;
				// Add the code that runs after the rest of the function
				function_block.extend(post_code);
				// Add the line that returns the return value
				function_block.extend(return_line);
				// Wrap all of this code inside curly braces
				let function_block = quote!{ { #function_block } };
				// Get a new ItemFn object
				let mut function = function.function.clone();
				// Set the code inside the function to the new code
				function.block = syn::parse(function_block.into()).unwrap();
				// Convert the function back to a TokenStream and return it
				quote!(#function)
			},
			// If the function has no code to insert before or after the function
			(None, None) =>
			{
				// Just return the function the way it is as a TokenStream
				let function = &function.function;
				quote!(#function)
			}
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn it_works()
	{
		assert(true);
	}
}
