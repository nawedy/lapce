use crate::ai_service::{AIService, AIModelConfig, AIProvider, AICapability};
use std::collections::HashMap;

// Struct to hold settings for a provider (e.g., API key)
#[derive(Clone, Debug)]
pub struct ProviderSettings {
    api_key: Option<String>,
    // Add other provider-specific settings, e.g., endpoint URL for local models
}

// UI for managing AI models and providers
pub struct AIModelsPanel {
    ai_service: AIService, // To interact with the underlying service
    // UI state for this panel
    provider_settings: HashMap<String, ProviderSettings>, // Keyed by provider name (e.g., "OpenAI")
    // Model list and selected model for editing could be part of the UI state
    // For simplicity, we'll assume direct interaction with ai_service for model configs
}

impl AIModelsPanel {
    // Create a new AIModelsPanel
    pub fn new(ai_service: AIService) -> Self {
        let mut provider_settings = HashMap::new();
        // Initialize with known providers, though keys would ideally come from AIProvider enum
        provider_settings.insert("OpenAI".to_string(), ProviderSettings { api_key: None });
        provider_settings.insert("Anthropic".to_string(), ProviderSettings { api_key: None });
        provider_settings.insert("Local".to_string(), ProviderSettings { api_key: None }); // Local might not need an API key

        Self {
            ai_service,
            provider_settings,
        }
    }

    // --- Model Configuration ---
    // (Enable/disable models might mean adding/removing them or toggling an 'active' flag within AIModelConfig)

    // Add a new model configuration (simplified: assumes all details are provided)
    // In a real UI, this would involve forms to input model details.
    pub fn add_model_config(&mut self, config: AIModelConfig) {
        self.ai_service.register_model(config);
        // Notify other parts of the application (e.g., chat panel to update model list)
        self.on_models_changed();
    }

    // Remove a model configuration
    pub fn remove_model_config(&mut self, model_name: &str) {
        // AIService doesn't have a remove_model method yet, so this is a conceptual placeholder.
        // We'd need to add `remove_model` to `AIService` that takes `model_name`.
        // For now, let's simulate by re-registering a new empty set of models or similar.
        // self.ai_service.remove_model(model_name); // Ideal
        println!("Conceptual: Remove model {} (requires AIService.remove_model)", model_name);
        // To make it testable without modifying AIService now, let's assume models can be overwritten to "remove"
        // This is not a good long-term solution.
        let placeholder_config = AIModelConfig {
             name: model_name.to_string(),
             provider: AIProvider::Local, // Placeholder
             capabilities: vec![], // No capabilities effectively disables it
        };
        // This is more like disabling than removing if name collision happens:
        // self.ai_service.configure_model(model_name, placeholder_config);
        // A true remove would need AIService.models to be mutable here or a method.
        // For now, this function is largely a placeholder for the UI action.
        self.on_models_changed();
    }

    // Set a model as default for a specific capability (conceptual)
    // This logic might reside more within AIService or be a UI preference that influences selection.
    pub fn set_default_model_for_capability(&mut self, model_name: &str, capability: AICapability) {
        // This would require AIService to store default model preferences per capability.
        println!("Conceptual: Set {} as default for {:?}", model_name, capability);
    }

    // --- API Key Management ---
    pub fn set_api_key(&mut self, provider_name: &str, api_key: String) {
        if let Some(settings) = self.provider_settings.get_mut(provider_name) {
            settings.api_key = Some(api_key);
            println!("API Key set for {}", provider_name);
            // Potentially notify AIService or other components that use this key.
        } else {
            eprintln!("Provider {} not recognized for API key.", provider_name);
        }
    }

    pub fn get_api_key(&self, provider_name: &str) -> Option<String> {
        self.provider_settings.get(provider_name).and_then(|s| s.api_key.clone())
    }

    // --- Provider Settings ---
    // (Example: configure endpoint for a local provider)
    pub fn set_provider_endpoint(&mut self, provider_name: &str, endpoint_url: String) {
        // This would require ProviderSettings to have an endpoint field.
        // For now, just a placeholder.
        println!("Conceptual: Set endpoint for {} to {}", provider_name, endpoint_url);
    }


