use std::cell::RefCell;

#[derive(Debug)]
struct Test {
    data: u8,
}

impl Test {
    fn add(&mut self, other: &mut Test) {
        self.data += other.data;
        other.data += self.data;
    }

    fn mul(&mut self, other: &mut Test) {
        self.data *= other.data;
        other.data *= self.data;
    }
}

fn main() {
    let r = vec![RefCell::new(Test { data: 0 }), RefCell::new(Test { data: 1 })];
    {
        let mut test1 = r[0].borrow_mut();
        let mut test2 = r[1].borrow_mut();
        test1.add(&mut test2);
        println!("{:?}", test1);
        println!("{:?}", test2);
        test2.mul(&mut test1);
        println!("{:?}", test1);
        println!("{:?}", test2);
    }
    println!("{:?}", &r);
}

