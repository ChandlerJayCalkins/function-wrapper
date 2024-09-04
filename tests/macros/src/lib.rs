use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::{parse_macro_input, FnArg};
use quote::quote;

// Attribute that tests adding code before and after the rest of a function.
#[proc_macro_attribute]
pub fn wrap_test(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
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
