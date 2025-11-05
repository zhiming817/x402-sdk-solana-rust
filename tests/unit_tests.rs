// This file contains unit tests for the x402 SDK for Solana.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::fetcher::Fetcher;
    use crate::server::middleware::log_request;
    use crate::facilitator::handler::Handler;
    use crate::solana::transaction::Transaction;
    use crate::solana::wallet::Wallet;
    use crate::types::{Payment, Request};
    use crate::error::X402Error;

    #[test]
    fn test_fetch_data() {
        let fetcher = Fetcher::new();
        let result = fetcher.fetch_data("https://api.solana.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_request() {
        let request = Request {
            request_id: "123".to_string(),
            data: "Test data".to_string(),
        };
        let log_result = log_request(&request);
        assert!(log_result.is_ok());
    }

    #[test]
    fn test_handle_request() {
        let handler = Handler::new();
        let request = Request {
            request_id: "456".to_string(),
            data: "Handle this request".to_string(),
        };
        let result = handler.handle_request(request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_transaction() {
        let transaction = Transaction::new();
        let result = transaction.execute();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_wallet() {
        let wallet = Wallet::create("my_wallet");
        assert!(wallet.is_ok());
    }

    #[test]
    fn test_get_balance() {
        let wallet = Wallet::create("my_wallet").unwrap();
        let balance = wallet.get_balance();
        assert!(balance >= 0.0);
    }

    #[test]
    fn test_payment_struct() {
        let payment = Payment {
            amount: 100.0,
            recipient: "recipient_address".to_string(),
        };
        assert_eq!(payment.amount, 100.0);
        assert_eq!(payment.recipient, "recipient_address");
    }

    #[test]
    fn test_request_struct() {
        let request = Request {
            request_id: "789".to_string(),
            data: "Some data".to_string(),
        };
        assert_eq!(request.request_id, "789");
        assert_eq!(request.data, "Some data");
    }

    #[test]
    fn test_x402_error() {
        let error = X402Error::SomeError;
        assert_eq!(format!("{}", error), "SomeError");
    }
}