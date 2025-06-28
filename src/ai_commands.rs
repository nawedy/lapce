use std::collections::HashMap;
use crate::ai_service::AICapability; // To map commands to capabilities

// Enum to represent different command types
#[derive(Debug, PartialEq, Clone)]
pub enum CommandType {
    Generate,
    Explain,
    Debug,
    Help,
    Unknown,
    // Add more specific commands as needed
}

// Struct to hold parsed command information
#[derive(Debug)]
pub struct ParsedCommand {
    pub command_type: CommandType,
    pub arguments: Vec<String>,
    pub full_input: String,
    pub capability: Option<AICapability>, // Capability associated with the command
}

// Processes natural language commands for the AI chat panel
pub struct AICommands {
    // For NLP, one might integrate a library or a more sophisticated parser here.
    // For now, we'll use simple prefix and keyword matching.
    command_map: HashMap<String, CommandType>,
}

impl AICommands {
    // Create a new AICommands processor
    pub fn new() -> Self {
        let mut command_map = HashMap::new();
        command_map.insert("/generate".to_string(), CommandType::Generate);
        command_map.insert("/explain".to_string(), CommandType::Explain);
        command_map.insert("/debug".to_string(), CommandType::Debug);
        command_map.insert("/help".to_string(), CommandType::Help);
        // Add natural language keywords if desired, e.g., "generate code" -> CommandType::Generate
        Self { command_map }
    }

    // Parse user input to identify commands
    pub fn parse_input(&self, input: &str) -> ParsedCommand {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return ParsedCommand {
                command_type: CommandType::Unknown, // Or perhaps a default like 'Chat'
                arguments: Vec::new(),
                full_input: input.to_string(),
                capability: Some(AICapability::Chat), // Default to chat if no command
            };
        }

        let command_keyword = parts[0].to_lowercase();
        let arguments = parts[1..].iter().map(|s| s.to_string()).collect();

        if let Some(cmd_type) = self.command_map.get(&command_keyword) {
            let capability = match cmd_type {
                CommandType::Generate => Some(AICapability::CodeGeneration),
                CommandType::Explain => Some(AICapability::Chat), // Or a specific 'Explanation' capability
                CommandType::Debug => Some(AICapability::Debugging),
                _ => None, // Help, Unknown might not map directly
            };
            ParsedCommand {
                command_type: cmd_type.clone(),
                arguments,
                full_input: input.to_string(),
                capability,
            }
        } else {
            // Basic NLP: check if the input *starts* with a known command verb
            // This is very rudimentary. Real NLP would be more complex.
            if command_keyword == "generate" || command_keyword == "create" {
                 return ParsedCommand { command_type: CommandType::Generate, arguments, full_input: input.to_string(), capability: Some(AICapability::CodeGeneration) };
            }
            if command_keyword == "explain" || command_keyword == "what" {
                 return ParsedCommand { command_type: CommandType::Explain, arguments, full_input: input.to_string(), capability: Some(AICapability::Chat) };
            }
             if command_keyword == "debug" || command_keyword == "fix" {
                 return ParsedCommand { command_type: CommandType::Debug, arguments, full_input: input.to_string(), capability: Some(AICapability::Debugging) };
            }

            // If no prefix or keyword matches, treat as a general chat message
            ParsedCommand {
                command_type: CommandType::Unknown, // Or a specific 'Chat' type
                arguments: parts.iter().map(|s| s.to_string()).collect(), // The whole input becomes "arguments" for a general chat
                full_input: input.to_string(),
                capability: Some(AICapability::Chat), // Default to chat
            }
        }
    }

    // Provide help information for available commands
    pub fn get_help(&self) -> String {
        let mut help_text = "Available commands:\n".to_string();
        for (cmd_prefix, cmd_type) in &self.command_map {
            help_text.push_str(&format!("  {} - {:?}\n", cmd_prefix, cmd_type));
        }
        help_text.push_str("\nYou can also try phrasing your requests in natural language, e.g., 'generate a python function to sort a list'.");
        help_text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_prefix_commands() {
        let processor = AICommands::new();
        let parsed = processor.parse_input("/generate python function");
        assert_eq!(parsed.command_type, CommandType::Generate);
        assert_eq!(parsed.arguments, vec!["python".to_string(), "function".to_string()]);
        assert_eq!(parsed.capability.unwrap(), AICapability::CodeGeneration);


        let parsed_explain = processor.parse_input("/explain this code");
        assert_eq!(parsed_explain.command_type, CommandType::Explain);
        assert_eq!(parsed_explain.capability.unwrap(), AICapability::Chat);
    }

    #[test]
    fn test_parse_natural_language_commands() {
        let processor = AICommands::new();
        // Rudimentary NLP test
        let parsed_gen = processor.parse_input("generate a rust struct");
        assert_eq!(parsed_gen.command_type, CommandType::Generate);
        assert_eq!(parsed_gen.arguments, vec!["a".to_string(), "rust".to_string(), "struct".to_string()]);
        assert_eq!(parsed_gen.capability.unwrap(), AICapability::CodeGeneration);

        let parsed_explain = processor.parse_input("explain what this function does");
         assert_eq!(parsed_explain.command_type, CommandType::Explain);
    }

    #[test]
    fn test_parse_unknown_command_as_chat() {
        let processor = AICommands::new();
        let parsed = processor.parse_input("Hello there, how are you?");
        assert_eq!(parsed.command_type, CommandType::Unknown); // Or a dedicated Chat type
        assert!(parsed.arguments.join(" ").contains("Hello there"));
        assert_eq!(parsed.capability.unwrap(), AICapability::Chat);
    }

    #[test]
    fn test_get_help() {
        let processor = AICommands::new();
        let help_text = processor.get_help();
        assert!(help_text.contains("/generate"));
        assert!(help_text.contains("/explain"));
        assert!(help_text.contains("natural language"));
    }

    #[test]
    fn test_parse_empty_input() {
        let processor = AICommands::new();
        let parsed = processor.parse_input("");
        assert_eq!(parsed.command_type, CommandType::Unknown);
        assert!(parsed.arguments.is_empty());
        assert_eq!(parsed.capability.unwrap(), AICapability::Chat); // Default to chat
    }

     #[test]
    fn test_parse_debug_command() {
        let processor = AICommands::new();
        let parsed = processor.parse_input("/debug this rust code");
        assert_eq!(parsed.command_type, CommandType::Debug);
        assert_eq!(parsed.arguments, vec!["this".to_string(), "rust".to_string(), "code".to_string()]);
        assert_eq!(parsed.capability.unwrap(), AICapability::Debugging);

        let parsed_nlp = processor.parse_input("fix this error in my code");
        assert_eq!(parsed_nlp.command_type, CommandType::Debug);
        assert_eq!(parsed_nlp.capability.unwrap(), AICapability::Debugging);
    }
}

