pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let m = nums1.len();
    let n = nums2.len();

    if m == 0 && n == 0 {
        return 0_f64;
    }

    let max = (m + n) / 2;

    // println!("max {:?}", max);

    let mut pv = &0;
    let mut nv = &0;
    let mut i = 0;
    let mut j = 0;
    while i + j <= max {
        // println!("{:?} {:?}", i, j);
        let v1 = nums1.get(i);
        let v2 = nums2.get(j);

        // println!("{:?} {:?}", v1, v2);
        pv = nv;

        if !v1.is_some() {
            // println!("{:?} {:?}", pv, nv);
            nv = v2.unwrap();
            j += 1;
            continue;
        }

        if !v2.is_some() {
            nv = v1.unwrap();
            // println!("{:?} {:?}", pv, nv);
            i += 1;
            continue;
        }

        if v1 < v2 {
            i += 1;
            nv = v1.unwrap();
        } else {
            j += 1;
            nv = v2.unwrap();
        }
    }

    // println!("{:?} {:?}", pv, nv);
    if (m + n) % 2 == 0 {
        return (pv + nv) as f64 / 2 as f64;
    }
    return nv.to_owned() as f64;
}

fn main() {
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3]), 2.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(find_median_sorted_arrays(vec![1, 1], vec![3]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 1], vec![]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![0, 1, 1], vec![0, 3]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![1, 1], vec![0, 0, 3]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![1], vec![]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(find_median_sorted_arrays(vec![], vec![]), 0.0);
    assert_eq!(
        find_median_sorted_arrays(vec![1, 5], vec![3, 7, 8, 19]),
        6.0
    );
    // [1, ,3, 5, 7, 8, 19]
}
