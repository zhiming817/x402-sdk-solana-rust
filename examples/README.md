# Examples 使用指南

本目录包含 X402 SDK 的完整使用示例。

## 准备工作

### 1. 安装 Solana CLI（如果本地测试）

```bash
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```

### 2. 配置环境变量

复制示例配置文件：

```bash
# Client 配置
cp .env_client.example .env_client

# Server 配置（如果需要）
cp .env_server.example .env_server

# Facilitator 配置（如果需要）
cp .env_facilitator.example .env_facilitator
```

### 3. 生成测试密钥（本地测试）

```bash
# 生成三个测试账户
solana-keygen new --outfile client-key.json --no-bip39-passphrase
solana-keygen new --outfile server-key.json --no-bip39-passphrase
solana-keygen new --outfile facilitator-key.json --no-bip39-passphrase

# 获取私钥（Base58 格式）
cat client-key.json | jq -r '.' | base58

# 或使用 Python
python3 -c "import json, base58; print(base58.b58encode(bytes(json.load(open('client-key.json')))).decode())"
```

### 4. 空投测试代币（devnet 或 localnet）

```bash
# 启动本地测试验证器（如果使用 localnet）
solana-test-validator

# 在另一个终端中
solana config set --url http://localhost:8899  # localnet
# 或
solana config set --url https://api.devnet.solana.com  # devnet

# 空投 SOL
solana airdrop 10 $(solana-keygen pubkey client-key.json)
solana airdrop 10 $(solana-keygen pubkey server-key.json)
solana airdrop 10 $(solana-keygen pubkey facilitator-key.json)
```

## 运行示例

### Client 示例

演示如何使用 Fetcher 自动处理 402 支付：

```bash
# 1. 编辑 .env_client 文件，设置：
#    - USER_SVM_PRIVATE_KEY: 客户端私钥
#    - SVM_NETWORK: 网络 (solana-localnet/solana-devnet/solana)
#    - SVM_RPC_URL: RPC URL（可选）
#    - NEED_PAY_RESOURCE_URL: 要访问的资源 URL

# 2. 运行示例
cargo run --example client_example

# 预期输出：
# === X402 Client Example ===
# Network: solana-devnet
# RPC URL: https://api.devnet.solana.com
# Resource URL: http://localhost:4021/weather
#
# Wallet created with public key: ABC...XYZ
# Fetcher created with max payment: 0.1 USDC
#
# Sending request to http://localhost:4021/weather...
# ✓ Response received
# Status: 200
# Body: {"temperature":72,"condition":"sunny"}
#
# Payment Response: ...
# ✓ Request completed successfully!
```

### Server 示例

演示如何创建需要支付的 API：

```bash
# 1. 编辑 .env_server 文件
# 2. 运行示例
cargo run --example server_example

# 服务器将在 http://127.0.0.1:4021 启动
```

### Facilitator 示例

演示如何创建 Facilitator 服务：

```bash
# 1. 编辑 .env_facilitator 文件
# 2. 运行示例
cargo run --example facilitator_example

# Facilitator 将在 http://127.0.0.1:3002 启动
```

## 完整测试流程

### 本地端到端测试

1. **启动 Solana 本地验证器**（终端 1）：
```bash
solana-test-validator
```

2. **启动 Facilitator**（终端 2）：
```bash
cargo run --example facilitator_example
```

3. **启动 Server**（终端 3）：
```bash
cargo run --example server_example
```

4. **运行 Client**（终端 4）：
```bash
cargo run --example client_example
```

### Devnet 测试

1. **启动 Facilitator**（终端 1）：
```bash
# 编辑 .env_facilitator，设置 SVM_NETWORK=solana-devnet
cargo run --example facilitator_example
```

2. **启动 Server**（终端 2）：
```bash
# 编辑 .env_server，设置 NETWORK=solana-devnet
cargo run --example server_example
```

3. **运行 Client**（终端 3）：
```bash
# 编辑 .env_client，设置 SVM_NETWORK=solana-devnet
cargo run --example client_example
```

## 环境变量说明

### .env_client

| 变量 | 说明 | 示例 |
|-----|------|------|
| `USER_SVM_PRIVATE_KEY` | 客户端私钥（Base58） | `3E8kogunw...` |
| `SVM_NETWORK` | Solana 网络 | `solana-devnet` |
| `SVM_RPC_URL` | RPC URL（可选） | `https://api.devnet.solana.com` |
| `NEED_PAY_RESOURCE_URL` | 资源 URL | `http://localhost:4021/weather` |

### .env_server

| 变量 | 说明 | 示例 |
|-----|------|------|
| `ADDRESS` | 服务器地址（接收支付） | `67uA54AUE...` |
| `NETWORK` | 支付网络 | `solana-devnet` |
| `FACILITATOR_URL` | Facilitator URL | `http://localhost:3002` |
| `TOKEN_MINT_ADDRESS` | Token 地址（可选） | `EPjFWdd5A...` |
| `TOKEN_DECIMALS` | Token 精度（可选） | `6` |
| `TOKEN_NAME` | Token 名称（可选） | `USDC` |

### .env_facilitator

| 变量 | 说明 | 示例 |
|-----|------|------|
| `SVM_PRIVATE_KEY` | Facilitator 私钥 | `4FdeM2Hyx...` |
| `SVM_NETWORK` | Solana 网络 | `solana-devnet` |
| `SVM_RPC_URL` | RPC URL | `http://127.0.0.1:8899` |
| `PORT` | 服务端口 | `3002` |

## 常见问题

### Q: 运行时提示 "USER_SVM_PRIVATE_KEY must be set"

A: 请确保已经创建 `.env_client` 文件并设置了所需的环境变量。

### Q: 连接 RPC 失败

A: 检查：
1. 本地验证器是否运行（如果使用 localnet）
2. RPC URL 是否正确
3. 网络连接是否正常

### Q: 支付失败

A: 检查：
1. 账户是否有足够的 SOL（用于交易费用）
2. 账户是否有足够的 Token（如果使用 SPL Token）
3. Facilitator 是否正在运行
4. 支付金额是否超过 max_value 限制

### Q: 如何调试

A: 设置 RUST_LOG 环境变量：
```bash
RUST_LOG=debug cargo run --example client_example
```

## 进一步阅读

- [快速开始指南](../QUICK_START.md)
- [项目结构说明](../PROJECT_STRUCTURE.md)
- [完整文档](../README_NEW.md)
