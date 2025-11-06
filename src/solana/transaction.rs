use crate::error::X402Error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::Transaction as SolanaTransaction,
};
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose::STANDARD};

/// Transaction builder for Solana payments
pub struct TransactionBuilder {
    rpc_client: RpcClient,
}

impl TransactionBuilder {
    /// Create a new transaction builder with RPC client
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_client: RpcClient::new(rpc_url.to_string()),
        }
    }

    /// Create a payment transaction (SOL transfer)
    pub fn create_payment_transaction(
        &self,
        from: &Keypair,
        to: &Pubkey,
        amount_lamports: u64,
    ) -> Result<SolanaTransaction, X402Error> {
        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| X402Error::SolanaError(format!("Failed to get blockhash: {}", e)))?;

        let instruction = system_instruction::transfer(&from.pubkey(), to, amount_lamports);

        let message = Message::new(&[instruction], Some(&from.pubkey()));
        let mut transaction = SolanaTransaction::new_unsigned(message);
        transaction.sign(&[from], recent_blockhash);

        Ok(transaction)
    }

    /// Create an SPL token transfer transaction
    pub fn create_token_transfer_transaction(
        &self,
        from: &Keypair,
        to_token_account: &Pubkey,
        from_token_account: &Pubkey,
        _amount: u64,
        token_program_id: &Pubkey,
    ) -> Result<SolanaTransaction, X402Error> {
        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| X402Error::SolanaError(format!("Failed to get blockhash: {}", e)))?;

        // SPL Token Transfer instruction
        let instruction = Instruction::new_with_bincode(
            *token_program_id,
            &3u8, // Transfer instruction discriminator
            vec![
                AccountMeta::new(*from_token_account, false),
                AccountMeta::new(*to_token_account, false),
                AccountMeta::new_readonly(from.pubkey(), true),
            ],
        );

        let message = Message::new(&[instruction], Some(&from.pubkey()));
        let mut transaction = SolanaTransaction::new_unsigned(message);
        transaction.sign(&[from], recent_blockhash);

        Ok(transaction)
    }

    /// Create a simple SPL token transfer by token mint address
    /// This is a simplified version that derives the associated token accounts
    pub fn create_spl_token_payment(
        &self,
        _from: &Keypair,
        _to_owner: &Pubkey,
        _token_mint: &Pubkey,
        _amount: u64,
        _decimals: u8,
    ) -> Result<SolanaTransaction, X402Error> {
        // For now, use a simplified approach
        // In production, you would:
        // 1. Get or create associated token accounts for both sender and receiver
        // 2. Build proper SPL token transfer instruction with spl_token crate
        
        // Placeholder: For this implementation, we'll return an error indicating
        // that full SPL token support requires additional setup
        Err(X402Error::NotImplemented(
            "Full SPL Token transfer requires associated token account setup. \
             Please ensure both sender and receiver have associated token accounts \
             for the token mint.".to_string()
        ))
    }

    /// Sign a transaction
    pub fn sign_transaction(
        &self,
        transaction: &mut SolanaTransaction,
        signers: &[&Keypair],
    ) -> Result<(), X402Error> {
        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| X402Error::SolanaError(format!("Failed to get blockhash: {}", e)))?;

        transaction.sign(signers, recent_blockhash);
        Ok(())
    }

    /// Send and confirm a transaction
    pub fn send_and_confirm_transaction(
        &self,
        transaction: &SolanaTransaction,
    ) -> Result<Signature, X402Error> {
        let signature = self
            .rpc_client
            .send_and_confirm_transaction(transaction)
            .map_err(|e| X402Error::SolanaError(format!("Transaction failed: {}", e)))?;

        Ok(signature)
    }

    /// Serialize transaction to base64
    pub fn serialize_transaction(transaction: &SolanaTransaction) -> Result<String, X402Error> {
        let serialized = bincode::serialize(transaction)
            .map_err(|e| X402Error::SerializationError(format!("Failed to serialize: {}", e)))?;
        Ok(STANDARD.encode(serialized))
    }

    /// Deserialize transaction from base64
    pub fn deserialize_transaction(encoded: &str) -> Result<SolanaTransaction, X402Error> {
        let decoded = STANDARD.decode(encoded)
            .map_err(|e| X402Error::DeserializationError(format!("Failed to decode: {}", e)))?;
        let transaction: SolanaTransaction = bincode::deserialize(&decoded)
            .map_err(|e| X402Error::DeserializationError(format!("Failed to deserialize: {}", e)))?;
        Ok(transaction)
    }
}

/// Helper struct for transaction details
pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub recipient: String,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(id: String, amount: u64, recipient: String) -> Self {
        Self {
            id,
            amount,
            recipient,
        }
    }

    /// Execute the transaction on the Solana network
    pub fn execute(&self, builder: &TransactionBuilder, from: &Keypair) -> Result<Signature, X402Error> {
        let to_pubkey = Pubkey::from_str(&self.recipient)
            .map_err(|e| X402Error::InvalidInput(format!("Invalid recipient address: {}", e)))?;

        let transaction = builder.create_payment_transaction(from, &to_pubkey, self.amount)?;
        let signature = builder.send_and_confirm_transaction(&transaction)?;

        println!("Transaction executed: {}", signature);
        Ok(signature)
    }
}