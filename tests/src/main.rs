use macros::test;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TestStruct
{
	pub x: i8,
	pub y: i8,
	pub z: i8
}

#[test]
fn nothing(a: &str, b: i32, TestStruct{x: _, y: d, z: _}: TestStruct) -> f64
{
	println!("{}: {}", a, b);
	1.1
}

fn main()
{
	nothing("a", 1, TestStruct{x: 10, y: 12, z: 14});
}
