fn check_record(s: &str) -> bool {
    let mut a_c = 0;
    let mut l_c = 0;
    for c in s.bytes() {
        if c == 65 {
            a_c = a_c + 1;
        }
        if a_c >= 2 {
            return false;
        }

        if c == 76 {
            l_c = l_c + 1;
        }

        if c != 76 {
            l_c = 0;
        }

        if l_c >= 3 {
            return false;
        }
    }
    return true;
}

fn main() {
    assert!(check_record("PPPPPP"));
    assert!(check_record("PPALLP"));
    assert!(!check_record("ALLLP"));
    assert!(!check_record("ALLPLLAP"));
    assert!(check_record("ALLPLLP"));
    assert!(!check_record("PPPPLLAA"));
    assert!(check_record("PPPPLLA"));
    assert!(!check_record("APPAPP"));
}
