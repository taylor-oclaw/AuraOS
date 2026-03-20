extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

mod ai_multimodal_router {
    pub struct AINativeRouter {
        routes: Vec<(String, String)>,
    }

    impl AINativeRouter {
        pub fn new() -> Self {
            AINativeRouter { routes: Vec::new() }
        }

        pub fn add_route(&mut self, input_type: &str, output_type: &str) {
            let input = String::from(input_type);
            let output = String::from(output_type);
            self.routes.push((input, output));
        }

        pub fn get_output_type(&self, input_type: &str) -> Option<&String> {
            for (in_type, out_type) in &self.routes {
                if in_type == input_type {
                    return Some(out_type);
                }
            }
            None
        }

        pub fn remove_route(&mut self, input_type: &str) {
            self.routes.retain(|(in_type, _)| in_type != input_type);
        }

        pub fn list_routes(&self) -> Vec<(String, String)> {
            self.routes.iter().map(|(a, b)| (a.clone(), b.clone())).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ai_multimodal_router::AINativeRouter;

    #[test]
    fn test_add_and_get_route() {
        let mut router = AINativeRouter::new();
        router.add_route("text", "speech");
        assert_eq!(router.get_output_type("text"), Some(&String::from("speech")));
    }

    #[test]
    fn test_remove_route() {
        let mut router = AINativeRouter::new();
        router.add_route("image", "description");
        router.remove_route("image");
        assert_eq!(router.get_output_type("image"), None);
    }

    #[test]
    fn test_list_routes() {
        let mut router = AINativeRouter::new();
        router.add_route("audio", "text");
        router.add_route("video", "summary");
        let routes = router.list_routes();
        assert_eq!(routes.len(), 2);
        assert!(routes.contains(&(String::from("audio"), String::from("text"))));
        assert!(routes.contains(&(String::from("video"), String::from("summary"))));
    }
}
