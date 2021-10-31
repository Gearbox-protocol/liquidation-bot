use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
pub struct Tst {
    n: u32,
}

impl Tst {
    pub fn new(num: u32) -> Self {
        Tst {n: num}
    }

    pub fn set(&mut self, n: u32) {
        self.n = n;
    }
}

impl Debug for Tst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NN: {}", self.n)
    }
}


fn main() {
    let value = Rc::new(RefCell::new(Tst::new(5)));

    let b = Rc::clone(&value);
    let c = Rc::clone(&value);

    let mut t = value.borrow_mut();
    t.set(23);

    drop(t);

    println!("a after = {:?}", value);
    println!("b after = {:?}", b);
    // println!("c after = {:?}", c);
}