fn spiral(m: usize, n: usize, t: usize, matrix: &Vec<Vec<i32>>, res: &mut Vec<i32>, tl: usize) {
    let mut r = 0 + t;
    let mut c = 0 + t;

    while c < n - 1 && res.len() < tl {
        // println!("1 > r{:?} c{:?} {:?}", r, c, matrix[r][c]);
        res.push(matrix[r][c]);
        c += 1;
    }

    while r < m && res.len() < tl {
        // println!("2 > r{:?} c{:?} {:?}", r, c, matrix[r][c]);
        res.push(matrix[r][c]);
        r += 1;
    }

    r -= 1;
    while c > 0 + t && res.len() < tl {
        c -= 1;
        // println!("3 > r{:?} c{:?} {:?}", r, c, matrix[r][c]);
        res.push(matrix[r][c]);
    }

    r -= 1;
    while r > 0 + t && res.len() < tl {
        // println!("4 > r{:?} c{:?} {:?}", r, c, matrix[r][c]);
        res.push(matrix[r][c]);
        r -= 1;
    }
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut res = Vec::with_capacity(m * n);
    if m == 1 && n == 1 {
        res.push(matrix[0][0]);
    } else {
        let time = if m > n { n } else { m };
        for t in 0..time {
            let tl = m * n;
            if res.len() >= tl {
                // println!("=================");
                break;
            }
            spiral(m - t, n - t, t, &matrix, &mut res, m * n);
        }
    }

    // println!("res is {:?}", res);
    // println!("len is {:?}", res.len());
    res
}

fn main() {
    let i = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];
    spiral_order(i);

    let i = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
    ];
    spiral_order(i);

    let i = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
    ];
    spiral_order(i);

    let i = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        // vec![16, 17, 18, 19, 20],
    ];
    spiral_order(i);

    let i = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]];
    spiral_order(i);

    let i = vec![vec![1, 2, 3], vec![4, 5, 6]];
    spiral_order(i);

    let i = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    spiral_order(i);

    let i = vec![vec![1, 2], vec![3, 4]];
    spiral_order(i);

    let i = vec![vec![1], vec![2]];
    spiral_order(i);

    let i = vec![vec![1]];
    spiral_order(i);
}
