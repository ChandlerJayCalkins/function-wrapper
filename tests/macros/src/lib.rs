use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::{parse_macro_input, FnArg};
use quote::quote;

// Attribute that tests adding code before and after the rest of a function
#[proc_macro_attribute]
pub fn wrap_test(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	let mut function = parse_macro_input!(item as WrappedFn);
	let mut start_code = quote!
	{
		println!("########################################");
		println!("hi at the start");
	};
	for arg in &function.function.sig.inputs
	{
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
	function.set_pre_code(start_code);
	let result_ident = &function.result_ident;
	function.set_post_code(quote!
	{
		println!("return value: {:?}", #result_ident);
		println!("hi at the end");
		println!("########################################");
	});
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
