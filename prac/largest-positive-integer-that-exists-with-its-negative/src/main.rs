fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut check = std::collections::HashMap::new();
    let mut max = -1;
    for i in nums {
        let p_i = if i.is_negative() { i * -1 } else { i };
        match check.get(&p_i) {
            Some(v) => {
                if v + i == 0 {
                    if p_i > max {
                        max = p_i;
                    }
                }
            }
            None => {
                check.insert(p_i, i);
            }
        }
    }
    return max;
}
fn main() {
    assert_eq!(find_max_k(vec![-1, 2, -3, 3]), 3);

    assert_eq!(find_max_k(vec![-1, 10, 6, 7, -7, 1]), 7);

    assert_eq!(find_max_k(vec![-1, 10, 6, 7, 7, 1]), 1);

    assert_eq!(find_max_k(vec![-1, 10, 6, -7, -7, 1]), 1);

    assert_eq!(find_max_k(vec![-1, 10, 6, -7, 7, 1, -7]), 7);

    assert_eq!(find_max_k(vec![-18, 10, 6, -7, 7, 1, -7, 18]), 18);

    assert_eq!(find_max_k(vec![-10, 8, 6, 7, -2, -3]), -1);
}
