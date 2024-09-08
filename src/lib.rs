//! Rust attribute macro library that makes it easy to wrap functions in code that runs before and / or after a function executes.
//!
//! Repository: <https://github.com/ChandlerJayCalkins/function-wrapper>
//!
//! This function
//!
//! ```rust
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
//! ```rust
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
//! ```rust
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
//!
//! If only pre-code is added, a wrapper closure and extra return expression won't be added since they are unecessary in this case.
//! If only post-code is added, the wrapper closure and return expression will still be added out of necessity.
//!

use proc_macro2::{TokenStream, Span};
use syn::{ItemFn, Block, Ident, /* Type, ReturnType */};
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
/// ```rust
/// let mut function = parse_macro_input!(token_stream as WrappedFn);
/// ```
///
/// Code that runs before the function can be set using the `set_pre_code()` method.
///
/// Example:
///
/// ```rust
/// function.set_pre_code(quote!{ println!("Hi at the start :)"); });
/// ```
///
/// Code that runs after the function can be set using the `set_post_code()` method.
///
/// Example:
///
/// ```rust
/// function.set_post_code(quote!{ println!("Hi at the end :)"); });
/// ```
#[derive(Clone, Debug)]
pub struct WrappedFn
{
	/// `syn::ItemFn` that contains all of the data of the original function, including the code inside, the function signature, any attributes, etc.
	pub function: ItemFn,
	/// Contains code that gets run before the rest of the function.
	pub pre_code: Option<TokenStream>,
	/// Contains code that gets run after the rest of the function.
	pub post_code: Option<TokenStream>,
	// /// The arguments to the function.
	// pub args: Vec<FnArgData>,
	// /// Return type.
	// pub output: WrappedFnOutput,
	/// Identifier token for the closure that wraps all of the original code from the wrapped function. `wrapper` by default.
	pub wrapper_ident: Ident,
	/// Identifier token for the variable that holds the return value of the wrapped function. `result` by default.
	pub result_ident: Ident
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

	/// Removes any code that was going to be added before the rest of the function.
	pub fn remove_pre_code(&mut self)
	{
		self.pre_code = None;
	}

	/// Removes any code that was going to be added after the rest of the function.
	pub fn remove_post_code(&mut self)
	{
		self.post_code = None;
	}

	/// Changes the identifier for the closure that wraps the code of the original function (`wrapper` by default).
	pub fn set_wrapper_ident(&mut self, ident: &str)
	{
		self.wrapper_ident = Ident::new(ident, Span::call_site());
	}

	/// Changes the identifier for the variable that holds the value that the function returns (`result` by default).
	pub fn set_result_ident(&mut self, ident: &str)
	{
		self.result_ident = Ident::new(ident, Span::call_site());
	}

	/// Inserts the unwrapped original code from a function into a function block.
	///
	/// Inputs:
	/// 
	/// `function_block`: The block of code that goes inside the function where the original code is re-added.
	///
	/// `og_code`: The original code from the function.
	fn add_unwrapped_code(function_block: &mut TokenStream, og_code: &Block)
	{
		// Convert the function's old code block into a TokenStream and add it after the pre code
		function_block.extend(quote!{ #og_code });
	}

	/// Wraps the original code of a function in a closure and inserts code after it inside a function block.
	///
	/// Inputs:
	///
	/// `og_code`: The original code of the function that comes before the post code.
	///
	/// `wrapper_ident`: Identifier token for the closure that wraps all of the original code from the wrapped function.
	///
	/// `result_ident`: Identifier token for the variable that holds the return value of the wrapped function.
	///
	/// `function_block`: The block of code that goes inside the function where the wrapper code and post code is added.
	///
	/// `post_code`: The code to be inserted that runs at the end of the function.
	fn add_wrapped_post_code(og_code: &Block, wrapper_ident: &Ident, result_ident: &Ident, function_block: &mut TokenStream, post_code: TokenStream)
	{
		// Wrap the code in a closure, get the result of running that closure, and turn all of it into a TokenStream
		let wrapper_code = quote!
		{
			let mut #wrapper_ident = || #og_code ;
			let #result_ident = #wrapper_ident ();
		};
		// Get a TokenStream of the return line
		let return_line = quote!{ #result_ident };
		// Add the wrapped code that came with the function
		function_block.extend(wrapper_code);
		// Add the code that runs after the rest of the function
		function_block.extend(post_code);
		// Add the line that returns the return value
		function_block.extend(return_line);
	}

	/// Gets a `proc_macro2::TokenStream` of a function that just had code inserted into it.
	///
	/// Inputs:
	///
	/// `function`: The original function that is being wrapped.
	///
	/// `function_block`: The new block of code that is replacing the old one inside the function that is being wrapped.
	///
	/// Outputs: A `proc_macro2::TokenStream` of the newly wrapped function.
	fn get_function(mut function: ItemFn, function_block: &TokenStream) -> TokenStream
	{
		// Wrap all of this code inside curly braces
		let function_block = quote!{ { #function_block } };
		// Set the code inside the function to the new code
		function.block = syn::parse(function_block.into()).unwrap();
		// Convert the function back to a TokenStream and return it
		quote!(#function)
	}
}

/// Main way to construct a `WrappedFn`.
/// Can be constructed using `syn::parse_macro_input` like this:
///
/// ```rust
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
			function: function,
			pre_code: None,
			post_code: None,
			// output: output,
			wrapper_ident: Ident::new("wrapper", Span::call_site()),
			result_ident: Ident::new("result", Span::call_site())
		};
		Ok(wrapped_function)
	}
}

/// Allows `WrappedFn`s to be converted into `proc_macro2::TokenStream`s for easy use in procedural attribute macros.
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
				// Create a new block of code that will replace the old one in the function and start it with the pre code
				let mut function_block = pre_code;
				// Wrap and add the original function code and add the post code to the new function block
				WrappedFn::add_wrapped_post_code(&function.function.block, &function.wrapper_ident, &function.result_ident, &mut function_block, post_code);
				// Replaces the function's code block with the new one and converts the entire function into a TokenStream to return it
				WrappedFn::get_function(function.function, &function_block)
			},
			// If the function has some code to get run before the function but not after
			(Some(pre_code), None) =>
			{
				// Create a new block of code that will replace the old one in the function and start it with the pre code
				let mut function_block = pre_code;
				// Add the function's original code after the pre code
				WrappedFn::add_unwrapped_code(&mut function_block, &function.function.block);
				// Replaces the function's code block with the new one and converts the entire function into a TokenStream to return it
				WrappedFn::get_function(function.function, &function_block)
			},
			// If the function has some code to get run after the function but not before
			(None, Some(post_code)) =>
			{
				// Create a new block of code that will replace the old one in the function
				let mut function_block = TokenStream::new();
				// Wrap and add the original function code and add the post code to the new function block
				WrappedFn::add_wrapped_post_code(&function.function.block, &function.wrapper_ident, &function.result_ident, &mut function_block, post_code);
				// Replaces the function's code block with the new one and converts the entire function into a TokenStream to return it
				WrappedFn::get_function(function.function, &function_block)
			},
			// If the function has no code to insert anywhere
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
