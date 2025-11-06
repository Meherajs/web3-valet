//! AI agent definitions and management.
//!
//! This module provides the configuration and metadata for all available AI agents.
//! Each agent has a unique ID, capabilities, and system prompt that defines its behavior.

use crate::models::Agent;

/// Returns the list of all available AI agents.
///
/// Each agent has a unique ID, name, description, capabilities, and system prompt.
/// The system prompt defines the agent's behavior and expertise area.
///
/// # Available Agents
///
/// - `agent_001` - General Assistant (general-purpose)
/// - `agent_002` - Web3 Expert (blockchain, crypto, DeFi)
/// - `agent_003` - Voice Specialist (conversational, voice-optimized)
/// - `agent_004` - Code Assistant (programming, debugging)
///
/// # Returns
///
/// A vector of `Agent` structs representing all available agents.
///
/// # Example
///
/// ```rust
/// let agents = get_agents();
/// for agent in agents {
///     println!("{}: {}", agent.id, agent.name);
/// }
/// ```
pub fn get_agents() -> Vec<Agent> {
    vec![
        Agent {
            id: "agent_001".to_string(),
            name: "General Assistant".to_string(),
            description: "A helpful general-purpose AI assistant powered by Gemini".to_string(),
            capabilities: vec![
                "text".to_string(),
                "conversation".to_string(),
                "reasoning".to_string(),
            ],
            model: "gemini-2.0-flash-exp".to_string(),
            system_prompt: "You are a helpful, friendly, and knowledgeable AI assistant. Provide clear, accurate, and concise responses.".to_string(),
        },
        Agent {
            id: "agent_002".to_string(),
            name: "Web3 Expert".to_string(),
            description: "Specialized in blockchain, Web3, and cryptocurrency technologies".to_string(),
            capabilities: vec![
                "web3".to_string(),
                "crypto".to_string(),
                "blockchain".to_string(),
                "nft".to_string(),
            ],
            model: "gemini-2.0-flash-exp".to_string(),
            system_prompt: "You are a Web3 and blockchain expert. Help users understand cryptocurrency, NFTs, smart contracts, DeFi, and related technologies. Provide accurate technical information and practical guidance.".to_string(),
        },
        Agent {
            id: "agent_003".to_string(),
            name: "Voice Specialist".to_string(),
            description: "Optimized for natural voice conversations and audio interactions".to_string(),
            capabilities: vec![
                "voice".to_string(),
                "audio".to_string(),
                "conversation".to_string(),
            ],
            model: "gemini-2.0-flash-exp".to_string(),
            system_prompt: "You are an AI assistant optimized for voice interactions. Respond in a natural, conversational tone suitable for speech. Keep responses concise and easy to understand when spoken aloud.".to_string(),
        },
        Agent {
            id: "agent_004".to_string(),
            name: "Code Assistant".to_string(),
            description: "Expert in programming, software development, and technical problem-solving".to_string(),
            capabilities: vec![
                "coding".to_string(),
                "debugging".to_string(),
                "technical".to_string(),
            ],
            model: "gemini-2.0-flash-exp".to_string(),
            system_prompt: "You are an expert programming assistant. Help users with code, debugging, architecture, and technical decisions. Provide clear explanations and working code examples.".to_string(),
        },
    ]
}

/// Finds an agent by ID.
///
/// # Arguments
///
/// * `agent_id` - The unique identifier of the agent to find
///
/// # Returns
///
/// `Some(Agent)` if found, `None` otherwise
///
/// # Example
///
/// ```rust
/// if let Some(agent) = find_agent_by_id("agent_002") {
///     println!("Found agent: {}", agent.name);
/// }
/// ```
pub fn find_agent_by_id(agent_id: &str) -> Option<Agent> {
    get_agents().into_iter().find(|a| a.id == agent_id)
}
