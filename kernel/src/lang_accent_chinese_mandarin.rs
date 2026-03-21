extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;

mod lang_accent_chinese_mandarin {
    use super::*;

    pub struct MandarinAccent {
        phrases: Vec<String>,
        greetings: Vec<String>,
        farewells: Vec<String>,
        questions: Vec<String>,
        compliments: Vec<String>,
    }

    impl MandarinAccent {
        pub fn new() -> Self {
            MandarinAccent {
                phrases: vec![
                    String::from("你好，世界！"), // Hello, world!
                    String::from("我喜欢编程。"),  // I like programming.
                    String::from("今天天气真好。"), // The weather is so nice today.
                    String::from("我爱我的家。"),   // I love my home.
                    String::from("祝你一切顺利！"), // Wish you all the best!
                ],
                greetings: vec![
                    String::from("你好"),    // Hello
                    String::from("早上好"),  // Good morning
                    String::from("下午好"),  // Good afternoon
                    String::from("晚上好"),  // Good evening
                    String::from("晚安"),    // Good night
                ],
                farewells: vec![
                    String::from("再见"),   // Goodbye
                    String::from("拜拜"),   // Bye-bye
                    String::from("祝你一路平安"), // Have a safe journey
                    String::from("到那边见"),  // See you there
                    String::from("晚安"),    // Good night
                ],
                questions: vec![
                    String::from("你好吗？"),   // How are you?
                    String::from("你今天过得怎么样？"), // How was your day?
                    String::from("你喜欢做什么？"),  // What do you like to do?
                    String::from("你最喜欢的食物是什么？"), // What is your favorite food?
                    String::from("你明天有计划吗？"), // Do you have plans for tomorrow?
                ],
                compliments: vec![
                    String::from("你真聪明！"),   // You are so smart!
                    String::from("你的笑容很灿烂。"), // Your smile is so bright.
                    String::from("你做得很好！"),  // You did a great job!
                    String::from("你很有才华！"),  // You are very talented!
                    String::from("你真有礼貌！"),  // You are so polite!
                ],
            }
        }

        pub fn get_phrase(&self, index: usize) -> Option<&String> {
            self.phrases.get(index)
        }

        pub fn get_greeting(&self, index: usize) -> Option<&String> {
            self.greetings.get(index)
        }

        pub fn get_farewell(&self, index: usize) -> Option<&String> {
            self.farewells.get(index)
        }

        pub fn get_question(&self, index: usize) -> Option<&String> {
            self.questions.get(index)
        }

        pub fn get_compliment(&self, index: usize) -> Option<&String> {
            self.compliments.get(index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::lang_accent_chinese_mandarin::MandarinAccent;

    #[test]
    fn test_new() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.phrases.len(), 5);
        assert_eq!(accent.greetings.len(), 5);
        assert_eq!(accent.farewells.len(), 5);
        assert_eq!(accent.questions.len(), 5);
        assert_eq!(accent.compliments.len(), 5);
    }

    #[test]
    fn test_get_phrase() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.get_phrase(0), Some(&String::from("你好，世界！")));
        assert_eq!(accent.get_phrase(1), Some(&String::from("我喜欢编程。")));
        assert_eq!(accent.get_phrase(5), None);
    }

    #[test]
    fn test_get_greeting() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.get_greeting(0), Some(&String::from("你好")));
        assert_eq!(accent.get_greeting(1), Some(&String::from("早上好")));
        assert_eq!(accent.get_greeting(5), None);
    }

    #[test]
    fn test_get_farewell() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.get_farewell(0), Some(&String::from("再见")));
        assert_eq!(accent.get_farewell(1), Some(&String::from("拜拜")));
        assert_eq!(accent.get_farewell(5), None);
    }

    #[test]
    fn test_get_question() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.get_question(0), Some(&String::from("你好吗？")));
        assert_eq!(accent.get_question(1), Some(&String::from("你今天过得怎么样？")));
        assert_eq!(accent.get_question(5), None);
    }

    #[test]
    fn test_get_compliment() {
        let accent = MandarinAccent::new();
        assert_eq!(accent.get_compliment(0), Some(&String::from("你真聪明！")));
        assert_eq!(accent.get_compliment(1), Some(&String::from("你的笑容很灿烂。")));
        assert_eq!(accent.get_compliment(5), None);
    }
}
