pub fn get_space(tab_level: u32) -> String {
    (0..tab_level)
        .map(|_| "| ".to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_space_tab_level_two() {
        assert_eq!(get_space(2), "| | ".to_string());
    }
}
