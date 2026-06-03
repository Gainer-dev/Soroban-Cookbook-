# DeFi Examples

Decentralized finance on Soroban: AMMs, lending, yield protocols.

## 📋 Examples (1 currently)

### [01-vault-strategies](../examples/vault-strategies/)
**Multi-strategy yield vault** with pluggable strategies and risk management.

**Key Concepts:**
- Strategy interface (`StrategyParams` + `StrategyType`)
- Three strategy implementations: Conservative, Balanced, Aggressive
- Admin-gated strategy switching with TVL circuit-breaker
- Emergency pause (deposits blocked, withdrawals always open)
- Allocation caps per strategy in basis points

**Quick Code:**
```rust
// Switch to a higher-yield strategy
client.switch_strategy(&admin, &StrategyType::Balanced);

// Estimate yield for planning
let yield_amount = client.estimate_yield(&10_000, &365);
```

---

## 📋 Coming Soon

### Automated Market Maker (AMM)
**Constant product pools** (x*y=k).

**Key Concepts:**
- Price curves & liquidity
- Swap math with slippage
- LP token mint/burn

### Lending Protocol
**Over-collateralized loans**.

**Key Concepts:**
- Oracle price feeds
- Liquidation thresholds
- Interest accrual

## Prerequisites
- [Basics](../basics.md), [Tokens](../tokens.md)

## Resources
- [Uniswap V2 Math](https://uniswap.org/whitepaper.pdf)
- Soroban token standards

## Next: [NFTs](../nfts.md)
