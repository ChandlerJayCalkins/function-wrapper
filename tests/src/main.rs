use macros::wrap_test;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TestStruct
{
	pub x: i8,
	pub y: i8,
	pub z: i8
}

#[wrap_test]
fn nothing(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("-------------------- Arguments --------------------");
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	println!("---------------------------------------------------");
	1.1
}

trait TestTrait
{
	fn test_method(&self, i: i32) -> f64;
}

impl TestTrait for TestStruct
{
	#[wrap_test]
	fn test_method(&self, i: i32) -> f64
	{
		println!("-------------------- Arguments --------------------");
		println!("self: {:?}, i: {}", self, i);
		println!("---------------------------------------------------");
		let _ = i + 1;
		3.14159265
	}
}

fn main()
{
	nothing("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
	let ts = TestStruct{x: -3, y: -65, z: 127};
	ts.test_method(90);
}
