use std::{sync::Arc, thread};

// Chapter 1

// Threads in rust
// Every program starts with exactly one thread: the main thread. This thread will execute your main function and can be used to spawn more threads if necessary.

fn f() {
    println!("Hello this is thread 1");

    let t_id = thread::current().id();

    println!("T Id: {t_id:?}");
}

fn f1(t: &[i32]) {
    println!("Hello this is thread 1");

    let t_id = thread::current().id();

    println!("T Id: {t_id:?}");

    println!("Vec is {:?}", t);
}

// There are several ways to create something that’s not owned by a single thread.
// The simplest one is a static value, which is "owned" by the entire program, instead of an individual thread.
// In the following example, both threads can access X, but neither of them owns it:

// shared reference
fn shared_by_static_ref() {
    static VALUE: [i32; 4] = [1, 2, 3, 4];

    let tj1 = thread::spawn(|| {
        println!("shared_by_static_ref Value is {:?}", VALUE);
    });

    let tj2 = thread::spawn(|| {
        println!("shared_by_static_ref Value is {:?}", VALUE);
    });

    let _ = tj1.join();
    let _ = tj2.join();
}

// The downside of leaking a Box is that we’re leaking memory.
fn shared_by_leaking() {
    // Note how the 'static lifetime doesn’t mean that the value lived since the start of the program, but only that it lives to the end of the program.
    //The past is simply not relevant.
    let value: &'static [i8; 4] = Box::leak(Box::new([1, 2, 3, 4]));

    // References are Copy, meaning that when you "move" them, the original still exists, just like with an integer or boolean.
    let tj1 = thread::spawn(move || {
        println!("shared_by_leaking Value is {:?}", value);
    });

    let tj2 = thread::spawn(move || {
        println!("shared_by_leaking Value is {:?}", value);
    });

    let _ = tj1.join();
    let _ = tj2.join();
}

fn shared_by_ref_counter() {
    let value = Arc::new(vec![1, 2, 3, 4]);

    // References are Copy, meaning that when you "move" them, the original still exists, just like with an integer or boolean.
    let tj1 = thread::spawn({
        let value = value.clone();
        move || {
            println!("shared_by_ref_counter Value is {:?}", value);
        }
    });

    let tj2 = thread::spawn({
        let value = value.clone();
        move || {
            println!("shared_by_ref_counter Value is {:?}", value);
        }
    });

    let _ = tj1.join();
    let _ = tj2.join();
}

// Ends Chapter 1

fn main() {
    // Chap 1
    let jh1 = thread::spawn(|| {
        f();
    });

    let jh2 = thread::spawn(|| {
        f();
    });

    jh1.join().unwrap();
    jh2.join().unwrap();

    let t = vec![1, 2, 3];

    // Here, ownership of numbers is transferred to the newly spawned thread, since we used a move closure.
    // If we had not used the move keyword, the closure would have captured numbers by reference.
    // This would have resulted in a compiler error, since the new thread might outlive that variable.

    // Since a thread might run until the very end of the program’s execution, the spawn function has a 'static lifetime bound on its argument type. In other words, it only accepts functions that may be kept around forever. A closure capturing a local variable by reference may not be kept around forever, since that reference would become invalid the moment the local variable ceases to exist.

    let jh = thread::spawn(move || {
        f1(&t);
    });

    jh.join().unwrap();

    // Scoped thread

    let number = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("th1: {number:?}");
        });
        s.spawn(|| {
            println!("th2: {number:?}");
        });
        s.spawn(|| {
            println!("th3: {number:?}");
        });
        s.spawn(|| {
            println!("th4: {number:?}");
        });
    });

    // Because ownership is shared, reference counting pointers (Rc<T> and Arc<T>) have the same restrictions as shared references (&T).
    // They do not give you mutable access to their contained value, since the value might be borrowed by other code at the same time.
    shared_by_static_ref();
    shared_by_leaking();
    shared_by_ref_counter();

    let t: [i8; 3] = [1, 2, 3];

    unsafe {
        let _p = t.get_unchecked(2);
    }

    extra_exa(1);
    // End Chapt 1
}

fn extra_exa(index: usize) {
    let x = || {};
    let y = || {};
    let z = |_v: usize| {};
    match index {
        0 => x(),
        1 => y(),
        _ => z(index),
    }

    let a = [123, 456, 789];
    let _b = unsafe { a.get_unchecked(index) };
}
