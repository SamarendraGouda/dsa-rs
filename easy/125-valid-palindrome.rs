pub fn is_palindrome(s: String) -> bool {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if !s[i].is_alphanumeric() {
            i += 1;
            continue;
        }
        if !s[j].is_alphanumeric() {
            j -= 1;
            continue;
        }
        if s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
