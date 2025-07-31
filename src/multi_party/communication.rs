use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::{Duration, timeout};
use crate::multi_party::protocol::ProtocolError;
use crate::schema::{DataPoint, Query, QueryResult};

/// Types of messages that can be sent between servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Initialize connection
    Init,
    /// Share data
    Share,
    /// Shuffle operation
    Shuffle,
    /// Reconstruct data
    Reconstruct,
    /// Query request
    Query,
    /// Query response
    QueryResponse,
    /// Heartbeat
    Heartbeat,
    /// Error message
    Error(String),
    /// Acknowledge message
    Ack,
}

/// Network message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    /// Message type
    pub message_type: MessageType,
    /// Source server ID
    pub source_id: usize,
    /// Target server ID
    pub target_id: usize,
    /// Message sequence number
    pub sequence: u64,
    /// Message payload
    pub payload: MessagePayload,
    /// Timestamp
    pub timestamp: u64,
}

/// Message payload types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Empty payload
    Empty,
    /// Data payload
    Data(Vec<DataPoint>),
    /// Share payload
    Shares(Vec<Vec<u8>>),
    /// Query payload
    Query(Query),
    /// Query result payload
    QueryResult(QueryResult),
    /// Permutation payload
    Permutation(Vec<usize>),
    /// Error payload
    Error(String),
    /// Heartbeat payload
    Heartbeat,
}

