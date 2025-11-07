use crate::error::X402Error;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction as SolanaTransaction,
};
use solana_system_interface::instruction as system_instruction;
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

        // Solana 3.0: Use system_instruction from solana_system_interface crate
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
    /// This creates a complete token transfer with automatic ATA creation if needed
    pub fn create_spl_token_payment(
        &self,
        from: &Keypair,
        to_owner: &Pubkey,
        token_mint: &Pubkey,
        amount: u64,
        decimals: u8,
    ) -> Result<SolanaTransaction, X402Error> {
        let payer = from.pubkey();
        let token_program_id = spl_token::ID;
        
        // 1. Derive associated token accounts using Solana 3.0 compatible method
        let sender_ata = spl_associated_token_account::get_associated_token_address(
            &payer,
            token_mint,
        );
        let receiver_ata = spl_associated_token_account::get_associated_token_address(
            to_owner,
            token_mint,
        );
        
        println!("  💳 Sender ATA: {}", sender_ata);
        println!("  💳 Receiver ATA: {}", receiver_ata);
        
        let mut instructions: Vec<Instruction> = Vec::new();
        
        // 2. Check if sender has the token account; if missing, create it
        match self.rpc_client.get_account(&sender_ata) {
            Ok(_) => {
                println!("  ✓ Sender ATA exists");
            }
            Err(_) => {
                println!("  ⚠️  Sender ATA doesn't exist, creating...");

                // Create ATA instruction for sender
                let create_sender_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
                    &payer,           // funding account
                    &payer,           // wallet address (sender)
                    token_mint,       // SPL Token mint
                    &token_program_id, // Token program ID
                );
                instructions.push(create_sender_ata_ix);
            }
        }
        
        // 3. Check if receiver's ATA exists, create if not
        match self.rpc_client.get_account(&receiver_ata) {
            Ok(_) => {
                println!("  ✓ Receiver ATA exists");
            }
            Err(_) => {
                println!("  ⚠️  Receiver ATA doesn't exist, creating...");
                
                // Create ATA instruction for receiver
                let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
                    &payer,           // funding account
                    to_owner,         // wallet address (receiver)
                    token_mint,       // SPL Token mint
                    &token_program_id, // Token program ID
                );
                instructions.push(create_ata_ix);
            }
        }
        
        // 4. Create transfer instruction
        let transfer_ix = spl_token::instruction::transfer_checked(
            &token_program_id, // token program
            &sender_ata,       // source
            token_mint,        // mint
            &receiver_ata,     // destination
            &payer,            // authority
            &[],               // signers (empty because authority will sign)
            amount,            // amount
            decimals,          // decimals
        )
        .map_err(|e| X402Error::SolanaError(format!("Failed to create transfer instruction: {}", e)))?;
        
        instructions.push(transfer_ix);
        
        // 5. Get recent blockhash
        let recent_blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .map_err(|e| X402Error::SolanaError(format!("Failed to get blockhash: {}", e)))?;
        
        // 6. Create and sign transaction
        let message = Message::new(&instructions, Some(&payer));
        let mut transaction = SolanaTransaction::new_unsigned(message);
        transaction.sign(&[from], recent_blockhash);
        
        println!("  ✅ Token transfer transaction created and signed");
        
        Ok(transaction)
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