# 测试指南

## ✅ 修复完成

已修复金额解析问题！现在 SDK 支持：
- 小数格式：`"0.0018"` → 自动转换为 1,800,000 lamports
- 原子单位：`"1800000"` → 直接使用

## 🚀 完整测试步骤

### 前提条件

1. **启动 Solana 本地验证器**（如果使用 localnet）
```bash
solana-test-validator
```

2. **准备测试账户**（需要有 SOL 余额）

### 步骤 1：启动 Facilitator（终端 1）

```bash
cargo run --example facilitator_example
```

预期输出：
```
=== X402 Facilitator Service ===
Network: SolanaDevnet
...
Starting Facilitator service at http://127.0.0.1:3002
```

### 步骤 2：启动 Server（终端 2）

```bash
cargo run --example server_example
```

预期输出：
```
=== X402 Payment-Protected Server ===
Configuration:
  Facilitator URL: http://localhost:3002
  Pay to address: <your_address>
  Network: SolanaDevnet
...
Starting payment-protected server at http://127.0.0.1:4021
```

### 步骤 3：运行 Client（终端 3）

```bash
cargo run --example client_example
```

预期输出：
```
=== X402 Client Example ===
Network: solana-localnet
...
Wallet created with public key: ...
Fetcher created with max payment: 0.1 USDC

Sending request to http://localhost:4021/weather...
✓ Response received
Status: 200
Body: {"report":{"weather":"sunny","temperature":70}}

✓ Request completed successfully!
```

## 🔧 故障排查

### 问题 1：金额解析错误（已修复）

**错误信息**：
```
Invalid input: Invalid amount: invalid digit found in string
```

**原因**：价格字符串包含小数点（如 `"0.0018"`），但代码尝试直接解析为 `u64`

**修复**：已在 `src/client/fetcher.rs` 中添加小数支持，自动转换为原子单位

### 问题 2：502 Bad Gateway

**可能原因**：
1. Facilitator 或 Server 未运行
2. 端口被占用
3. 配置错误

**解决方法**：
1. 确保 Facilitator 在端口 3002 运行
2. 确保 Server 在端口 4021 运行
3. 检查 `.env_*` 文件配置

### 问题 3：支付验证失败

**可能原因**：
1. 钱包余额不足
2. Facilitator URL 配置错误
3. 网络不匹配

**解决方法**：
1. 空投 SOL 到测试账户
2. 确保 `.env_server` 中 `FACILITATOR_URL=http://localhost:3002`
3. 确保所有配置使用相同的网络（如都是 `solana-localnet`）

## 📝 配置示例

### .env_client
```bash
USER_SVM_PRIVATE_KEY=your_private_key_here
SVM_NETWORK=solana-localnet
SVM_RPC_URL=http://127.0.0.1:8899
NEED_PAY_RESOURCE_URL=http://localhost:4021/weather
```

### .env_facilitator
```bash
SVM_PRIVATE_KEY=your_facilitator_private_key_here
SVM_NETWORK=solana-localnet
SVM_RPC_URL=http://127.0.0.1:8899
PORT=3002
HOST=127.0.0.1
```

### .env_server
```bash
ADDRESS=your_server_public_key_here
NETWORK=solana-localnet
FACILITATOR_URL=http://localhost:3002
HOST=127.0.0.1
PORT=4021
```

## ✅ 成功的端到端测试流程

1. ✅ Client 发送请求到 Server
2. ✅ Server 返回 402 Payment Required
3. ✅ Client 自动创建支付交易
4. ✅ Client 重试请求（带支付信息）
5. ✅ Server 调用 Facilitator 验证支付
6. ✅ Facilitator 验证交易签名和金额
7. ✅ Server 返回受保护内容
8. ✅ Server 调用 Facilitator 结算支付
9. ✅ Facilitator 提交交易到区块链

## 🎉 测试成功标志

当你看到以下输出时，说明一切正常：

**Client 输出**：
```
✓ Response received
Status: 200
Body: {"report":{"weather":"sunny","temperature":70}}
✓ Request completed successfully!
```

**Server 输出**：
```
=== Weather endpoint called ===
✓ Payment verified, settling...
✓ Payment settled successfully
```

**Facilitator 输出**：
```
=== Verify Payment Request ===
✓ Payment verified successfully

=== Settle Payment Request ===
✓ Payment settled successfully
Transaction signature: <signature>
```

---

**最后更新**：2025-01-05  
**修复版本**：0.1.1  
**状态**：金额解析问题已修复 ✅
