use std::collections::HashMap;

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut r = vec![];

    let mut d_f = HashMap::from([
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
        (9, 0),
        (0, 0),
    ]);

    for i in digits {
        if let Some(v) = d_f.get_mut(&i) {
            *v += 1;
        }
    }

    for d in 100..999 {
        if d % 2 == 0 {
            let (i, j, k) = (d / 100, (d - d % 10) / 10 % 10, d % 10);
            d_f.get_mut(&i).map(|v| *v -= 1);
            d_f.get_mut(&j).map(|v| *v -= 1);
            d_f.get_mut(&k).map(|v| *v -= 1);

            if let (Some(i), Some(j), Some(k)) = (d_f.get(&i), d_f.get(&j), d_f.get(&k)) {
                let z = &0;
                if i >= z && j >= z && k >= z {
                    r.push(d);
                }
            }

            d_f.get_mut(&i).map(|v| *v += 1);
            d_f.get_mut(&j).map(|v| *v += 1);
            d_f.get_mut(&k).map(|v| *v += 1);
        }
    }
    r
}

fn main() {
    let r = find_even_numbers(vec![1, 2, 3]);
    let o = vec![132, 312];
    let res: Vec<&i32> = r.iter().filter(|x| o.contains(x)).collect();
    // println!("{:?}", res);
    assert!(!res.is_empty());

    let r = find_even_numbers(vec![2, 1, 3, 0]);
    let o = vec![102, 120, 130, 132, 210, 230, 302, 310, 312, 320];
    let res: Vec<&i32> = r.iter().filter(|x| !o.contains(x)).collect();
    assert!(res.is_empty());

    let r = find_even_numbers(vec![2, 2, 3, 2]);
    let o = vec![222, 232, 322];
    let res: Vec<&i32> = r.iter().filter(|x| !o.contains(x)).collect();
    // println!("{:?}", res);
    assert!(res.is_empty());

    let r = find_even_numbers(vec![2, 2, 2, 2, 2, 2]);
    let o = vec![222, 232, 322];
    let res: Vec<&i32> = r.iter().filter(|x| !o.contains(x)).collect();
    // println!("{:?}", res);
    assert!(res.is_empty());
}
