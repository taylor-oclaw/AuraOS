extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() {
    let mut converter = asf_auto_convert::AutoConverter::new();
    converter.add_conversion("AI", "Artificial Intelligence");
    converter.add_conversion("ML", "Machine Learning");
    converter.add_conversion("DL", "Deep Learning");

    let converted_text = converter.convert_text("This is an AI example. We use ML and DL techniques.");
    println!("Converted Text: {}", converted_text);
}

mod asf_auto_convert {
    extern crate alloc;
    use alloc::string::String;
    use alloc::vec::Vec;

    pub struct AutoConverter {
        conversions: Vec<(String, String)>,
    }

    impl AutoConverter {
        pub fn new() -> Self {
            AutoConverter {
                conversions: Vec::new(),
            }
        }

        pub fn add_conversion(&mut self, abbreviation: &str, full_form: &str) {
            self.conversions.push((abbreviation.to_string(), full_form.to_string()));
        }

        pub fn convert_text(&self, text: &str) -> String {
            let mut converted = String::from(text);
            for (abbrev, full_form) in &self.conversions {
                converted = converted.replace(abbrev, full_form);
            }
            converted
        }

        pub fn get_conversions(&self) -> Vec<(String, String)> {
            self.conversions.clone()
        }

        pub fn remove_conversion(&mut self, abbreviation: &str) {
            self.conversions.retain(|&(ref abbr, _)| abbr != abbreviation);
        }
    }
}
