#!/bin/bash
# check_balances.sh

echo "=== Balance Check ==="
echo ""

echo "📊 Client (Payer):"
echo "  Address: 9j8Z38Zu61hD9qrp8GFQkYXNyLE1hg7Gj4WpUZaaKSyx"
echo "  SOL Balance:"
solana balance 9j8Z38Zu61hD9qrp8GFQkYXNyLE1hg7Gj4WpUZaaKSyx
echo "  USDC Balance:"
spl-token balance 3Suaq8Vp3RnyARZ9oyw3yaNyMtf2ShHGyKbNgn5dqVPY --owner 9j8Z38Zu61hD9qrp8GFQkYXNyLE1hg7Gj4WpUZaaKSyx
echo ""

echo "💼 Server (Receiver):"
echo "  Address: 42YEZmQvsHoENRD85tNNY3KY5nbqZMwPa4CQ2eDfW4Y5"
echo "  SOL Balance:"
solana balance 42YEZmQvsHoENRD85tNNY3KY5nbqZMwPa4CQ2eDfW4Y5
echo "  USDC Balance:"
spl-token balance 3Suaq8Vp3RnyARZ9oyw3yaNyMtf2ShHGyKbNgn5dqVPY --owner 42YEZmQvsHoENRD85tNNY3KY5nbqZMwPa4CQ2eDfW4Y5
echo ""

echo "📋 All Token Accounts:"
echo "Client:"
spl-token accounts --owner 9j8Z38Zu61hD9qrp8GFQkYXNyLE1hg7Gj4WpUZaaKSyx
echo ""
echo "Server:"
spl-token accounts --owner 42YEZmQvsHoENRD85tNNY3KY5nbqZMwPa4CQ2eDfW4Y5