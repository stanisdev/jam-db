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

    pub fn snake_case_to_camel_case(value: &str) -> String {
        value
            .split('_')
            .into_iter()
            .map(|elem| Self::capitalize_first_letter(elem))
            .collect::<String>()
    }
}
