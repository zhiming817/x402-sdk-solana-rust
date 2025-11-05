use crate::error::X402Error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};

/// Wallet for managing Solana accounts
pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    /// Create a new wallet with a new keypair
    pub fn new() -> Self {
        let keypair = Keypair::new();
        Self { keypair }
    }

    /// Create wallet from a base58 encoded private key
    pub fn from_private_key(private_key: &str) -> Result<Self, X402Error> {
        let decoded = bs58::decode(private_key)
            .into_vec()
            .map_err(|e| X402Error::InvalidInput(format!("Failed to decode private key: {}", e)))?;
        
        let keypair = Keypair::from_bytes(&decoded)
            .map_err(|e| X402Error::InvalidInput(format!("Invalid keypair bytes: {}", e)))?;
        
        Ok(Self { keypair })
    }

    /// Get the public key of this wallet
    pub fn public_key(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    /// Get a reference to the keypair
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    /// Get the balance of this wallet
    pub fn get_balance(&self, rpc_client: &RpcClient) -> Result<u64, X402Error> {
        let balance = rpc_client
            .get_balance(&self.keypair.pubkey())
            .map_err(|e| X402Error::SolanaError(format!("Failed to get balance: {}", e)))?;
        Ok(balance)
    }

    /// Get the balance of a specific pubkey
    pub fn get_balance_for(
        &self,
        pubkey: &Pubkey,
        rpc_client: &RpcClient,
    ) -> Result<u64, X402Error> {
        let balance = rpc_client
            .get_balance(pubkey)
            .map_err(|e| X402Error::SolanaError(format!("Failed to get balance: {}", e)))?;
        Ok(balance)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a signer (wallet) from network and private key
pub fn create_signer(_network: &str, private_key: &str) -> Result<Wallet, X402Error> {
    Wallet::from_private_key(private_key)
}