    // --- Capability Assignment ---
    // (Assigning models to specific capabilities is part of AIModelConfig)
    // This might involve editing an existing AIModelConfig.
    pub fn assign_capability_to_model(&mut self, model_name: &str, capability: AICapability) {
        // This requires getting the model config, modifying it, and re-registering or updating.
        // AIService.configure_model could be used if we fetch the existing config first.
        // This is a simplified view; a real UI would fetch, modify, then save.
        if let Some(mut existing_config) = self.ai_service.models.get(model_name).cloned() { // Requires pub access or method
            if !existing_config.capabilities.iter().any(|c| std::mem::discriminant(c) == std::mem::discriminant(&capability)) {
                existing_config.capabilities.push(capability);
                self.ai_service.configure_model(model_name, existing_config);
                self.on_models_changed();
                println!("Assigned capability to {}", model_name);
            }
        } else {
            eprintln!("Model {} not found for capability assignment.", model_name);
        }
    }

    // Helper to simulate notifying other parts of the application
    fn on_models_changed(&self) {
        // In a real app, this would emit an event or call a callback
        // that other components (like AIChatPanel) listen to.
        println!("Models configuration changed. UI should update.");
        // Example: ai_chat_panel.update_available_models(self.ai_service.get_model_names());
    }

    // Getter for AIService (e.g. for tests or other parts of UI)
    pub fn get_ai_service(&self) -> &AIService {
        &self.ai_service
    }
     // Mutable getter for AIService for tests that need to modify it directly
    pub fn get_ai_service_mut(&mut self) -> &mut AIService {
        &mut self.ai_service
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_models_panel() -> AIModelsPanel {
        AIModelsPanel::new(AIService::new()) // AIService::new() creates some default models
    }

    #[test]
    fn test_add_and_get_model_config() {
        let mut panel = setup_models_panel();
        let new_model_config = AIModelConfig {
            name: "test-local-model".to_string(),
            provider: AIProvider::Local,
            capabilities: vec![AICapability::Debugging],
        };
        panel.add_model_config(new_model_config);

        let service = panel.get_ai_service();
        assert!(service.models.contains_key("test-local-model"));
        let model = service.models.get("test-local-model").unwrap();
        assert!(matches!(model.provider, AIProvider::Local));
    }

    #[test]
    fn test_api_key_management() {
        let mut panel = setup_models_panel();
        let provider = "OpenAI";
        let key = "test_api_key_123".to_string();
        panel.set_api_key(provider, key.clone());
        assert_eq!(panel.get_api_key(provider), Some(key));

        assert_eq!(panel.get_api_key("NonExistentProvider"), None);
    }

    #[test]
    fn test_assign_capability_to_model() {
        let mut panel = setup_models_panel();
        // Add a model to ensure it exists for the test.
        // AIService::new() already adds "gpt-3.5-turbo"
        let model_name = "gpt-3.5-turbo";

        // Ensure the model exists from AIService::new()
        assert!(panel.get_ai_service().models.contains_key(model_name), "Default model gpt-3.5-turbo not found");

        // Assign a new capability (e.g., Debugging, assuming it's not there by default)
        // First, check current capabilities to ensure test is valid
        let initial_caps = panel.get_ai_service().models.get(model_name).unwrap().capabilities.len();

        panel.assign_capability_to_model(model_name, AICapability::Debugging);

        let model = panel.get_ai_service().models.get(model_name).unwrap();
        assert!(model.capabilities.iter().any(|c| matches!(c, AICapability::Debugging)));
        // Ensure it didn't just replace, but added (if not present)
        // This depends on the default capabilities of "gpt-3.5-turbo" in AIService::new()
        // If Debugging was already there, length wouldn't change.
        // Let's assume Debugging is NOT a default capability for gpt-3.5-turbo.
        // The default capabilities are Chat and CodeGeneration.
        assert_eq!(model.capabilities.len(), initial_caps + 1, "Debugging capability was not added or was already present.");

        // Test assigning to a non-existent model
        panel.assign_capability_to_model("non-existent-model", AICapability::Chat);
        // No panic, error message printed (eprintln). How to check this in test?
        // For now, just ensure it doesn't crash and the existing model is unchanged further.
        let model_after_failed_assign = panel.get_ai_service().models.get(model_name).unwrap();
        assert_eq!(model_after_failed_assign.capabilities.len(), initial_caps + 1);
    }

    // Note: Testing `remove_model_config` is tricky without modifying AIService
    // or having a way to inspect calls to it (mocks).
    // The current `remove_model_config` is mostly conceptual.
}
