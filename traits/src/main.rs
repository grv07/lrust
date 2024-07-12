// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;

    fn art(&self) {
        println!("su art");
    }

    fn eat_it(self) -> String
    where
        Self: Sized,
    {
        String::from("data")
    }
}

struct Range {
    start: usize,
    end: usize,
    // current: usize,
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            return None;
        }

        // if self.start < self.end {
        self.start = self.start + 1;

        return Some(self.start);
        // }

        // todo!()
    }
}

// impl IntoIterator for &Range {
//     type Item = usize;

//     type IntoIter = ;

//     fn into_iter(self) -> Self::IntoIter {
//         self.into_iter()
//     }
// }

struct Tweet {
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet summary {:?}", self.content.get(0..10))
    }
}

#[derive(Clone)]
struct Article {
    content: String,
}

impl Article {
    fn art() {
        println!("art");
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Tweet summary {:?}", self.content.get(0..10))
    }
}

fn print_it(data: &dyn Summary) -> String {
    // fn print_it<T: Summary + Clone>(data: &T) -> String {
    let p = data.summarize();
    // data
    // data.clone().eat_it();
    data.art();
    p
}

// We could define the Summary trait to have a summarize_author method whose implementation is required,
// and then define a summarize method that has a default implementation that calls the summarize_author method

// >>> We can’t implement external traits on external types.
//
// For example, we can’t implement the Display trait on Vec<T> within our aggregator crate,
// because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate.
// This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present.
// This rule ensures that other people’s code can’t break your code and vice versa.
// Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

// Using the Newtype Pattern to Implement External Traits on External Types
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// The impl Trait syntax is convenient and makes for more concise code in simple cases, while the fuller trait bound syntax can express more complexity in other cases.
// For example, we can have two parameters that implement Summary. Doing so with the impl Trait syntax looks like this:

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// If we want to force both parameters to have the same type, however, we must use a trait bound, like this:
// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specifying Multiple Trait Bounds with the + Syntax

// We can do so using the + syntax:

// pub fn notify(item: &(impl Summary + Display)) {
// The + syntax is also valid with trait bounds on generic types:
// pub fn notify<T: Summary + Display>(item: &T)

// Clearer Trait Bounds with where Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,

// Returning Types that Implement Traits
// fn ret_summary<'a>(switch: bool, art: &'a Article, tw: &'a Tweet) -> &'a dyn Summary {
fn ret_summary<'a>(switch: bool, art: &'a Article, tw: &'a Tweet) -> Box<dyn Summary> {
    if switch {
        Box::new(Article {
            content: "article there".to_string(),
        })
        // art
    } else {
        // tw
        Box::new(Tweet {
            content: "tweet there ... ".to_string(),
        })
    }
}

fn main_summary(tw: impl Summary) -> String {
    tw.summarize()
}

fn test(case: bool) {
    if case {
        let a = Article {
            content: "data".to_string(),
        };
        main_summary(a);
    } else {
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// impl block, Pair<T> only implements the cmp_display method if its inner type T implements
// the PartialOrd trait that enables comparison and the Display trait that enables printing.

// impl<T: Display + PartialOrd> Pair<T> {

// impl<T: Display> ToString for T {

/// Returns the n-th largest element in a slice
fn find_nth<T: Ord + Clone>(elems: &mut [T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}

trait Animal {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

impl Animal for Dog {}

struct Human;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your caption");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up");
    }
}

impl Human {
    fn fly(&self) {
        println!("flaping arms");
    }
}

mod inner {

    pub trait A {
        fn f(&self) -> usize {
            0
        }
    }

    pub trait B {
        fn f(&self) -> usize {
            1
        }
    }

    pub struct P;

    impl A for P {}

    impl B for P {}
}

fn main() {
    use inner::{B, P};
    // use inner::P;

    let t = if 4 > 3 {
        println!("exe");
        true
    } else {
        false
    };

    // println!("{:?}", t);

    println!("{}", P.f());

    dbg!(Dog::baby_name());
    dbg!(<Dog as Animal>::baby_name());

    // &impl Wizard is only allowed for function args and return type
    let h: &dyn Wizard = &Human;
    h.fly();

    let h = &Human;

    <Human as Pilot>::fly(h);

    Pilot::fly(&Human);
    // let t: &String;
    // {
    //     let s = String::from("hello");
    //     t = &s;
    // }

    // println!("Hello, world! {t}");

    let p = print_it(&Article {
        content: "hello there".to_string(),
    });

    dbg!(p);

    let r = Range { start: 4, end: 10 };

    // r.;
    // for r in r.iter() {
    //     println!("{r}");
    // }
    // for r in r.into_iter() {
    //     println!("{r}");
    // }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
