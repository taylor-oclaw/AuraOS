impl LlmEngine {
    pub fn process_requests(&mut self) {
        for request in &self.requests {
            if let Some(model) = self.models.iter().find(|m| m.name == request.model_name) {
                if model.loaded {
                    let tokens_used = request.max_tokens;
                    let cost = tokens_used * model.cost_per_token;
                    self.total_tokens += tokens_used;
                    self.total_cost += cost;

                    let response_text = format!("Response to prompt: {}", request.prompt);
                    self.responses.push(LlmResponse {
                        request_id: request.id,
                        text: response_text,
                        tokens_used,
                        cost,
                        model_used: model.name.clone(),
                    });
                } else {
                    // Handle model not loaded scenario
                }
            } else {
                // Handle model not found scenario
            }
        }
        self.requests.clear();
    }

    pub fn load_model(&mut self, model_name: &str) -> bool {
        if let Some(model) = self.models.iter_mut().find(|m| m.name == model_name) {
            model.loaded = true;
            true
        } else {
            false
        }
    }

    pub fn unload_model(&mut self, model_name: &str) -> bool {
        if let Some(model) = self.models.iter_mut().find(|m| m.name == model_name) {
            model.loaded = false;
            true
        } else {
            false
        }
    }
}