// --- IDE Command Integration Points ---

// Define unique IDs for commands that can be triggered from outside the chat panel
// (e.g., command palette, keyboard shortcuts)
pub mod ide_commands {
    pub const EXPLAIN_CODE: &str = "lapce.ai_explain_code";
    pub const GENERATE_CODE_FROM_SELECTION: &str = "lapce.ai_generate_code_from_selection";
    pub const DEBUG_CODE_SELECTION: &str = "lapce.ai_debug_code_selection";
    pub const SHOW_AI_CHAT_PANEL: &str = "lapce.ai_show_chat_panel";
    // Add more as new global commands are identified
}

// Example of how these commands might be handled.
// In a real Lapce plugin, you would register these with Lapce's command system.
// The handler functions would typically interact with the AIChatPanel or AIServiceHandler.
/*
fn register_ide_commands(chat_panel: &mut AIChatPanel, service_handler: &mut AIServiceHandler) {
    // Example for EXPLAIN_CODE
    register_command(ide_commands::EXPLAIN_CODE, "AI: Explain Selected Code", |ide_context| {
        let selection = ide_context.get_selected_text();
        if !selection.is_empty() {
            // Option 1: Send directly to service_handler
            let request = AIRequest {
                capability: AICapability::Chat, // Or a more specific "Explain" capability
                prompt: format!("Explain this code: {}", selection),
                // Gather context: active file, project, etc.
            };
            let response = service_handler.process_request(request);
            ide_context.show_in_notification_or_panel(response.content);

            // Option 2: Programmatically send to chat_panel
            // chat_panel.send_message(format!("/explain {}", selection));
            // This might be better if we want the interaction in chat history.
        }
    });

    // Example for GENERATE_CODE_FROM_SELECTION (e.g. "implement this function based on its signature")
    register_command(ide_commands::GENERATE_CODE_FROM_SELECTION, "AI: Generate Code from Selection", |ide_context| {
        let selection = ide_context.get_selected_text(); // e.g. a function signature or comment
        if !selection.is_empty() {
            // chat_panel.send_message(format!("/generate based on: {}", selection));
            // Or directly use service_handler with CodeGeneration capability
        }
    });

    // SHOW_AI_CHAT_PANEL would simply focus/toggle the AIChatPanel UI component.
}
*/

// The `AICommands` struct itself primarily deals with text parsing from the chat input.
// The IDE command integration would likely call methods on `AIChatPanel` or `AIServiceHandler` directly,
// or use a new layer that coordinates between IDE actions and the AI services.
