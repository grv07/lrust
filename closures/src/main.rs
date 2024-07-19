// Defination
// Closures are fun that can capture the enclose env.

// example
fn outer_var() {
    let outer_var = 1;

    // A regular fun can not capture the outer space environment
    // fn test(i: i32) -> i32 {
    //     i + outer_var
    // }

    // But this will captures the outer env
    let test = |i: i32| i + outer_var;

    // type infer
    // Will show error until you did not call it
    // once call it can infer the types and the never changes
    let test1 = |i| i + outer_var;

    println!("{}", test(68));
    println!("{}", test1(68));

    // It can alredy infer the type now can't change
    // println!("{}", test1('r'));
}

fn capturing_type() {
    // Closures can capture var smartly by
    // 1. reference &T
    // 2. &mut T
    // 3. T

    // They prefrentialy cature by ref and then go lower as required.
    let mut list = vec![1, 2, 3, 4];

    // ------------ CASE 1: Take referance

    // TODO: If you use LSP you can see the type of x is a x: impl Fn
    let x = || println!("List of ids {:?}", list);
    // closure auto detect that only ref is enough for computation of this closure so it uses Fn or pass by ref implementation
    println!("Before calling closure list: {:?}", list); // you can call this bcs it closure take list as ref
    x();

    // TODO: If you use LSP you can see the type of x is a x: impl FnMut
    let mut x = || {
        println!("List of ids {:?}", list);
        list.push(69);
    };
    // closure auto detect that only ref is enough for computation of this closure so it uses Fn or pass by ref implementation
    // TODO: Uncomment below line
    // println!("Before calling closure list: {:?}", list); // you can call this bcs it closure take list as ref
    x();

    // -----------  CASE 2: Take mut referance

    // TODO: If you use LSP you can see the type of x is a x: impl FnMut
    // NOTICE THE MUT
    let mut x = || {
        println!("List of ids {:?}", list);
        list.push(69);
    };
    // closure auto detect that mut ref is enough for computation of this closure so it uses FnMut or pass by mut ref implementation
    // TODO: Uncomment below line
    // println!("Before calling closure list: {:?}", list); // you can call this bcs it closure take list as mut ref
    x();

    // -----------  CASE 3: Take pass by value

    // TODO: If you use LSP you can see the type of x is a x: impl FnOnce(u32)
    let x = |i: u32| {
        println!("List of ids {:?}", list);
        drop(list);
        // list.push(69);
    };
    // closure auto detect that move value for computation of this closure so it uses FnOnce or pass by value implementation
    // TODO: Uncomment below line
    // println!("Before calling closure list: {:?}", list); // you can call this bcs it closure take list as value
    x(69);

    // TODO: Uncomment below line to lnow more
    // println!("can't use list now since it is already moves {:?}", list);
}

fn as_input_parameters() {
    let mut list = vec![1, 2, 3];
    // On a variable-by-variable basis, the compiler will capture variables in the least restrictive manner possible.

    // For instance, consider a parameter annotated as FnOnce.
    // This specifies that the closure may capture by &T, &mut T, or T, but the compiler will ultimately choose based on how the captured variables are used in the closure.

    // FnOnce can take Fn and FnMut types of closures
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    // Fn
    apply(|| println!("apply"));

    // FnMut
    apply(|| {
        println!("apply {:?}", list);
        list.push(69);
    });

    // FnOnce
    apply(|| {
        println!("apply");
        drop(list);
    });

    let mut list = vec![1, 2, 3];

    fn apply_l1<F>(mut f: F)
    where
        F: FnMut(),
    {
        f();
    }

    // pass Fn for FnMut
    apply_l1(|| println!("apply"));

    // pass FnMut for FnMut
    apply_l1(|| {
        println!("apply {:?}", list);
        list.push(69);
    });

    // TODO: Uncomment below code and you will get to know that
    // you can upcast list FnOnce van take FnOnce, FnMut or Fn.
    // FnMut can take FnMut and Fn only
    // Fn can take Fn only

    // pass FnOnce for FnMut
    // apply_l1(|| {
    //     println!("apply");
    //     drop(list);
    // });
}

fn main() {
    outer_var();
    capturing_type();
    as_input_parameters();
}
