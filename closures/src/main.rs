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

    // They prefrentialy cature by ref and then go lower as required
}

fn main() {
    outer_var();
}
