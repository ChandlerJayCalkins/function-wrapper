use proc_macro2::{TokenStream, Span};
use syn::{ItemFn, Ident, PatIdent, PatType, Pat, FnArg, Type, ReturnType};
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

/// Argument to a wrapped function.
/// Contains the identifier (if one is given), the type (if it can be determined),
/// whether or not the argument is a reference, and whether or not it's mutable.
#[derive(Clone)]
pub struct WrappedFnArg<'a>
{
	/// Identifier for the argument (None if a `_` is used).
	pub ident: Option<&'a PatIdent>,
	/// Type of the argument (None if the type cannot be determined).
	pub ty: Option<&'a Type>,
	/// Whether or not the argument is a reference.
	pub reference: bool,
	/// Whether or not the argument is mutable.
	pub mutable: bool,
	/// Any subpatterns the argument might have.
	pub subpattern: Option<Pat>
}

/// Traverses an argument to a function recursively to find each identifier and the type of that identifier in it.
/// Identifiers that around found are placed in the args vec with their type in the form of a WrappedFnArg object.
fn collect_args<'a>(args: &mut Vec<WrappedFnArg<'a>>, pat: &'a Pat, ty: &'a Type, mut reference: bool, mut mutable: bool)
{
	(reference, mutable) = check_ref_mut(ty, &reference, &mutable);
	// Determine what kind of pattern the left side of the argument is
	match pat
	{
		// Const block
		Pat::Const(_) => (),
		// Identifier
		Pat::Ident(ident) => parse_ident(args, ident, ty, reference, mutable),
		// Literal value, like a number or a string literal (1, "hello", 5.33, etc.)
		Pat::Lit(_) => (),
		// Macro invocation
		Pat::Macro(_) => (),
		// Or statement with 2 matchable patterns
		Pat::Or(_) => (),
		// A pattern within parentheses
		Pat::Paren(_) => (),
		// A scope path like `lib::mod::Thing`
		Pat::Path(_) => (),
		// A range pattern like `1..10` or `3..=9`
		Pat::Range(_) => (),
		// A pattern in a reference (like `&var` or `&mut var`)
		Pat::Reference(_) => (),
		// Double dots to skip something(s) (like `Struct { x, y, .. }`)
		Pat::Rest(_) => (),
		// A pattern inside a slice
		Pat::Slice(_) => (),
		// A struct with more patterns inside
		Pat::Struct(_) => (),
		// A tuple containing patterns
		Pat::Tuple(_) => (),
		// A tuple struct with more patterns inside
		Pat::TupleStruct(_) => (),
		// A variable binding to a type (like `foo: f64` or `bar: &str`)
		Pat::Type(_) => (),
		// A raw TokenStream of a pattern
		Pat::Verbatim(_) => (),
		// An underscore that matches anything
		Pat::Wild(_) => (),
		// All other patterns not yet implemented
		_ => ()
	}
}

/// Returns whether or not a given type is a reference and / or mutable.
/// Returns the existing values if the type is not a reference.
fn check_ref_mut(ty: &Type, reference: &bool, mutable: &bool) -> (bool, bool)
{
	match ty
	{
		// If the type is a reference
		Type::Reference(r) =>
		{
			match r.mutability
			{
				// If the reference is mutable
				Some(_) => (true, true),
				// If the reference is not mutable
				None => (true, false)
			}
		},
		// If the type is not a reference
		_ => (*reference, *mutable)
	}
}

/// Parses an identifier argument in a wrapped function.
fn parse_ident<'a>(args: &mut Vec<WrappedFnArg<'a>>, ident: &'a PatIdent, ty: &'a Type, mut reference: bool, mut mutable: bool)
{
	// Last check to see if this argument is a reference
	reference = match ident.by_ref
	{
		Some(_) => true,
		None => reference
	};
	// Last check to see if this argument is mutable
	mutable = match ident.mutability
	{
		Some(_) => true,
		None => mutable
	};
	// Find out if this argument has a subpattern and collect it if it does
	let subpattern = match ident.subpat.clone()
	{
		Some(p) => Some(*p.1),
		None => None
	};
	// Construct a new WrappedFnArg object with the current identifier and type
	let arg = WrappedFnArg
	{
		ident: Some(ident),
		ty: Some(ty),
		reference: reference,
		mutable: mutable,
		subpattern: subpattern
	};
	// Add it to the list of arguments
	args.push(arg);
}

/// Contains the type variants that wrapped function can return.
#[derive(Clone)]
pub enum WrappedFnOutput
{
	/// No return type was given. Usually represented as `()`.
	Default,
	/// All other explicitly written types. Contains a `syn::Type` as the internal value.
	Type(Type)
}

/// Function that can have code inserted before and after the rest of the function executes.
/// Can be constructed with `syn::parse()` and other variations of parsing from the `syn`crate.
#[derive(Clone)]
pub struct WrappedFn
{
	/// `proc_macro2::TokenStream` that contains code that gets run before the rest of the function.
	pub pre_code: Option<TokenStream>,
	/// `syn::ItemFn` that contains all of the data of the original function, including the code inside, the function signature, any attributes, etc.
	pub function: ItemFn,
	// TODO: add args
	/// Return type.
	pub output: WrappedFnOutput,
	/// Identifier token for the closure that wraps all of the original code from the wrapped function.
	pub wrapper_ident: Ident,
	/// Identifier token for the variable that holds the return value of the wrapped function. Is almost always `result`.
	pub result_ident: Ident,
	/// `proc_macro2::TokenStream` that contains code that gets run after the rest of the function.
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
		// Get an iterator through all of the arguments in the function.
		let fn_args = function.sig.inputs.iter();
		let mut args: Vec<WrappedFnArg> = Vec::new();
		// Get the return type
		let output = match function.sig.output.clone()
		{
			// If a return type was explicitly given, extract it
			ReturnType::Type(_, o) => WrappedFnOutput::Type(*o),
			// If no return type was given, use the default return type variant (usually represented as ())
			ReturnType::Default => WrappedFnOutput::Default
		};
		// Construct a WrappedFn to return
		let wrapped_function = Self
		{
			pre_code: None,
			function: function,
			wrapper_ident: Ident::new("wrapper", Span::call_site()),
			result_ident: Ident::new("result", Span::call_site()),
			output: output,
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
				let return_line = quote!{ return result; };
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
				let return_line = quote!{ return result; };
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
