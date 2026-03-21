extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod speech_phoneme_substitution {
    use super::*;

    pub struct PhonemeSubstitutor {
        substitution_map: Vec<(String, String)>,
    }

    impl PhonemeSubstitutor {
        pub fn new() -> Self {
            PhonemeSubstitutor {
                substitution_map: Vec::new(),
            }
        }

        pub fn add_substitution(&mut self, from: &str, to: &str) {
            let from_str = String::from(from);
            let to_str = String::from(to);
            self.substitution_map.push((from_str, to_str));
        }

        pub fn substitute(&self, input: &str) -> String {
            let mut result = String::new();
            let mut chars = input.chars().peekable();

            while let Some(c) = chars.next() {
                if c == ' ' {
                    result.push(' ');
                    continue;
                }

                let mut phoneme = String::from(c);
                while let Some(&next_c) = chars.peek() {
                    if next_c != ' ' {
                        phoneme.push(next_c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if let Some(substitution) = self.find_substitution(&phoneme) {
                    result.push_str(substitution);
                } else {
                    result.push_str(&phoneme);
                }
            }

            result
        }

        fn find_substitution(&self, phoneme: &str) -> Option<&String> {
            for (from, to) in &self.substitution_map {
                if from == phoneme {
                    return Some(to);
                }
            }
            None
        }

        pub fn remove_substitution(&mut self, from: &str) {
            self.substitution_map.retain(|(f, _)| f != from);
        }

        pub fn list_substitutions(&self) -> Vec<(String, String)> {
            self.substitution_map.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::speech_phoneme_substitution::*;

    #[test]
    fn test_phoneme_substitutor() {
        let mut substitutor = PhonemeSubstitutor::new();
        substitutor.add_substitution("th", "z");
        substitutor.add_substitution("ing", "ink");

        assert_eq!(substitutor.substitute("this is a thing"), "zis is a zink");
        assert_eq!(substitutor.substitute("think of the time"), "zinck of ze time");

        substitutor.remove_substitution("th");
        assert_eq!(substitutor.substitute("this is a thing"), "this is a zink");

        let substitutions = substitutor.list_substitutions();
        assert_eq!(substitutions.len(), 1);
        assert_eq!(&substitutions[0], ("ing", "ink"));
    }
}
