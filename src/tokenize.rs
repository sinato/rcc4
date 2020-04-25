pub fn tokenize(code: String) -> u64 {
    let mut chars = code.chars().peekable();

    let mut s = String::from("");
    while let Some(c) = chars.peek() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => s += &c.to_string(),
            _ => panic!("unexpected char {:?}", c),
        }
        chars.next();
    }
    let num: u64 = s.parse().unwrap_or_else(|err| panic!(err));
    num
}

#[cfg(test)]
mod tests {

    use self::super::*;

    #[test]
    fn sigle_number() {
        assert_eq!(tokenize("10".to_string()), 10);
    }

    #[test]
    #[should_panic(expected = "unexpected char \'a\'")]
    fn illegal_number() {
        tokenize("1a0".to_string());
    }
}