impl NetworkMessage {
    /// Create a new network message
    pub fn new(
        message_type: MessageType,
        source_id: usize,
        target_id: usize,
        sequence: u64,
        payload: MessagePayload,
    ) -> Self {
        Self {
            message_type,
            source_id,
            target_id,
            sequence,
            payload,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Create an init message
    pub fn init(source_id: usize, target_id: usize, sequence: u64) -> Self {
        Self::new(MessageType::Init, source_id, target_id, sequence, MessagePayload::Empty)
    }

    /// Create a share message
    pub fn share(source_id: usize, target_id: usize, sequence: u64, shares: Vec<Vec<u8>>) -> Self {
        Self::new(MessageType::Share, source_id, target_id, sequence, MessagePayload::Shares(shares))
    }

    /// Create a shuffle message
    pub fn shuffle(source_id: usize, target_id: usize, sequence: u64, permutation: Vec<usize>) -> Self {
        Self::new(MessageType::Shuffle, source_id, target_id, sequence, MessagePayload::Permutation(permutation))
    }

    /// Create a query message
    pub fn query(source_id: usize, target_id: usize, sequence: u64, query: Query) -> Self {
        Self::new(MessageType::Query, source_id, target_id, sequence, MessagePayload::Query(query))
    }

    /// Create a query response message
    pub fn query_response(source_id: usize, target_id: usize, sequence: u64, result: QueryResult) -> Self {
        Self::new(MessageType::QueryResponse, source_id, target_id, sequence, MessagePayload::QueryResult(result))
    }

    /// Create a heartbeat message
    pub fn heartbeat(source_id: usize, target_id: usize, sequence: u64) -> Self {
        Self::new(MessageType::Heartbeat, source_id, target_id, sequence, MessagePayload::Heartbeat)
    }

    /// Create an error message
    pub fn error(source_id: usize, target_id: usize, sequence: u64, error: String) -> Self {
        Self::new(MessageType::Error(error), source_id, target_id, sequence, MessagePayload::Error(error))
    }

    /// Create an acknowledgment message
    pub fn ack(source_id: usize, target_id: usize, sequence: u64) -> Self {
        Self::new(MessageType::Ack, source_id, target_id, sequence, MessagePayload::Empty)
    }

    /// Check if message is expired
    pub fn is_expired(&self, max_age_seconds: u64) -> bool {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        current_time - self.timestamp > max_age_seconds
    }

    /// Get message age in seconds
    pub fn age_seconds(&self) -> u64 {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        current_time - self.timestamp
    }
}

/// Communication channel between servers
pub struct CommunicationChannel {
    /// Target server ID
    pub target_id: usize,
    /// Message sender
    pub sender: Sender<NetworkMessage>,
    /// Message receiver
    pub receiver: Receiver<NetworkMessage>,
    /// Connection state
    pub connected: bool,
    /// Last heartbeat time
    pub last_heartbeat: u64,
    /// Message sequence counter
    pub sequence_counter: u64,
}

impl CommunicationChannel {
    /// Create a new communication channel
    pub fn new(target_id: usize, sender: Sender<NetworkMessage>, receiver: Receiver<NetworkMessage>) -> Self {
        Self {
            target_id,
            sender,
            receiver,
            connected: false,
            last_heartbeat: 0,
            sequence_counter: 0,
        }
    }

    /// Send a message through the channel
    pub async fn send(&self, message: NetworkMessage) -> Result<(), ProtocolError> {
        if !self.connected {
            return Err(ProtocolError::network_error("Channel not connected".to_string()));
        }

        self.sender.send(message).await
            .map_err(|e| ProtocolError::network_error(format!("Failed to send message: {}", e)))?;

        Ok(())
    }

    /// Receive a message from the channel
    pub async fn receive(&mut self) -> Result<Option<NetworkMessage>, ProtocolError> {
        if !self.connected {
            return Err(ProtocolError::network_error("Channel not connected".to_string()));
        }

        self.receiver.recv().await
            .ok_or_else(|| ProtocolError::network_error("Channel closed".to_string()))
    }

    /// Receive a message with timeout
    pub async fn receive_timeout(&mut self, timeout_duration: Duration) -> Result<Option<NetworkMessage>, ProtocolError> {
        if !self.connected {
            return Err(ProtocolError::network_error("Channel not connected".to_string()));
        }

        match timeout(timeout_duration, self.receiver.recv()).await {
            Ok(Some(message)) => Ok(Some(message)),
            Ok(None) => Err(ProtocolError::network_error("Channel closed".to_string())),
            Err(_) => Err(ProtocolError::timeout(timeout_duration.as_millis() as u64)),
        }
    }

    /// Establish connection
    pub async fn connect(&mut self) -> Result<(), ProtocolError> {
        self.connected = true;
        self.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(())
    }

    /// Disconnect
    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    /// Send heartbeat
    pub async fn send_heartbeat(&mut self, source_id: usize) -> Result<(), ProtocolError> {
        let message = NetworkMessage::heartbeat(source_id, self.target_id, self.sequence_counter);
        self.sequence_counter += 1;
        self.send(message).await
    }

    /// Check if channel is healthy
    pub fn is_healthy(&self, max_heartbeat_age: u64) -> bool {
        if !self.connected {
            return false;
        }

        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        current_time - self.last_heartbeat <= max_heartbeat_age
    }

    /// Update heartbeat time
    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Get next sequence number
    pub fn next_sequence(&mut self) -> u64 {
        let sequence = self.sequence_counter;
        self.sequence_counter += 1;
        sequence
    }
}

/// Network manager for coordinating communication between servers
pub struct NetworkManager {
    /// Server ID
    pub server_id: usize,
    /// Communication channels to other servers
    pub channels: HashMap<usize, CommunicationChannel>,
    /// Message handlers
    pub handlers: HashMap<MessageType, Box<dyn MessageHandler + Send + Sync>>,
    /// Network configuration
    pub config: NetworkConfig,
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    /// Maximum heartbeat age in seconds
    pub max_heartbeat_age: u64,
    /// Message timeout in milliseconds
    pub message_timeout_ms: u64,
    /// Maximum retries for failed messages
    pub max_retries: usize,
    /// Whether to enable message encryption
    pub enable_encryption: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: 30,
            max_heartbeat_age: 90,
            message_timeout_ms: 5000,
            max_retries: 3,
            enable_encryption: true,
        }
    }
}

/// Message handler trait
pub trait MessageHandler {
    /// Handle incoming message
    fn handle(&self, message: &NetworkMessage) -> Result<(), ProtocolError>;
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(server_id: usize, config: NetworkConfig) -> Self {
        Self {
            server_id,
            channels: HashMap::new(),
            handlers: HashMap::new(),
            config,
        }
    }

    /// Add communication channel
    pub fn add_channel(&mut self, target_id: usize, channel: CommunicationChannel) {
        self.channels.insert(target_id, channel);
    }

    /// Register message handler
    pub fn register_handler(&mut self, message_type: MessageType, handler: Box<dyn MessageHandler + Send + Sync>) {
        self.handlers.insert(message_type, handler);
    }

