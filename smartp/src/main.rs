use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierValue(String),
    True,
    False,
    Nil,
}

#[derive(Default, Debug, Clone)]
struct Foo {
    d: HashMap<String, LiteralValue>,
}

impl Foo {
    fn define(&mut self, k: String, v: LiteralValue) {
        // self.d.insert()
        self.d.insert(k, v);
    }
}

//struct Foo(Rc<RefCell<FnMut()>>);

fn test(env: Rc<RefCell<Foo>>) {
    let mut env = env.borrow_mut();
    // &d.d.insert("k".to_string(), "v".to_string());
    env.define("k".to_string(), LiteralValue::True);
    dbg!(env);
}

fn main() {
    let data = Rc::new(RefCell::new(Foo { d: HashMap::new() }));
    /*let a = Foo(Rc::new(RefCell::new(||{
        println!("Hello, world!");
    })));
    a.0.borrow_mut()();*/
    let _data1 = data.clone();
    let data2 = data.clone();
    let data3 = data.clone();
    test(data2);

    let mut d = data3.borrow_mut();
    // &d.d.insert("k".to_string(), "v".to_string());
    d.define("k".to_string(), LiteralValue::True);
    dbg!(d);
}
