pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut ps = &intervals[0][0];
    let mut pl = &intervals[0][1];

    for i in 1..intervals.len() {
        let s = &intervals[i][0];
        let l = &intervals[i][1];

        if pl >= s {
            pl = l;
        } else if l > ps {
            ps = s;
        } else {
            res.push(vec![ps.clone(), pl.clone()]);
            ps = s;
            pl = l;
        }
        // println!("s:{:?} l:{:?}", s, l);
    }
    res.push(vec![ps.clone(), pl.clone()]);
    res
}

fn main() {
    let i = vec![vec![1, 4], vec![0, 0]];
    let o = vec![vec![1, 4], vec![0, 0]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![1, 6], vec![0, 5]];
    let o = vec![vec![0, 6]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![2, 4], vec![1, 6], vec![0, 5], vec![5, 17]];
    let o = vec![vec![0, 17]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![1, 2], vec![0, 5]];
    let o = vec![vec![0, 5]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let o = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![1, 4], vec![4, 6], vec![8, 10], vec![15, 18]];
    let o = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![1, 4], vec![4, 5]];
    let o = vec![vec![1, 5]];
    let res = merge(i);
    assert_eq!(o, res);

    let i = vec![vec![0, 0], vec![0, 0]];
    let o = vec![vec![0, 0]];
    let res = merge(i);
    assert_eq!(o, res);
}