    /// Send message to target server
    pub async fn send_message(&self, target_id: usize, message: NetworkMessage) -> Result<(), ProtocolError> {
        if let Some(channel) = self.channels.get(&target_id) {
            channel.send(message).await
        } else {
            Err(ProtocolError::network_error(format!("No channel to server {}", target_id)))
        }
    }

    /// Broadcast message to all servers
    pub async fn broadcast(&self, message: NetworkMessage) -> Result<(), ProtocolError> {
        let mut errors = Vec::new();

        for (target_id, channel) in &self.channels {
            if *target_id != self.server_id {
                if let Err(e) = channel.send(message.clone()).await {
                    errors.push(format!("Failed to send to server {}: {}", target_id, e));
                }
            }
        }

        if !errors.is_empty() {
            return Err(ProtocolError::network_error(format!("Broadcast errors: {:?}", errors)));
        }

        Ok(())
    }

    /// Start network manager
    pub async fn start(&mut self) -> Result<(), ProtocolError> {
        // Establish connections with all servers
        for (target_id, channel) in &mut self.channels {
            channel.connect().await?;
            log::info!("Connected to server {}", target_id);
        }

        // Start heartbeat loop
        self.start_heartbeat_loop().await?;

        Ok(())
    }

    /// Start heartbeat loop
    async fn start_heartbeat_loop(&mut self) -> Result<(), ProtocolError> {
        let heartbeat_interval = Duration::from_secs(self.config.heartbeat_interval);

        loop {
            tokio::time::sleep(heartbeat_interval).await;

            // Send heartbeats to all servers
            for (target_id, channel) in &mut self.channels {
                if let Err(e) = channel.send_heartbeat(self.server_id).await {
                    log::warn!("Failed to send heartbeat to server {}: {}", target_id, e);
                }
            }

            // Check health of all channels
            for (target_id, channel) in &self.channels {
                if !channel.is_healthy(self.config.max_heartbeat_age) {
                    log::warn!("Channel to server {} is unhealthy", target_id);
                }
            }
        }
    }

    /// Process incoming messages
    pub async fn process_messages(&self) -> Result<(), ProtocolError> {
        for (target_id, channel) in &mut self.channels {
            while let Ok(Some(message)) = channel.receive_timeout(
                Duration::from_millis(self.config.message_timeout_ms)
            ).await {
                // Handle message based on type
                if let Some(handler) = self.handlers.get(&message.message_type) {
                    if let Err(e) = handler.handle(&message) {
                        log::error!("Failed to handle message: {}", e);
                    }
                } else {
                    log::warn!("No handler for message type: {:?}", message.message_type);
                }
            }
        }

        Ok(())
    }

    /// Get channel to specific server
    pub fn get_channel(&self, target_id: usize) -> Option<&CommunicationChannel> {
        self.channels.get(&target_id)
    }

    /// Get mutable channel to specific server
    pub fn get_channel_mut(&mut self, target_id: usize) -> Option<&mut CommunicationChannel> {
        self.channels.get_mut(&target_id)
    }

    /// Check if all channels are healthy
    pub fn all_channels_healthy(&self) -> bool {
        self.channels.values().all(|channel| channel.is_healthy(self.config.max_heartbeat_age))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::QueryType;

    #[test]
    fn test_network_message_creation() {
        let message = NetworkMessage::init(0, 1, 1);
        assert_eq!(message.source_id, 0);
        assert_eq!(message.target_id, 1);
        assert_eq!(message.sequence, 1);
        assert!(matches!(message.message_type, MessageType::Init));
    }

    #[test]
    fn test_network_message_expiration() {
        let message = NetworkMessage::init(0, 1, 1);
        assert!(!message.is_expired(60)); // Should not be expired after 1 second
    }

    #[tokio::test]
    async fn test_communication_channel() {
        let (tx, rx) = mpsc::channel(10);
        let mut channel = CommunicationChannel::new(1, tx, rx);
        
        channel.connect().await.unwrap();
        assert!(channel.connected);
    }

    #[tokio::test]
    async fn test_network_manager() {
        let config = NetworkConfig::default();
        let manager = NetworkManager::new(0, config);
        assert_eq!(manager.server_id, 0);
        assert!(manager.channels.is_empty());
    }
} 