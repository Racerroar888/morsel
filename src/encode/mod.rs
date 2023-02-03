///
/// Checks if a string can safely be encoded. Returns false if the string contains unsupported characters.
///
/// # Example
/// ```
/// use morsel::encode::check;
///
/// let string:String = "SOS".to_string();
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

///
/// Encodes a string in morse code.
///
/// # Example
/// ```
/// use morsel::encode::encode;
///
/// let string:String = "SOS".to_string();
/// let result:String = match encode(string){
///     Ok(string) => string,
///     Err(error) => error,
/// };
///
/// assert_eq!(result, " ... --- ...".to_string())
///
/// ```
///

pub fn encode(encode:String) -> Result<String, String> {
    if ! check(encode.clone()) {
        return Err("String contains unsupported characters.".to_string());
    }
    let mut encoded:String = String::new();
    for char in encode.to_ascii_lowercase().chars() {
        encoded.push_str(" ");
        match char {
            ' ' => encoded.push_str("  "),
            'a' => encoded.push_str(".-"),
            'b' => encoded.push_str("-..."),
            'c' => encoded.push_str("-.-."),
            'd' => encoded.push_str("-.."),
            'e' => encoded.push_str("."),
            'f' => encoded.push_str("..-."),
            'g' => encoded.push_str("--."),
            'h' => encoded.push_str("...."),
            'i' => encoded.push_str(".."),
            'j' => encoded.push_str(".---"),
            'k' => encoded.push_str("-.-"),
            'l' => encoded.push_str(".-.."),
            'm' => encoded.push_str("--"),
            'n' => encoded.push_str("-."),
            'o' => encoded.push_str("---"),
            'p' => encoded.push_str(".--."),
            'q' => encoded.push_str("--.-"),
            'r' => encoded.push_str(".-."),
            's' => encoded.push_str("..."),
            't' => encoded.push_str("-"),
            'u' => encoded.push_str("..-"),
            'v' => encoded.push_str("...-"),
            'w' => encoded.push_str(".--"),
            'x' => encoded.push_str("-..-"),
            'y' => encoded.push_str("-.--"),
            'z' => encoded.push_str("--.."),
            '0' => encoded.push_str("-----"),
            '1' => encoded.push_str(".----"),
            '2' => encoded.push_str("..---"),
            '3' => encoded.push_str("...--"),
            '4' => encoded.push_str("....-"),
            '5' => encoded.push_str("....."),
            '6' => encoded.push_str("-...."),
            '7' => encoded.push_str("--..."),
            '8' => encoded.push_str("---.."),
            '9' => encoded.push_str("----."),
            '.' => encoded.push_str(".-.-.-"),
            _ => {}
        }
    }
    return Ok(encoded);
}