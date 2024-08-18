use std::collections::HashSet;

fn p(res: &mut HashSet<Vec<i32>>, mut v: Vec<i32>, f: usize) {
    let n = v.len();
    if f >= (n - 1) {
        // println!(" --- {:?}", v);
        res.insert(v);
        return;
    }

    // println!("start with n: {:?}, f: {:?}, v: {:?}", n, f, v);
    for i in f..n {
        // let mut v = v.clone();
        let tmp = v[f];

        if i != f && tmp == v[i] {
            continue;
        }

        v[f] = v[i];
        v[i] = tmp;
        // println!(
        //     "next start with i :{:?} n: {:?}, f: {:?}, v: {:?}",
        //     i,
        //     n,
        //     f + 1,
        //     v
        // );
        p(res, v.clone(), f + 1);
    }
    // println!(" >> --- {:?}", v);
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let fact = |mut n: usize| -> usize {
        let mut res = 1;
        while n > 1 {
            res *= n;
            n -= 1;
        }
        res
    };
    // println!("c=fact is {:?}", fact(nums.len()));

    let mut res: HashSet<Vec<i32>> = HashSet::with_capacity(fact(nums.len()));
    p(&mut res, nums, 0);
    let res: Vec<Vec<i32>> = res.into_iter().map(|d| d).collect();
    return res;
}

fn main() {
    let i = vec![1, 2, 3];
    let o = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    let res = permute(i);
    // println!("res -> {:?}", res);
    assert_eq!(res, o);

    let i = vec![1, 1, 3];
    let o = vec![vec![1, 1, 3], vec![1, 3, 1], vec![3, 1, 1]];

    let res = permute(i);
    // println!("res -> {:?}", res);
    assert_eq!(res, o);

    let i = vec![1, 1, 1];
    let o = vec![vec![1, 1, 1]];
    let res = permute(i);
    // println!("res -> {:?}", res);
    assert_eq!(res, o);

    let i = vec![1, 2];
    let o = vec![vec![1, 2], vec![2, 1]];
    let res = permute(i);
    // println!("res -> {:?}", res);
    assert_eq!(res, o);

    let i = vec![1];
    let o = vec![vec![1]];
    let res = permute(i);
    // println!("res -> {:?}", res);
    assert_eq!(res, o);

    let i = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let res = permute(i);
    assert_eq!(res.len(), 3628800);

    let i = vec![3, 3, 0, 3];
    let o = vec![
        vec![0, 3, 3, 3],
        vec![3, 0, 3, 3],
        vec![3, 3, 0, 3],
        vec![3, 3, 3, 0],
    ];

    let res = permute(i);

    // println!("res -> {:?}", res);
    assert_eq!(res, o);
}
