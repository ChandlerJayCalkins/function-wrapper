use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::{parse_macro_input, FnArg};
use quote::quote;

/// Adds print statements before and after a function executes.
#[proc_macro_attribute]
pub fn example_wrapper(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
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

/// Attribute that tests adding code before and after the rest of a function and debug prints all parameters.
#[proc_macro_attribute]
pub fn access_parameters_attr(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	// Parses the input as a WrappedFn so code can be added before and / or after the rest of the function executes.
	let mut function = parse_macro_input!(item as WrappedFn);
	// Get a proc_macro2::TokenStream of the code that runs before the function.
	let mut start_code = quote!
	{
		println!("########################################");
		println!("hi at the start");
	};
	// Add a print statement for each argument to the function.
	for arg in &function.function.sig.inputs
	{
		// Determine whether the argument is a receiver (self arg) or a normal arg.
		match arg
		{
			FnArg::Receiver(receiver) => start_code = quote!{ #start_code println!("{:?}", #receiver); },
			FnArg::Typed(pat_type) =>
			{
				let pat = pat_type.pat.clone();
				start_code = quote!{ #start_code println!("{:?}", #pat); };
			}
		}
	}
	// Apply the pre_code to the wrapped function.
	function.set_pre_code(start_code);
	// Get the identifier of the variable that holds the return value of the function.
	let result_ident = &function.result_ident;
	// Set the code that runs after the function to be a print statement of the return value and some other print statements.
	function.set_post_code(quote!
	{
		println!("return value: {:?}", #result_ident);
		println!("hi at the end");
		println!("########################################");
	});
	// Convert the function into a proc_macro2::TokenStream
	let ts = proc_macro2::TokenStream::from(function);
	println!("{}", ts.clone());
	// Return the TokenStream of the function
	ts.into()
}

/// Adds print statements before and / or after a function executes, or adds nothing.
#[proc_macro_attribute]
pub fn test_attr(parms: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	// Parse input as a WrappedFn object from the function-wrapper crate.
	let mut function = parse_macro_input!(item as WrappedFn);
	match parms.to_string().to_lowercase().as_str()
	{
		"both" =>
		{
			// Put a print statement in the code that gets run before the function.
			function.set_pre_code(quote!{ println!("Hi at the start :)"); });
			// Put a print statement in the code that gets run after the function.
			function.set_post_code(quote!{ println!("Hi at the end :)"); });
		},
		"pre" =>
		{
			// Put a print statement in the code that gets run before the function.
			function.set_pre_code(quote!{ println!("Hi at the start :)"); });
		},
		"post" =>
		{
			// Put a print statement in the code that gets run after the function.
			function.set_post_code(quote!{ println!("Hi at the end :)"); });
		},
		"" | "none" => (),
		_ => panic!("Invalid attribute parameter.")
	}
	// Convert the function into a TokenStream and return it.
	let ts = proc_macro2::TokenStream::from(function);
	println!("{}", ts.clone());
	ts.into()
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
