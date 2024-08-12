use function_wrapper::WrappedFn;
extern crate proc_macro;
extern crate proc_macro2;
use syn::{parse_macro_input};
use quote::quote;

// Attribute that tests adding code before and after the rest of a function
#[proc_macro_attribute]
pub fn test(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream
{
	let mut idk = parse_macro_input!(item as WrappedFn);
	idk.set_pre_code(quote!
	{
		println!("hi at the start");
	});
	idk.set_post_code(quote!
	{
		println!("hi at the end");
	});
	let ts = proc_macro2::TokenStream::from(idk);
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
