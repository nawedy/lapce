use crate::ai_service_handler::{AIServiceHandler, AIRequest, AIResponse};
use crate::ai_service::{AICapability, AIModelConfig}; // Assuming these are needed for model selection display
use crate::ai_commands::AICommands; // For processing commands

// Represents a message in the chat history
pub struct ChatMessage {
    sender: String, // "User" or "AI"
    content: String,
    is_code_snippet: bool, // To handle code snippet display differently
}

// The user interface for interacting with AI
pub struct AIChatPanel {
    history: Vec<ChatMessage>,
    service_handler: AIServiceHandler, // To send requests to the AI
    command_processor: AICommands,   // To parse and process commands
    available_models: Vec<String>, // Names of models for selection dropdown
    selected_model: Option<String>,  // Currently selected model
                                   // UI specific fields would go here (e.g., text input, message display area)
}

impl AIChatPanel {
    // Create a new AIChatPanel
    pub fn new(service_handler: AIServiceHandler, command_processor: AICommands, initial_models: Vec<String>) -> Self {
        Self {
            history: Vec::new(),
            service_handler,
            command_processor,
            available_models: initial_models,
            selected_model: None,
        }
    }

    // Called when the user sends a message
    pub fn send_message(&mut self, text: String) {
        self.add_message("User".to_string(), text.clone(), false);

        let parsed_command = self.command_processor.parse_input(&text);

        // Use the capability from the parsed command, or default if None
        // (AICommands.parse_input now always provides a capability, typically Chat for unknown commands)
        let capability_to_use = parsed_command.capability.unwrap_or(AICapability::Chat);

        // The prompt for the AI might be the full input or just arguments,
        // depending on the command type. For simplicity, we'll use the full input for now.
        // For commands like /help, we might handle them directly in AIChatPanel.
        if parsed_command.command_type == crate::ai_commands::CommandType::Help {
            let help_text = self.command_processor.get_help();
            self.add_message("System".to_string(), help_text, false);
            return; // Don't send help commands to AI service
        }

        let request = AIRequest {
            capability: capability_to_use,
            prompt: parsed_command.full_input, // Or construct prompt from parsed_command.arguments
            // Context would be gathered here or in the service_handler
        };

        let ai_response = self.service_handler.process_request(request);
        self.add_message("AI".to_string(), ai_response.content, false); // Assuming response content is not code by default
    }

    // Add a message to the chat history
    fn add_message(&mut self, sender: String, content: String, is_code_snippet: bool) {
        let message = ChatMessage {
            sender,
            content,
            is_code_snippet,
        };
        self.history.push(message);
        // In a real UI, this would trigger a redraw of the chat panel
        self.render_chat();
    }

    // Select an AI model from the dropdown
    pub fn select_model(&mut self, model_name: String) {
        if self.available_models.contains(&model_name) {
            self.selected_model = Some(model_name);
            println!("Model selected: {:?}", self.selected_model);
            // Potentially reconfigure service_handler or notify it
        } else {
            eprintln!("Model {} not available.", model_name);
        }
    }

    // Placeholder for rendering the chat (in a real UI, this would update the view)
    fn render_chat(&self) {
        println!("\n--- Chat History ---");
        for message in &self.history {
            println!("{}: {}", message.sender, message.content);
            if message.is_code_snippet {
                println!("[Code Snippet - Copy/Insert options would be here]");
            }
        }
        println!("--------------------\n");
    }

    // Handle code snippets (display, copy, insert)
    // This is highly dependent on the UI framework
    pub fn handle_code_snippet(&self, code: String) {
        // For now, just adds it as a special message
        self.add_message("AI".to_string(), code, true);
    }

    // Gather context from active editors, etc. (placeholder)
    // This might be called before sending a request
    pub fn gather_context_for_ui(&self) -> String {
        // In a real scenario, this would interact with the IDE's state
        "Context from active editor: [Editor Content]".to_string()
    }

