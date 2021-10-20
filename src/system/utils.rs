pub struct Utils {}

impl Utils {
    pub fn capitalize_first_letter(value: &str) -> String {
        if value.len() < 1 {
            return String::from("");
        }
        let mut chars = value.chars().collect::<Vec<char>>();
        chars[0] = chars[0].to_uppercase().nth(0).unwrap();
        chars.into_iter().collect::<String>()
    }
}
