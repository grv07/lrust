use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicI16},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

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

fn mutex_example() {
    let m = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = m.lock().unwrap();

                for _ in 0..100 {
                    *guard += 1;
                }
                // If we remove this below drop it will keeps the guard around till outer for is complete and will take 10 sec to complete.
                // if we drop it will only take 1 sec to complete bcs of concurrency.
                drop(guard);
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    assert_eq!(m.into_inner().unwrap(), 1000);
}

fn parking_example() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());

    let t = thread::scope(|s| {
        let consumer = s.spawn(|| loop {
            println!("start consumer");
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                println!("{item:?}");
            } else {
                println!("park");
                thread::park();
            }
        });

        for i in 0..5 {
            let _ = queue.lock().unwrap().push_back(i);
            // NOTE Following sequence of operation will not drop MutexGuard;
            // So it will first push all of the items then wakes up the consumer.
            // BCS consumer will always waits for the mutex lock.
            // let mut item = queue.lock().unwrap();
            // item.push_back(i);

            println!("push and unpark");
            consumer.thread().unpark();

            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn cond_var_example() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        let _ = s.spawn(|| {
            println!("Start consumer");
            for _ in 0..20 {
                let mut guard = queue.lock().unwrap();
                let item = guard.pop_back();
                if let Some(item) = item {
                    println!("Item: {item}");
                } else {
                    println!("Empty ... ");
                    guard = not_empty.wait(guard).unwrap();
                }
                drop(guard);
            }
        });

        for i in 1..10 {
            println!("Push data");

            queue.lock().unwrap().push_back(i);
            queue.lock().unwrap().push_back(i + 1);

            not_empty.notify_one();

            thread::sleep(Duration::from_secs(1));
        }
    });

    // let consumer =
}

// Ends Chapter 1

// Starts Chapter 2
fn stop_flag() {
    static STOP: AtomicBool = AtomicBool::new(false);

    let t = thread::spawn(|| {
        println!("start...");
        while !STOP.load(std::sync::atomic::Ordering::Relaxed) {}
    });

    println!("waiting for input ...");
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "stop" => {
                STOP.swap(true, std::sync::atomic::Ordering::Relaxed);
                break;
            }
            "help" => println!("command: help, stop"),
            cmd => println!("Unknown command found {cmd}"),
        }
    }

    let _ = t.join();
}

fn progress_example() {
    let progress = AtomicI16::new(0);

    let thread = thread::current();
    thread::scope(|s| {
        let _t = s.spawn(|| {
            println!("start progress");
            for _ in 0..100 {
                progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                // progress.store(i + 1, std::sync::atomic::Ordering::Relaxed);
                thread::sleep(Duration::from_millis(10));
                thread.unpark();
            }
        });

        loop {
            let n = progress.load(std::sync::atomic::Ordering::Relaxed);

            std::process::Command::new("clear").status().unwrap();
            println!("Working on.. {n}/100");

            if n == 100 {
                break;
            }

            thread::park_timeout(Duration::from_millis(10));
        }
    });
}

// Ends Chapter 2

fn main() {
    // stop_flag();
    progress_example();
}

fn chap1() {
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

    mutex_example();

    let t: [i8; 3] = [1, 2, 3];

    unsafe {
        let _p = t.get_unchecked(2);
    }

    extra_exa(1);

    // parking_example();
    cond_var_example();

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
