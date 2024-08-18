pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashMap;
    let vr = || -> bool {
        let mut res = true;
        for i in 0..9 {
            let mut rm = HashMap::new();
            let mut cm = HashMap::new();
            for j in 0..9 {
                let rch = board[i][j];
                let cch = board[j][i];
                if rch != '.' {
                    if let Some(_) = rm.get(&rch) {
                        res = false;
                        break;
                    } else {
                        rm.insert(rch, 1);
                    }
                }
                if cch != '.' {
                    if let Some(_) = cm.get(&cch) {
                        res = false;
                        break;
                    } else {
                        cm.insert(cch, 1);
                    }
                }
            }
            if !res {
                break;
            }
            // println!("{:?}", rm);
            // println!("{:?}", cm);
        }
        res
    };

    if !vr() {
        return false;
    }

    let vb = || -> bool {
        let mut res = true;
        let mut bj: i32 = -1;

        for m in 0..9 {
            let mut bm = HashMap::new();
            bj += 1;
            if (m) % 3 == 0 {
                bj = 0;
            }
            for i in 0..3 {
                // println!("_");
                for j in 0..3 {
                    let j = j + ((bj) * 3);
                    let i = i + ((m / 3) * 3);
                    // print!("({:?} {:?}) ", i, j);
                    let bch = board[i][j as usize];
                    if bch != '.' {
                        if let Some(_) = bm.get(&bch) {
                            res = false;
                            break;
                        } else {
                            bm.insert(bch, 1);
                        }
                    }
                }
                // println!("_");
            }
            // println!("{:?}", bm);
            if !res {
                break;
            }
        }
        return res;
    };

    if !vb() {
        return false;
    }
    return true;
}

fn main() {
    let i = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '4', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    assert_eq!(is_valid_sudoku(i), true);

    let i = vec![
        vec!['.', '2', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '5', '.', '1'],
        vec!['.', '.', '.', '.', '.', '.', '8', '1', '3'],
        vec!['4', '.', '9', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '2', '.', '.', '.', '.', '.', '.'],
        vec!['7', '.', '6', '.', '.', '.', '.', '.', '.'],
        vec!['9', '.', '.', '.', '.', '4', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
    ];

    assert_eq!(is_valid_sudoku(i), false);
}
