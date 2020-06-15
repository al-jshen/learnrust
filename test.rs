use std::cell::RefCell;

#[derive(Debug)]
struct Test {
    data: u8,
}

fn main() {
    let r = vec![RefCell::new(Test { data: 0 }), RefCell::new(Test { data: 1 })];
    {
        let mut test1 = r[0].borrow_mut();
        let mut test2 = r[1].borrow_mut();
        test1.data += 1;
        test2.data += 2;
        println!("{:?}", test1);
        println!("{:?}", test2);
    }
    println!("{:?}", &r);
}

