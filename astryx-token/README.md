# Astryx (NXS) - The 2026 Standard for Secure Solana Tokens

Astryx is a "Self-Healing, User-First, Compliant-by-Design" utility token built using SPL Token-2022 extensions on Solana. It solves 28 persistent problems in the cryptocurrency space through on-chain logic.

## Core Features
- **Dynamic Stability Mechanism (DSM):** Multi-oracle consensus (Pyth + Switchboard) to mitigate volatility.
- **Real-Yield Staking:** 40% of all transfer fees are redistributed to stakers.
- **Advanced Security:** Permanent bug-bounty fund, anti-scam blacklisting, and team vesting locks.
- **Regulatory Readiness:** Toggleable KYC/AML whitelisting and Travel Rule reporting.
- **Confidential Transfers:** Optional ZK-based privacy mode.

## 28 Problems Solved
1. Security & Scam Epidemic (Bug-bounty fund)
2. Extreme Volatility (DSM)
3. Awful UX (One-click metadata)
4. Privacy (ZK-mode)
... [Full list in the prompt/code]

## Quick Start
### Prerequisites
- Rust 1.80+
- Solana CLI 1.18+
- Anchor CLI 0.30.0+
- Node.js & Yarn

### Build & Deploy
1. **Clone & Setup:**
   ```bash
   git clone <repo-url>
   cd astryx-token
   ```
2. **Configure Program ID:**
   Generate a new keypair and replace the placeholder ID in `Anchor.toml` and `lib.rs`.
   ```bash
   solana-keygen new -o target/deploy/astryx-keypair.json
   ```
3. **Build:**
   ```bash
   anchor build
   ```
4. **Deploy to Devnet:**
   ```bash
   anchor deploy --provider.cluster devnet
   ```

### Initialize Token
Use the `initialize` instruction via a script or CLI to set the total supply (1,000,000,000 NXS) and vesting parameters.

### Tests
Run the Mocha test suite:
```bash
anchor test
```

## Security & Audits
Astryx logic is designed for immutability. Once the `upgrade_authority` is burned (after verification), the program becomes a trustless primitive. 
Recommended Auditors: Hacken, Certik, OtterSec.

---
*Built with ❤️ for the 2026 Solana Ecosystem.*
