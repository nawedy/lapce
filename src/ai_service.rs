use std::collections::HashMap;

// Define enums for AI providers and capabilities
#[derive(Debug, Clone, PartialEq)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Local,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AICapability {
    Chat,
    CodeGeneration,
    Debugging,
}

// Define a struct for AI model configuration
#[derive(Debug, Clone)]
pub struct AIModelConfig {
    pub name: String, // Made public for cloning/reconstruction in demo
    pub provider: AIProvider, // Made public
    pub capabilities: Vec<AICapability>, // Made public
    // Add other configuration fields as needed
}

// The core service that manages AI models, providers, and capabilities
pub struct AIService {
    pub models: HashMap<String, AIModelConfig>, // Made public for ai_models.rs tests, consider accessor methods
    default_provider: AIProvider,
}

impl AIService {
    // Create a new AIService with default configurations
    pub fn new() -> Self {
        let mut models = HashMap::new();
        // Add some default model configurations
        models.insert(
            "gpt-3.5-turbo".to_string(),
            AIModelConfig {
                name: "gpt-3.5-turbo".to_string(),
                provider: AIProvider::OpenAI,
                capabilities: vec![AICapability::Chat, AICapability::CodeGeneration],
            },
        );
        models.insert(
            "claude-2".to_string(),
            AIModelConfig {
                name: "claude-2".to_string(),
                provider: AIProvider::Anthropic,
                capabilities: vec![AICapability::Chat, AICapability::CodeGeneration],
            },
        );
        Self {
            models,
            default_provider: AIProvider::OpenAI, // Set a default provider
        }
    }

    // Register a new AI model
    pub fn register_model(&mut self, config: AIModelConfig) {
        self.models.insert(config.name.clone(), config);
    }

    // Configure an existing AI model
    pub fn configure_model(&mut self, model_name: &str, new_config: AIModelConfig) {
        if self.models.contains_key(model_name) {
            self.models.insert(model_name.to_string(), new_config);
        }
        // Optionally, handle the case where the model doesn't exist
    }

    // Select an AI model based on capability
    pub fn select_model_for_capability(&self, capability: AICapability) -> Option<&AIModelConfig> {
        // Simple selection strategy: find the first model that supports the capability
        self.models.values().find(|config| {
            config.capabilities.iter().any(|cap| {
                // Compare capabilities - assuming direct comparison works for enums
                std::mem::discriminant(cap) == std::mem::discriminant(&capability)
            })
        })
    }

    // Set the default AI provider
    pub fn set_default_provider(&mut self, provider: AIProvider) {
        self.default_provider = provider;
    }

    // Get the default AI provider
    pub fn get_default_provider(&self) -> &AIProvider {
        &self.default_provider
    }

    // Add more methods as needed for provider integration and capability handling
}

// Example of how to use AIService
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_service_new() {
        let ai_service = AIService::new();
        assert!(!ai_service.models.is_empty());
        // Add more assertions to check default configurations
    }

    #[test]
    fn test_register_and_select_model() {
        let mut ai_service = AIService::new();
        let new_model_config = AIModelConfig {
            name: "test-model".to_string(),
            provider: AIProvider::Local,
            capabilities: vec![AICapability::Debugging],
        };
        ai_service.register_model(new_model_config);

        let selected_model = ai_service.select_model_for_capability(AICapability::Debugging);
        assert!(selected_model.is_some());
        assert_eq!(selected_model.unwrap().name, "test-model");
    }

    #[test]
    fn test_configure_model() {
        let mut ai_service = AIService::new();
        let model_name = "gpt-3.5-turbo"; // Exists by default

        // Ensure it exists and has some initial capability
        let initial_model = ai_service.models.get(model_name).unwrap();
        assert!(initial_model.capabilities.contains(&AICapability::Chat));

        let new_capabilities = vec![AICapability::Debugging];
        let configured_model_config = AIModelConfig {
            name: model_name.to_string(),
            provider: AIProvider::OpenAI, // Keep provider same or change
            capabilities: new_capabilities.clone(),
        };
        ai_service.configure_model(model_name, configured_model_config);

        let configured_model = ai_service.models.get(model_name).unwrap();
        assert_eq!(configured_model.capabilities, new_capabilities);
        assert!(!configured_model.capabilities.contains(&AICapability::Chat)); // Original cap gone

        // Test configuring a non-existent model (should do nothing)
        let non_existent_name = "non-existent-model";
        let original_num_models = ai_service.models.len();
        let non_existent_config = AIModelConfig {
            name: non_existent_name.to_string(),
            provider: AIProvider::Local,
            capabilities: vec![],
        };
        ai_service.configure_model(non_existent_name, non_existent_config);
        assert_eq!(ai_service.models.len(), original_num_models); // No new model added
        assert!(ai_service.models.get(non_existent_name).is_none());
    }

    #[test]
    fn test_default_provider() {
        let mut ai_service = AIService::new();
        assert_eq!(*ai_service.get_default_provider(), AIProvider::OpenAI); // Default

        ai_service.set_default_provider(AIProvider::Anthropic);
        assert_eq!(*ai_service.get_default_provider(), AIProvider::Anthropic);

        ai_service.set_default_provider(AIProvider::Local);
        assert_eq!(*ai_service.get_default_provider(), AIProvider::Local);
    }

    #[test]
    fn test_select_model_no_capability_match() {
        let mut ai_service = AIService::new();
        // Clear existing models to ensure no accidental match
        ai_service.models.clear();
        // Add a model with only Chat capability
        ai_service.register_model(AIModelConfig {
            name: "chat-only-model".to_string(),
            provider: AIProvider::Local,
            capabilities: vec![AICapability::Chat],
        });

        // Try to select for Debugging capability
        let selected_model = ai_service.select_model_for_capability(AICapability::Debugging);
        assert!(selected_model.is_none());
    }
}
