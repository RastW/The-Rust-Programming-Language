use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(List::Cons(Rc::clone(&value), Rc::new(List::Nil)));
    let b = List::Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
    let c = List::Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));

    // *解引用为 RefCell<i32>类型，.borrow_mut 获得可变引用
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // value的所有权 b、c也可以获取
}