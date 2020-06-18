struct Test {
    x: i32,
}

unsafe trait Foo {
    fn get_x(&self) -> i32;
}

unsafe impl Foo for Test {
    fn get_x(&self) -> i32 {
        self.x
    }
}

fn main() {
    let test = Test { x: 5 };
    println!("{}", test.get_x());
}
