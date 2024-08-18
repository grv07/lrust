use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut m: HashMap<String, usize> = HashMap::with_capacity(strs.len());
    let mut c: usize = 0;
    let mut res: Vec<Vec<String>> = Vec::with_capacity(strs.len());
    for s in strs {
        let rs = s.clone();
        let mut p = s.into_bytes();
        p.sort();

        let t = String::from_utf8(p).unwrap();
        if m.contains_key(&t) {
            let i = m.get(&t).unwrap();
            res[i.to_owned()].push(rs);
        } else {
            m.insert(t, c);
            res.push(vec![rs]);
            c += 1;
        }
    }
    res
}

fn main() {
    let i = vec!["eat", "ate"];
    let i = i.iter().map(|v| v.to_string()).collect();
    let o = vec![vec!["eat", "ate"]];
    let o: Vec<Vec<String>> = o
        .iter()
        .map(|v| v.iter().map(|v| v.to_string()).collect())
        .collect();
    assert_eq!(o, group_anagrams(i));

    let i = vec!["eaa", "ate"];
    let i = i.iter().map(|v| v.to_string()).collect();
    let o = vec![vec!["eaa"], vec!["ate"]];
    let o: Vec<Vec<String>> = o
        .iter()
        .map(|v| v.iter().map(|v| v.to_string()).collect())
        .collect();
    assert_eq!(o, group_anagrams(i));

    let i = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
    let i = i.iter().map(|v| v.to_string()).collect();
    let o = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]];
    let o: Vec<Vec<String>> = o
        .iter()
        .map(|v| v.iter().map(|v| v.to_string()).collect())
        .collect();
    assert_eq!(o, group_anagrams(i));

    let i = vec![""];
    let i = i.iter().map(|v| v.to_string()).collect();
    let o = vec![vec![""]];
    let o: Vec<Vec<String>> = o
        .iter()
        .map(|v| v.iter().map(|v| v.to_string()).collect())
        .collect();
    assert_eq!(o, group_anagrams(i));
}
