pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let k = k as usize;
    let n = nums.len();
    let m: usize = if k > n {
        let mod_o = k % n;
        if mod_o == 0 {
            return;
        } else {
            n - mod_o
        }
    } else {
        n - k
    };
    // println!("M {:?} ", m);

    if m == 0 {
        return;
    }

    let mut cnt = 0;
    let mut last = nums[n - 1];
    let mut j = n - 1;
    // println!("M {:?} {:?} ", m, j);
    while cnt < n {
        if j < m {
            return;
        }
        let tmp = nums[cnt];
        nums[cnt] = last;
        last = tmp;
        cnt += 1;
        if cnt == m {
            nums[j] = last;
            last = nums[j - 1];
            cnt = 0;
            j -= 1;
        }
    }
}

fn main() {
    let mut i = vec![1, 2, 3, 4, 5, 6, 7];
    let o = vec![7, 1, 2, 3, 4, 5, 6];
    rotate(&mut i, 1);
    assert_eq!(i, o);

    let mut i = vec![1, 2, 3, 4, 5, 6, 7];
    let o = vec![6, 7, 1, 2, 3, 4, 5];
    rotate(&mut i, 2);
    assert_eq!(i, o);

    let mut i = vec![1, 2, 3, 4, 5, 6, 7];
    let o = vec![4, 5, 6, 7, 1, 2, 3];
    rotate(&mut i, 4);
    assert_eq!(i, o);

    let mut i = vec![1, 2, 3, 4, 5, 6, 7];
    let o = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut i, 7);
    assert_eq!(i, o);

    let mut i = vec![1, 2, 3, 4, 5, 6, 7];
    let o = vec![7, 1, 2, 3, 4, 5, 6];
    rotate(&mut i, 8);
    assert_eq!(i, o);
}
