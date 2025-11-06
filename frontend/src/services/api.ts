// API Service for MCP API Backend Communication

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000';

/**
 * Agent information structure
 */
export interface Agent {
  id: string;
  name: string;
  description: string;
}

/**
 * Response from text or audio input endpoints
 */
export interface AgentReplyResponse {
  reply_text: string;
  audio_url: string;
}

/**
 * Request payload for text input
 */
export interface TextInputRequest {
  agent_id: string;
  user_text: string;
}

/**
 * Check if the API server is healthy
 */
export async function checkHealth(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE_URL}/health`);
    return response.ok && (await response.text()) === 'OK';
  } catch (error) {
    console.error('Health check failed:', error);
    return false;
  }
}

/**
 * Fetch the list of available agents
 */
export async function getAgents(): Promise<Agent[]> {
  const response = await fetch(`${API_BASE_URL}/agents`);
  
  if (!response.ok) {
    throw new Error(`Failed to fetch agents: ${response.statusText}`);
  }
  
  return response.json();
}

/**
 * Send text input to a specific agent and get response
 */
export async function sendTextInput(
  agentId: string,
  userText: string
): Promise<AgentReplyResponse> {
  const payload: TextInputRequest = {
    agent_id: agentId,
    user_text: userText,
  };

  const response = await fetch(`${API_BASE_URL}/input/text`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to send text: ${response.statusText} - ${errorText}`);
  }

  return response.json();
}

/**
 * Send audio file to a specific agent and get response
 */
export async function sendAudioInput(
  agentId: string,
  audioFile: File
): Promise<AgentReplyResponse> {
  const formData = new FormData();
  formData.append('audio_file', audioFile);
  formData.append('agent_id', agentId);

  const response = await fetch(`${API_BASE_URL}/input/audio`, {
    method: 'POST',
    body: formData,
  });

  if (!response.ok) {
    const errorText = await response.text();
    throw new Error(`Failed to send audio: ${response.statusText} - ${errorText}`);
  }

  return response.json();
}

/**
 * Get the full URL for an audio file
 */
export function getAudioUrl(audioPath: string): string {
  // If it's already a full URL, return as is
  if (audioPath.startsWith('http://') || audioPath.startsWith('https://')) {
    return audioPath;
  }
  
  // If it starts with a slash, append to base URL
  if (audioPath.startsWith('/')) {
    return `${API_BASE_URL}${audioPath}`;
  }
  
  // Otherwise, append with a slash
  return `${API_BASE_URL}/${audioPath}`;
}