    // Update the list of available models (e.g., when models are configured)
    pub fn update_available_models(&mut self, models: Vec<String>) {
        self.available_models = models;
        // If current selected model is no longer available, reset it
        if let Some(selected) = &self.selected_model {
            if !self.available_models.contains(selected) {
                self.selected_model = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_service::{AIService, AIModelConfig, AIProvider};
    // AIServiceHandler and AICommands are needed.
    // We need a mock or simple AIService for the handler.

    fn setup_chat_panel() -> AIChatPanel {
        let ai_service = AIService::new(); // Uses default models
        let service_handler = AIServiceHandler::new(ai_service);
        let command_processor = AICommands::new(); // Assuming AICommands has a simple new()
        let models = vec!["gpt-3.5-turbo".to_string(), "claude-2".to_string()];
        AIChatPanel::new(service_handler, command_processor, models)
    }

    #[test]
    fn test_send_and_receive_message() {
        let mut chat_panel = setup_chat_panel();
        chat_panel.send_message("Hello AI!".to_string());

        assert_eq!(chat_panel.history.len(), 2);
        assert_eq!(chat_panel.history[0].sender, "User");
        assert_eq!(chat_panel.history[0].content, "Hello AI!");
        assert_eq!(chat_panel.history[1].sender, "AI");
        assert!(chat_panel.history[1].content.contains("[Simulated AI Response]"));
    }

    #[test]
    fn test_model_selection() {
        let mut chat_panel = setup_chat_panel();
        let model_to_select = "claude-2".to_string();
        chat_panel.select_model(model_to_select.clone());
        assert_eq!(chat_panel.selected_model, Some(model_to_select));

        chat_panel.select_model("non-existent-model".to_string());
        assert_eq!(chat_panel.selected_model, Some(model_to_select)); // Should not change
    }

     #[test]
    fn test_update_available_models_and_selection() {
        let mut chat_panel = setup_chat_panel();
        chat_panel.select_model("claude-2".to_string());

        let new_models = vec!["gpt-4".to_string(), "local-llama".to_string()];
        chat_panel.update_available_models(new_models.clone());

        assert_eq!(chat_panel.available_models, new_models);
        // Selected model ("claude-2") is no longer in available_models, so it should be reset
        assert_eq!(chat_panel.selected_model, None);

        // Select a new model from the updated list
        chat_panel.select_model("gpt-4".to_string());
        assert_eq!(chat_panel.selected_model, Some("gpt-4".to_string()));
    }

    #[test]
    fn test_send_message_with_help_command() {
        let mut chat_panel = setup_chat_panel();
        chat_panel.send_message("/help".to_string());

        assert_eq!(chat_panel.history.len(), 2); // User message + System help message
        assert_eq!(chat_panel.history[0].sender, "User");
        assert_eq!(chat_panel.history[0].content, "/help");
        assert_eq!(chat_panel.history[1].sender, "System");
        assert!(chat_panel.history[1].content.contains("Available commands:"));
        // Ensure no AI response was added (i.e., it didn't go to service_handler)
        // This is implicitly checked by history.len() == 2, as an AI response would make it 3.
    }

    #[test]
    fn test_send_message_with_generate_command() {
        // This test relies on AIServiceHandler and AICommands correctly interpreting
        // the command and AIRequest having the right capability.
        // The actual check of whether CodeGeneration capability was used by AIService
        // is harder here without deeper mocking of AIServiceHandler or AIService.
        // We are primarily testing that AIChatPanel correctly uses its components.
        let mut chat_panel = setup_chat_panel();
        let command_text = "/generate a rust function".to_string();
        chat_panel.send_message(command_text.clone());

        assert_eq!(chat_panel.history.len(), 2);
        assert_eq!(chat_panel.history[0].sender, "User");
        assert_eq!(chat_panel.history[0].content, command_text);
        assert_eq!(chat_panel.history[1].sender, "AI");
        // The response content will include the prompt, which was the full command.
        assert!(chat_panel.history[1].content.contains(&command_text));
        // To verify capability, we'd need to inspect the AIRequest passed to service_handler.
        // This requires a more complex setup (e.g. service_handler mock).
        // For now, we trust AICommands and the wiring. The demo helps verify this flow too.
        // We can check that the output prompt includes the model name that supports CodeGeneration
        // (assuming 'gpt-3.5-turbo' supports it by default in AIService::new())
        assert!(chat_panel.history[1].content.contains("gpt-3.5-turbo"));
    }
     #[test]
    fn test_add_message_internal_logic() {
        let mut chat_panel = setup_chat_panel();
        // Clear history for a clean test of add_message
        chat_panel.history.clear();

        chat_panel.add_message("TestSender".to_string(), "Test content".to_string(), false);
        assert_eq!(chat_panel.history.len(), 1);
        assert_eq!(chat_panel.history[0].sender, "TestSender");
        assert_eq!(chat_panel.history[0].content, "Test content");
        assert!(!chat_panel.history[0].is_code_snippet);

        chat_panel.add_message("CodeSender".to_string(), "<code>".to_string(), true);
        assert_eq!(chat_panel.history.len(), 2);
        assert!(chat_panel.history[1].is_code_snippet);
    }

    // Test for handle_code_snippet (though it currently just calls add_message)
    #[test]
    fn test_handle_code_snippet_adds_message() {
        let mut chat_panel = setup_chat_panel();
        chat_panel.history.clear(); // Start clean

        let code_content = "let x = 10;".to_string();
        chat_panel.handle_code_snippet(code_content.clone());

        assert_eq!(chat_panel.history.len(), 1);
        assert_eq!(chat_panel.history[0].sender, "AI"); // Default sender for handle_code_snippet
        assert_eq!(chat_panel.history[0].content, code_content);
        assert!(chat_panel.history[0].is_code_snippet);
    }
}
