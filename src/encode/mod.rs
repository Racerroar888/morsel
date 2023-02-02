///
/// Checks if a string can safely be encoded. Returns false if the string contains unsupported characters.
///
/// # Example
/// ```
/// use morsel::encode::check;
///
/// let string:String = "The quick brown fox jumps over the lazy dog.".to_string();
/// let result = check(string);
///
/// assert_eq!(result, true)
/// ```
///

pub fn check(check:String) -> bool {
    let alphabet:&str = "abcdefghijklmnopqrstuvwxyz 1234567890.";
    for char in check.to_ascii_lowercase().chars() {
        if ! alphabet.contains(char) {
            return false;
        }
    }
    return true;
}

