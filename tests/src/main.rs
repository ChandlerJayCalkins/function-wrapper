use macros::*;

fn main()
{
	test_all();
}

fn test_all()
{
	both_test();
	pre_test();
	post_test();
	none_test();
}

fn access_parameters_test()
{
	access_parameters_fn_test();
	access_parameters_mtd_test();
}

fn access_parameters_fn_test()
{
	access_parameters_fn("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}

fn access_parameters_mtd_test()
{
	let ts = TestStruct{x: -3, y: -65, z: 127};
	ts.access_parameters_mtd(90);
}

fn both_test()
{
	both_fn("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}

fn pre_test()
{
	pre_fn("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}

fn post_test()
{
	post_fn("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}

fn none_test()
{
	none_fn("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}

#[test_attr(both)]
fn both_fn(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("All");
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

#[test_attr(pre)]
fn pre_fn(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("Pre");
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

#[test_attr(post)]
fn post_fn(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("Post");
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

#[test_attr(none)]
fn none_fn(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("None");
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

#[access_parameters_attr]
fn access_parameters_fn(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("Access Parameters Function");
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TestStruct
{
	pub x: i8,
	pub y: i8,
	pub z: i8
}

trait TestTrait
{
	fn access_parameters_mtd(&self, i: i32) -> f64;
}

impl TestTrait for TestStruct
{
	#[access_parameters_attr]
	fn access_parameters_mtd(&self, i: i32) -> f64
	{
		println!("Access Parameters Method");
		println!("-------------------- Arguments --------------------");
		println!("self: {:?}, i: {}", self, i);
		println!("---------------------------------------------------");
		let _ = i + 1;
		3.14159265
	}
}
