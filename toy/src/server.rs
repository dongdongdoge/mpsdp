use crate::finite_field::FieldElement;
use crate::secret_sharing::SecretShare;
use crate::ToyConfig;
use serde::{Deserialize, Serialize};

/// Server roles in the protocol
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerRole {
    /// P₀: Auxiliary server that generates correlated randomness
    Auxiliary,
    /// P₁, P₂: Computational servers that perform local computations
    Computational,
}

/// Server states
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerState {
    /// Server is offline
    Offline,
    /// Server is online and ready
    Online,
    /// Server is participating in protocol
    Participating,
    /// Server has completed its part
    Completed,
    /// Server has failed
    Failed(String),
}

impl ServerState {
    /// Check if server is available
    pub fn is_available(&self) -> bool {
        matches!(self, ServerState::Online | ServerState::Participating | ServerState::Completed)
    }

    /// Check if server has failed
    pub fn is_failed(&self) -> bool {
        matches!(self, ServerState::Failed(_))
    }
}

/// Server implementation
pub struct Server {
    /// Server ID
    pub id: usize,
    /// Server role
    pub role: ServerRole,
    /// Server state
    pub state: ServerState,
    /// Configuration
    pub config: ToyConfig,
    /// Permutation shares (for computational servers)
    pub permutation_shares: Vec<Vec<Vec<SecretShare>>>,
    /// Mask shares (for computational servers)
    pub mask_shares: Vec<Vec<Vec<SecretShare>>>,
    /// Noise shares (for computational servers)
    pub noise_shares: Vec<Vec<SecretShare>>,
    /// Final result (for computational servers)
    pub final_result: Option<Vec<Vec<FieldElement>>>,
}

impl Server {
    /// Create a new server
    pub fn new(id: usize, role: ServerRole, config: ToyConfig) -> Self {
        Self {
            id,
            role,
            state: ServerState::Offline,
            config,
            permutation_shares: Vec::new(),
            mask_shares: Vec::new(),
            noise_shares: Vec::new(),
            final_result: None,
        }
    }

    /// Initialize the server
    pub fn initialize(&mut self) {
        self.state = ServerState::Online;
    }

    /// Get server ID
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get server role
    pub fn role(&self) -> &ServerRole {
        &self.role
    }

    /// Get server state
    pub fn state(&self) -> &ServerState {
        &self.state
    }

    /// Set server state
    pub fn set_state(&mut self, state: ServerState) {
        self.state = state;
    }

    /// Check if server is auxiliary
    pub fn is_auxiliary(&self) -> bool {
        matches!(self.role, ServerRole::Auxiliary)
    }

    /// Check if server is computational
    pub fn is_computational(&self) -> bool {
        matches!(self.role, ServerRole::Computational)
    }

    /// Check if server is available
    pub fn is_available(&self) -> bool {
        self.state.is_available()
    }

    /// Check if server has failed
    pub fn is_failed(&self) -> bool {
        self.state.is_failed()
    }

    /// Store permutation shares (for computational servers)
    pub fn store_permutation_shares(&mut self, shares: Vec<Vec<Vec<SecretShare>>>) {
        if self.is_computational() {
            self.permutation_shares = shares;
        }
    }

    /// Store mask shares (for computational servers)
    pub fn store_mask_shares(&mut self, shares: Vec<Vec<Vec<SecretShare>>>) {
        if self.is_computational() {
            self.mask_shares = shares;
        }
    }

    /// Store noise shares (for computational servers)
    pub fn store_noise_shares(&mut self, shares: Vec<Vec<SecretShare>>) {
        if self.is_computational() {
            self.noise_shares = shares;
        }
    }

    /// Get permutation shares
    pub fn get_permutation_shares(&self) -> &Vec<Vec<Vec<SecretShare>>> {
        &self.permutation_shares
    }

    /// Get mask shares
    pub fn get_mask_shares(&self) -> &Vec<Vec<Vec<SecretShare>>> {
        &self.mask_shares
    }

    /// Get noise shares
    pub fn get_noise_shares(&self) -> &Vec<Vec<SecretShare>> {
        &self.noise_shares
    }

    /// Set final result
    pub fn set_final_result(&mut self, result: Vec<Vec<FieldElement>>) {
        if self.is_computational() {
            self.final_result = Some(result);
        }
    }

    /// Get final result
    pub fn get_final_result(&self) -> Vec<Vec<FieldElement>> {
        self.final_result.clone().unwrap_or_default()
    }

    /// Receive permutation shares
    pub fn receive_permutation_shares(&mut self, shares: Vec<Vec<Vec<SecretShare>>>) {
        if self.is_computational() {
            self.permutation_shares = shares;
        }
    }

    /// Receive mask shares
    pub fn receive_mask_shares(&mut self, shares: Vec<Vec<Vec<SecretShare>>>) {
        if self.is_computational() {
            self.mask_shares = shares;
        }
    }

    /// Receive noise shares
    pub fn receive_noise_shares(&mut self, shares: Vec<Vec<SecretShare>>) {
        if self.is_computational() {
            self.noise_shares = shares;
        }
    }

    /// Get server statistics
    pub fn get_stats(&self) -> ServerStats {
        ServerStats {
            id: self.id,
            role: self.role.clone(),
            state: self.state.clone(),
            permutation_shares_count: self.permutation_shares.len(),
            mask_shares_count: self.mask_shares.len(),
            noise_shares_count: self.noise_shares.len(),
            has_final_result: self.final_result.is_some(),
        }
    }
}

/// Server statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStats {
    /// Server ID
    pub id: usize,
    /// Server role
    pub role: ServerRole,
    /// Server state
    pub state: ServerState,
    /// Number of permutation shares
    pub permutation_shares_count: usize,
    /// Number of mask shares
    pub mask_shares_count: usize,
    /// Number of noise shares
    pub noise_shares_count: usize,
    /// Whether server has final result
    pub has_final_result: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let config = ToyConfig::default();
        let server = Server::new(0, ServerRole::Auxiliary, config);
        
        assert_eq!(server.id(), 0);
        assert!(server.is_auxiliary());
        assert!(!server.is_computational());
        assert_eq!(server.state(), &ServerState::Offline);
    }

    #[test]
    fn test_server_initialization() {
        let config = ToyConfig::default();
        let mut server = Server::new(1, ServerRole::Computational, config);
        
        server.initialize();
        assert_eq!(server.state(), &ServerState::Online);
        assert!(server.is_available());
    }

    #[test]
    fn test_server_role_checking() {
        let config = ToyConfig::default();
        let auxiliary = Server::new(0, ServerRole::Auxiliary, config.clone());
        let computational = Server::new(1, ServerRole::Computational, config);
        
        assert!(auxiliary.is_auxiliary());
        assert!(!auxiliary.is_computational());
        
        assert!(computational.is_computational());
        assert!(!computational.is_auxiliary());
    }

    #[test]
    fn test_server_state_management() {
        let config = ToyConfig::default();
        let mut server = Server::new(0, ServerRole::Auxiliary, config);
        
        server.set_state(ServerState::Participating);
        assert!(server.is_available());
        assert!(!server.is_failed());
        
        server.set_state(ServerState::Failed("Network error".to_string()));
        assert!(!server.is_available());
        assert!(server.is_failed());
    }
} 