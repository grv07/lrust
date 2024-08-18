fn valid_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut resu = s.to_string();

    for x in t.chars() {
        let new_resu = resu.replacen(x, "", 1);
        if new_resu.len() == resu.len() {
            return false;
        }
        resu = new_resu;
    }
    return resu.len() == 0;
}

fn main() {
    assert!(valid_anagram("anagram", "nagaram"));
    assert!(!valid_anagram("rat", "car"));

    assert!(valid_anagram("flatten", "tetlanf"));
    assert!(!valid_anagram("aaaab", "aaaaa"));

    assert!(!valid_anagram("aaaabb", "aaaaa"));
    assert!(!valid_anagram("ac", "bb"));
}
