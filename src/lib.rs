// Placeholder for lib.rs content
// This file will eventually contain module declarations and other top-level code.

pub mod ai_service;
pub mod ai_service_handler;
pub mod ai_chat;
pub mod ai_commands;
pub mod ai_models;

// Import necessary structs for the demo
use ai_service::{AIService, AIModelConfig, AIProvider, AICapability};
use ai_service_handler::AIServiceHandler;
use ai_chat::AIChatPanel;
use ai_commands::AICommands;
use ai_models::AIModelsPanel;

// Function to demonstrate component integration
pub fn run_ai_integration_demo() {
    println!("--- Running AI Integration Demo ---");

    // 1. Initialize AI Service Core
    let mut ai_service = AIService::new();
    // Add a custom model for the demo
    ai_service.register_model(AIModelConfig {
        name: "demo-custom-model".to_string(),
        provider: AIProvider::Local,
        capabilities: vec![AICapability::Chat, AICapability::CodeGeneration, AICapability::Debugging],
    });

    // 2. Initialize AI Service Handler
    // AIServiceHandler takes ownership of AIService, so we need to handle this.
    // For the demo, AIModelsPanel might also need access or a way to modify AIService.
    // Let's assume AIModelsPanel is created first and can provide a reference or modified AIService.
    // Or, AIService could be wrapped in Rc<RefCell<>> for shared mutable access if needed by multiple UI panels.
    // For this demo, let's keep it simpler: AIModelsPanel will hold the AIService.

    // Initialize AI Models Panel (which internally holds AIService)
    let mut models_panel = AIModelsPanel::new(ai_service); // models_panel now owns ai_service

    // Configure an API key via the models_panel
    models_panel.set_api_key("OpenAI", "DEMO_OPENAI_KEY_123".to_string());
    println!("OpenAI API Key set via ModelsPanel: {:?}", models_panel.get_api_key("OpenAI"));

    // Add another model via the models_panel
    models_panel.add_model_config(AIModelConfig {
        name: "panel-added-model".to_string(),
        provider: AIProvider::Anthropic,
        capabilities: vec![AICapability::Chat],
    });

    // Retrieve all model names from the service via the models_panel for the chat panel
    let all_model_names: Vec<String> = models_panel.get_ai_service().models.keys().cloned().collect();


    // 3. Initialize AI Commands Module
    let ai_commands = AICommands::new();

    // 4. Initialize AI Service Handler (it needs AIService)
    // This is tricky because models_panel owns ai_service.
    // A real application would use a shared ownership model (e.g. Rc<RefCell<AIService>>)
    // or pass messages between components.
    // For this demo, we will re-create an AIServiceHandler with a *new* AIService instance
    // that is similar to the one in models_panel. This is NOT ideal for a real app
    // but sufficient to demonstrate AIChatPanel using an AIServiceHandler.
    // A better approach would be for ModelsPanel to provide a way to get/construct handlers,
    // or for AIService to be shared.

    // Let's re-fetch models from the models_panel's service to make the new service handler more realistic
    let mut service_for_handler = AIService::new(); // Fresh service
    for model_config in models_panel.get_ai_service().models.values() {
        // Cloning config. In real app, AIModelConfig might not be clonable or might be complex.
        // For this demo, AIModelConfig is not Clone. We'll need to make it Clone or reconstruct.
        // Let's make AIModelConfig Clone.
        service_for_handler.register_model(AIModelConfig {
            name: model_config.name.clone(),
            provider: model_config.provider.clone(), // Requires AIProvider to be Clone
            capabilities: model_config.capabilities.clone(), // Requires AICapability to be Clone
        });
    }
    let mut service_handler = AIServiceHandler::new(service_for_handler);


    // 5. Initialize AI Chat Panel
    let mut chat_panel = AIChatPanel::new(service_handler, ai_commands, all_model_names.clone());
    println!("\n--- Chat Panel Initialized ---");
    println!("Available models in Chat Panel: {:?}", all_model_names);

    // Simulate user selecting a model in chat panel
    if !all_model_names.is_empty() {
        chat_panel.select_model(all_model_names[0].clone());
    }

    // Simulate user sending messages / commands
    println!("\n--- Simulating User Input ---");
    chat_panel.send_message("Hello AI, tell me a joke.".to_string());
    chat_panel.send_message("/generate a python function for fibonacci".to_string());
    chat_panel.send_message("Explain this code: `let x = 5;`".to_string()); // Will be parsed by AICommands
    chat_panel.send_message("/help".to_string());

    // Simulate using AIModelsPanel to change config and see if ChatPanel can be updated
    // (This requires a mechanism for ChatPanel to be aware of changes from AIModelsPanel)
    println!("\n--- Simulating Model Configuration Change via AIModelsPanel ---");
    models_panel.add_model_config(AIModelConfig {
        name: "newly-added-model-via-panel".to_string(),
        provider: AIProvider::Local,
        capabilities: vec![AICapability::Chat],
    });
    let updated_model_names: Vec<String> = models_panel.get_ai_service().models.keys().cloned().collect();

    // In a real app, AIModelsPanel.on_models_changed() would trigger an event
    // that AIChatPanel subscribes to. Here, we manually update it.
    chat_panel.update_available_models(updated_model_names.clone());
    println!("Chat Panel available models updated: {:?}", updated_model_names);
    chat_panel.select_model("newly-added-model-via-panel".to_string());


    // Simulate context gathering (very abstractly)
    // Context gathering is mostly within AIServiceHandler or AIChatPanel.
    // AIServiceHandler.gather_context is a placeholder.
    // AIChatPanel.gather_context_for_ui is also a placeholder.
    let ui_context = chat_panel.gather_context_for_ui();
    println!("\n--- Context Simulation ---");
    println!("Context gathered by ChatPanel: {}", ui_context);
    // To see AIServiceHandler's context gathering, we'd need to inspect a request.
    // The send_message in chat_panel already invokes service_handler.process_request which calls gather_context.
    // The output of that is internal to process_request in this setup.

    println!("\n--- AI Integration Demo Finished ---");
}

// To make the demo runnable and testable, we need AIModelConfig, AIProvider, and AICapability to be Clone.
// We'll need to edit ai_service.rs to add `#[derive(Clone)]` to these.
// AIModelConfig also contains String and Vec, which are Clone.
// The fields `name`, `provider`, `capabilities` inside `AIModelConfig` also need to be public
// or have getters if we are to reconstruct it as done above.
// For simplicity in the demo, let's make them public in ai_service.rs.
// (Alternatively, AIModelConfig could implement a proper clone method).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_demo() {
        // This test essentially runs the demo to ensure it doesn't panic.
        // Output can be manually inspected if `cargo test -- --nocapture` is run.
        run_ai_integration_demo();
        // Add assertions here if the demo function returned state or had side effects
        // that could be programmatically checked. For now, it's a print-based demo.
    }
}
