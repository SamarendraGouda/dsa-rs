//Given a string s, return true if the s can be palindrome after deleting at most one character from it.
pub fn valid_palindrome(s: String) -> bool {
    let mut s = s.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return is_palindrome(&s, i + 1, j) || is_palindrome(&s, i, j - 1);
        }
        i += 1;
        j -= 1;
    }
    true
}

fn is_palindrome(s: &Vec<char>, mut i: usize, mut j: usize) -> bool {
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
