use macros::test;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TestStruct
{
	pub x: i8,
	pub y: i8,
	pub z: i8
}

#[test]
fn nothing(a: &str, b: i32, TestStruct{x: c, y: d, z: e}: TestStruct, (foo, bar, baz): (&str, u8, bool)) -> f64
{
	println!("a: {}, b: {}, c: {}, foo: {}, bar: {}, baz: {}", a, b, c, foo, bar, baz);
	1.1
}

trait TestTrait
{
	fn test_method(&self, i: i32) -> f64;
}

fn main()
{
	nothing("a", 1, TestStruct{x: 10, y: 12, z: 14}, ("ahoy", 200, true));
}
