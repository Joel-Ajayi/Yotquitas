# The Complete Guide to Yotquitas: From End-User to Blockchain

**A Comprehensive Understanding of Blockchain Economics, DeFi, and the Yotquitas Ecosystem**

---

## Table of Contents

1. [Introduction: Why This Guide Exists](#introduction)
2. [Part I: Blockchain Fundamentals for End-Users](#part-i)
3. [Part II: Money, Tokens, and Value](#part-ii)
4. [Part III: The Yotquitas Launch Process](#part-iii)
5. [Part IV: How Smart Contract Developers Make Money](#part-iv)
6. [Part V: DeFi Explained: The Complete Picture](#part-v)
7. [Part VI: The Complete User Journey](#part-vi)
8. [Part VII: Technical Flow: What Happens Behind the Scenes](#part-vii)
9. [Part VIII: Token Economics Deep Dive](#part-viii)
10. [Part IX: Common Questions Answered](#part-ix)

---

## Introduction: Why This Guide Exists {#introduction}

You're building Yotquitas, a next-generation blockchain focused on fair DeFi. But to build it right, you need to understand not just the technology, but the complete economic and user ecosystem. This guide bridges the gap between being a blockchain builder and understanding what happens when real users interact with your blockchain.

**What You'll Learn:**

- How real people use blockchains (not just developers)
- How money flows through the system
- How smart contract developers monetize their work
- What happens during launch
- How DeFi actually works from a user perspective
- The complete journey from "I want to use DeFi" to "transaction confirmed"

---

## Part I: Blockchain Fundamentals for End-Users {#part-i}

### 1.1 What is a Blockchain? (User Perspective)

Think of a blockchain like a **public ledger** that everyone can see, but no one can change. Imagine a Google Sheet that:

- Everyone has a copy of
- When someone adds a row, everyone's copy updates
- Once a row is added, it can never be deleted or changed
- You can see who added what and when

**Real-World Analogy:**

- Traditional banking: You trust your bank to keep your money safe. The bank has a private ledger.
- Blockchain: Everyone has a copy of the ledger. No single person controls it.

### 1.2 What is a Wallet?

A **wallet** is NOT where your money is stored. It's more like a **keychain** that holds:

- **Private Key**: Like a password that proves you own your money (NEVER share this!)
- **Public Key/Address**: Like a bank account number (you can share this to receive money)

**Important:** Your money isn't "in" the wallet. It's on the blockchain. The wallet just lets you access it.

**Example:**

- Your address: `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb`
- This is like your bank account number
- Anyone can send you money to this address
- Only you (with your private key) can send money FROM this address

### 1.3 What is a Transaction?

A transaction is like writing a check:

- **From**: Your address
- **To**: Recipient's address
- **Amount**: How much you're sending
- **Fee**: Small payment to process the transaction (like a bank fee)
- **Signature**: Your digital signature proving you authorized it

**What Happens:**

1. You create a transaction in your wallet
2. Your wallet signs it with your private key
3. Transaction is broadcast to the network
4. Validators (nodes) verify and process it
5. Transaction is added to a block
6. Block is added to the blockchain
7. Your transaction is now permanent

### 1.4 What is Gas/Fees?

**Gas** is the fee you pay to use the blockchain. Think of it like:

- **Toll road**: You pay to use the highway
- **Processing fee**: Like a credit card processing fee
- **Computational cost**: Paying for the computer power to run your transaction

**On Yotquitas:**

- Fee is paid in **AEQ** (the native token)
- Fees go to validators who process transactions
- Higher fees = faster processing (validators prioritize high-fee transactions)

**Example:**

- You want to send 100 AEQ to Alice
- Transaction fee: 0.001 AEQ
- Total deducted from your account: 100.001 AEQ
- Alice receives: 100 AEQ
- Validator receives: 0.001 AEQ

### 1.5 What is a Smart Contract?

A **smart contract** is like a vending machine:

- You put money in (send a transaction)
- The machine automatically gives you what you paid for
- No human needed to operate it
- It always works the same way

**Real Examples on Yotquitas:**

- **Token Contract**: Creates a new token on Yotquitas
- **DEX Contract**: Lets you swap AEQ for other tokens
- **Lending Contract**: Lets you borrow tokens by putting up AEQ as collateral

**Key Point:** Smart contracts are code that runs on the blockchain. Once deployed, they run automatically and can't be changed (unless the contract allows it).

---

## Part II: Money, Tokens, and Value {#part-ii}

### 2.1 What is AEQ? (Yotquitas Native Token)

**AEQ** (pronounced "A-E-Q" or "Aequitas") is the native token of Yotquitas. Think of it like:

- **Naira in Nigeria**: The official currency
- **AEQ on Yotquitas**: The official currency

**Exchange Rate:**

- **1 AEQ = ₦100** (Nigerian Naira) - Fixed initial exchange rate at launch
- This rate is used for all NGN ↔ AEQ conversions on the purchase/sell portal

**What AEQ is Used For:**

1. **Transaction Fees**: Pay to use the network
2. **Staking**: Lock up AEQ to become a validator
3. **Value Transfer**: Send money to others
4. **Governance**: Vote on network upgrades (in future)

### 2.2 What Does 1 AEQ Equal?

**Fixed Exchange Rate:**

- **1 AEQ = ₦100** (Nigerian Naira)

This is the fixed rate used for:

- Purchase portal (NGN → AEQ)
- Sell portal (AEQ → NGN)
- Initial DEX pricing

**After Launch:**

- Price may fluctuate based on supply and demand on DEX
- Purchase/sell portal may maintain fixed rate or follow market price
- DEX trading determines market price through supply/demand
- **Value is Determined By:**
  - How many people want to use Yotquitas
  - How useful the network is
  - How much people trust it
  - Supply and demand (like stocks)

**For Yotquitas:**

- Initial supply: 1 billion AEQ (as per L4 spec)
- Distribution: Determined at launch (see Part III)
- Price discovery: Begins immediately through purchase portal and built-in DEX
- **Initial Rate**: 1 AEQ = ₦100 (fixed at launch)
- **After Launch**: Price may fluctuate based on supply/demand on Yotquitas purchase portal and DEX trading

### 2.3 How Does Fiat Money (Naira) Become Blockchain Currency?

**Yotquitas is a Self-Contained Ecosystem:** Yotquitas includes built-in mechanisms to convert NGN (Nigerian Naira) to AEQ and vice versa, so users don't need to wait for external exchanges. The system is fully functional from day one.

#### Method 1: Direct Purchase Portal (Built into Yotquitas)

**Yotquitas Purchase Portal:**

- Built directly into the Yotquitas ecosystem
- Available at launch on the Yotquitas website
- Accepts NGN (Nigerian Naira) via bank transfer, card payment, or payment processors (Paystack, Flutterwave)
- AEQ sent directly to your wallet instantly
- No external exchanges needed

**How It Works:**

1. Visit Yotquitas purchase portal (yotquitas.dev/buy)
2. Connect your Yotquitas wallet
3. Enter amount of NGN you want to spend
4. Pay via bank transfer, card payment, or mobile money
5. AEQ is minted and sent to your wallet immediately
6. You can start using Yotquitas right away

**Example:**

- You want to buy 1,000 AEQ
- Price: ₦100 per AEQ (fixed rate)
- You pay ₦100,000 NGN via bank transfer
- 1,000 AEQ appears in your wallet within seconds
- You can immediately use it for transactions or DeFi

**Technical Implementation:**

- Yotquitas treasury holds initial AEQ supply
- Purchase portal is a smart contract that:
  - Receives NGN payment (via payment processor API like Paystack or Flutterwave)
  - Mints or transfers AEQ from treasury
  - Sends AEQ to user's wallet
  - Records transaction on blockchain

#### Method 2: Built-in DEX (Launches with Network)

**Yotquitas Native DEX:**

- Launches simultaneously with the network
- Pre-funded with initial liquidity from treasury
- Allows swapping between AEQ and other tokens
- No need to wait for third-party DEXs

**How It Works:**

1. User connects wallet to Yotquitas DEX
2. DEX has pre-funded liquidity pools (AEQ + stablecoins/tokens)
3. User can swap NGN-pegged tokens for AEQ
4. Or swap AEQ for other tokens
5. All happens on Yotquitas blockchain

**Initial Liquidity:**

- Yotquitas treasury provides initial liquidity
- Example: 1M AEQ + 100M NGN worth of stablecoins (₦100 per AEQ)
- Enables trading from day one
- Community can add more liquidity over time

#### Method 3: Faucet (For Testing and Getting Started)

**Yotquitas Faucet:**

- Free AEQ for new users to get started
- Small amounts (e.g., 10-100 AEQ)
- Helps users pay for first transactions
- Available on Yotquitas website

**How It Works:**

1. Visit Yotquitas faucet page
2. Enter your wallet address
3. Complete simple verification (captcha, email)
4. Receive free AEQ (limited per address)
5. Use for your first transactions

**Purpose:**

- Onboard new users
- Allow users to pay gas fees
- Enable first transactions without purchase
- Build user base

#### Method 4: Airdrops (Community Building)

**Airdrop Program:**

- Free AEQ distributed to early supporters
- Sign up before launch
- Complete tasks (social media, referrals)
- Receive AEQ directly to wallet

**Example:**

- Sign up for Yotquitas airdrop
- Follow on Twitter, join Discord
- Refer 3 friends
- Receive 100 AEQ free

#### Method 5: Earn AEQ

**Become a Validator:**

- Stake 1M+ AEQ
- Earn transaction fees + inflation rewards
- Help secure the network

**Provide Liquidity:**

- Deposit AEQ + tokens to DEX pools
- Earn trading fees
- Grow your holdings

**Develop Applications:**

- Build DeFi apps on Yotquitas
- Earn fees from users
- Receive grants from treasury

#### Method 6: Sell AEQ Back to NGN

**Yotquitas Sell Portal:**

- Built-in mechanism to convert AEQ back to NGN
- Visit sell portal (yotquitas.dev/sell)
- Connect wallet
- Enter amount of AEQ to sell
- Receive NGN via bank transfer or payment processor
- AEQ is burned or returned to treasury

**How It Works:**

1. User sends AEQ to sell portal contract
2. Contract burns AEQ or returns to treasury
3. User receives NGN equivalent
4. Processed within 1-3 business days

#### Complete User Journey: NGN to AEQ and Back

**Buying AEQ (NGN → AEQ):**

1. **Get Wallet:**

   - Download Yotquitas wallet
   - Create account, save seed phrase

2. **Buy AEQ:**

   - Visit yotquitas.dev/buy
   - Connect wallet
   - Enter NGN amount
   - Pay via credit card/bank transfer
   - Receive AEQ instantly

3. **Use AEQ:**
   - Send to others
   - Use DeFi applications
   - Trade on DEX
   - Stake as validator

**Selling AEQ (AEQ → NGN):**

1. **Sell AEQ:**

   - Visit yotquitas.dev/sell
   - Connect wallet
   - Enter AEQ amount
   - Confirm transaction
   - Receive NGN in 1-3 business days

2. **Alternative: Use DEX**
   - Swap AEQ for stablecoins on DEX
   - Stablecoins can be redeemed for NGN
   - Faster but requires DEX liquidity

#### Important Concepts

**Yotquitas Purchase/Sell Portal:**

- Built directly into Yotquitas ecosystem
- No external dependencies
- Works from day one
- Handles NGN ↔ AEQ conversion
- Managed by Yotquitas treasury

**Native DEX:**

- Launches with the network
- Pre-funded with initial liquidity
- Enables token trading immediately
- Community can add liquidity
- Fully decentralized after launch

**Self-Contained Economy:**

- Users can buy AEQ with NGN directly (1 AEQ = ₦100)
- Users can sell AEQ for NGN directly (1 AEQ = ₦100)
- No need for external exchanges
- Fully functional from launch
- Ecosystem is independent and complete

### 2.4 Token Supply and Inflation

**Initial Supply:**

- 1 billion AEQ created at launch
- Distributed to:
  - Early investors/backers
  - Team/developers
  - Community/airdrops
  - Treasury (for future development)

**Inflation (Annual):**

- 5% annually (as per L4 spec)
- New AEQ created each year
- Goes to validators as rewards
- Purpose: Incentivize validators to secure the network

**Example:**

- Year 1: 1,000,000,000 AEQ
- Year 2: 1,050,000,000 AEQ (5% increase)
- Year 3: 1,102,500,000 AEQ (5% of new total)

**Why Inflation?**

- Rewards validators for securing the network
- Encourages staking (locking up tokens)
- Similar to how governments print money (but predictable)

**Deflation Mechanisms:**

- Transaction fees might be burned (destroyed)
- Reduces total supply
- Can offset inflation

---

## Part III: The Yotquitas Launch Process {#part-iii}

### 3.1 Pre-Launch: Building the Foundation

**Phase 1: Development (Levels 0-3)**

- Build the core blockchain
- Test on private testnet
- Fix bugs and optimize
- No public access yet

**Phase 2: Testnet Launch**

- Public testnet opens
- Free test tokens (not real value)
- Developers test their applications
- Community tests the network
- Find and fix issues

**Phase 3: Mainnet Preparation**

- Final security audits
- Token distribution planning
- Exchange listings (getting AEQ on Coinbase, etc.)
- Marketing and community building

### 3.2 Launch Day: What Actually Happens

**Step 1: Genesis Block**

- First block is created
- Contains initial token distribution
- All initial AEQ tokens are created
- Network goes live

**Step 2: Validators Come Online**

- Validators start running nodes
- They stake their AEQ
- Network begins processing transactions
- Blocks start being produced

**Step 3: Ecosystem Launch**

- Purchase portal goes live (NGN → AEQ conversion)
- Built-in DEX launches with initial liquidity
- Faucet opens for new users
- Airdrops distributed to early supporters
- Users can immediately buy AEQ with NGN and start using the network
- Price discovery begins through DEX trading and purchase portal

**Step 4: First Transactions**

- Early users send AEQ to each other
- Developers deploy first Move smart contracts
- First DeFi applications launch on Yotquitas
- Network proves it works

### 3.3 Post-Launch: Building the Economy

**Week 1-4: Early Adoption**

- Early users and developers join
- First DeFi applications launch
- Community grows
- Price volatility (normal for new tokens)

**Month 2-6: Growth Phase**

- More applications built
- More users join
- Network effects kick in
- Price stabilizes (or grows if successful)

**Month 6+: Maturity**

- Established ecosystem
- Many applications
- Large user base
- Stable network

### 3.4 Token Distribution at Launch

**Typical Distribution (Example):**

- **Team/Founders**: 20% (vested over 4 years)
- **Investors**: 15% (vested over 2-3 years)
- **Community/Airdrop**: 10% (distributed to early supporters)
- **Treasury**: 15% (for future development)
- **Validators/Staking Rewards**: 40% (distributed over time via inflation)

**Vesting:**

- Tokens are "locked" and released over time
- Prevents team/investors from dumping all tokens at once
- Protects early users

**Airdrop:**

- Free tokens given to early supporters
- Rewards community engagement
- Marketing tool

---

## Part IV: How Smart Contract Developers Make Money {#part-iv}

### 4.1 Revenue Models for Smart Contract Developers

Smart contract developers make money in several ways:

#### Model 1: Transaction Fees (Most Common)

**How It Works:**

- Developer creates a DeFi application (e.g., a DEX)
- Every time someone uses it, they pay a small fee
- Developer takes a percentage (e.g., 0.3% of each trade)
- Fees accumulate over time

**On Yotquitas:**

- Developer deploys a DEX contract
- Sets fee: 0.5% per trade
- User trades 10,000 AEQ (worth ₦1,000,000 at ₦100 per AEQ)
- Fee: ₦5,000 (0.5%)
- Developer earns ₦5,000 (minus gas costs)

#### Model 2: Token Launch / ICO

**How It Works:**

- Developer creates a new token
- Sells tokens to raise money
- Early buyers get tokens at lower price
- If project succeeds, token value increases

**Example:**

- Developer creates "MyDeFi Token" (MDF)
- Sells 1 million tokens at ₦100 each (or equivalent in AEQ)
- Raises ₦100 million
- If MDF value increases, early buyers profit
- Developer uses raised funds to build project

#### Model 3: Subscription / Premium Features

**How It Works:**

- Free basic features
- Paid premium features
- Users pay monthly/yearly subscription
- Paid in AEQ or project tokens

**Example:**

- Free: Basic trading
- Premium (₦1,000/month or 10 AEQ): Advanced analytics, lower fees
- Users pay in AEQ
- Developer earns recurring revenue

#### Model 4: Governance Tokens

**How It Works:**

- Developer creates a protocol
- Issues governance tokens
- Token holders vote on protocol changes
- Tokens have value (can be traded)
- Developer keeps some tokens

**Example:**

- Developer creates lending protocol
- Issues 1 million governance tokens
- Keeps 200,000 tokens (20%)
- If tokens worth ₦1,000 each = ₦200 million
- Can sell over time or use for voting power

#### Model 5: Yield Farming / Liquidity Mining

**How It Works:**

- Developer creates DeFi protocol
- Users provide liquidity (deposit tokens)
- Protocol rewards them with new tokens
- Developer controls token distribution
- Can keep tokens or sell them

**Example:**

- Developer creates lending platform
- Users deposit ₦100 million (1M AEQ at ₦100 per AEQ)
- Protocol mints 10,000 tokens/day as rewards
- Developer keeps 10% = 1,000 tokens/day
- If tokens worth ₦100 each = ₦100,000/day income

### 4.2 Real-World Example: Building a DEX on Yotquitas

**Step 1: Developer Creates DEX Contract**

- Writes Move smart contract
- Deploys to Yotquitas
- Cost: ~100 AEQ in gas fees

**Step 2: Set Fee Structure**

- Trading fee: 0.3%
- Developer keeps: 0.1%
- Liquidity providers get: 0.2%

**Step 3: Launch and Marketing**

- Announce on social media
- Get users to provide liquidity
- Users start trading

**Step 4: Revenue Generation**

- Day 1: ₦1,000,000 trading volume (10,000 AEQ at ₦100 per AEQ)
  - Fees: ₦3,000 (0.3%)
  - Developer share: ₦1,000
- Day 30: ₦100,000,000 trading volume (1M AEQ)
  - Fees: ₦300,000
  - Developer share: ₦100,000
- Month 1 total: ~₦3,000,000 developer revenue

**Step 5: Scale**

- More users = more volume
- More volume = more fees
- Successful DEX can generate ₦10M+/month

### 4.3 How Developers Get Paid

**Method 1: Direct Withdrawal**

- Fees accumulate in smart contract
- Developer calls "withdraw" function
- AEQ sent to developer's wallet
- Developer can sell for fiat

**Method 2: Treasury/Multisig**

- Fees go to treasury contract
- Multiple people control it (security)
- Regular distributions to team
- More secure, less flexible

**Method 3: Token Distribution**

- Developer creates project token
- Fees buy back and burn tokens
- Increases token value
- Developer sells tokens for profit

### 4.4 Risks and Challenges

**Smart Contract Bugs:**

- Code has vulnerability
- Hackers exploit it
- Developer loses money
- Reputation damage

**Market Competition:**

- Other developers build better version
- Users switch to competitor
- Revenue drops

**Regulatory Risk:**

- Government bans or regulates
- Can't operate in certain countries
- Legal issues

**Market Conditions:**

- Crypto market crashes
- Users leave
- Revenue drops
- Project fails

---

## Part V: DeFi Explained: The Complete Picture {#part-v}

### 5.1 What is DeFi?

**DeFi** = **Decentralized Finance**

Traditional finance (TradFi):

- Banks hold your money
- Banks control transactions
- Need permission to use
- Centralized

Decentralized finance (DeFi):

- You control your money
- Smart contracts handle transactions
- No permission needed
- Decentralized

### 5.2 Core DeFi Applications

#### 5.2.1 Decentralized Exchanges (DEX)

**What It Does:**

- Lets you swap one token for another
- Like a currency exchange
- No middleman

**How It Works:**

1. User wants to swap 100 AEQ for another token (e.g., a project token)
2. User connects wallet to DEX
3. DEX finds best price from liquidity pools
4. User approves transaction
5. 100 AEQ sent to pool, tokens sent to user
6. Small fee charged (0.3-0.5%)

**Liquidity Pools:**

- Users deposit AEQ and tokens (provide liquidity)
- Earn fees from trades
- Like being a market maker

**Example:**

- Alice deposits 1000 AEQ + 2000 tokens to pool
- Bob swaps 100 AEQ for tokens
- Pool now has 1100 AEQ + 1900 tokens (approximately)
- Alice earns portion of Bob's trading fee

#### 5.2.2 Lending and Borrowing

**What It Does:**

- Lend your tokens, earn interest
- Borrow tokens by putting up collateral

**How Lending Works:**

1. User deposits 1000 AEQ
2. Gets interest-bearing token (e.g., aAEQ)
3. Earns interest over time
4. Can withdraw anytime

**How Borrowing Works:**

1. User deposits 2000 AEQ as collateral
2. Can borrow up to 1400 AEQ worth of tokens (70% loan-to-value)
3. Pays interest on borrowed amount
4. Must repay to get collateral back

**Example:**

- Alice deposits 10,000 AEQ (worth ₦1,000,000 at ₦100 per AEQ)
- Borrows tokens worth 7,000 AEQ
- Uses borrowed tokens for other purposes
- If AEQ price goes up, Alice profits
- If AEQ price drops too much, Alice's collateral gets liquidated

**Interest Rates:**

- Determined by supply and demand
- More lenders = lower rates
- More borrowers = higher rates

#### 5.2.3 Yield Farming

**What It Does:**

- Earn rewards by providing liquidity
- Like earning interest, but higher risk/reward

**How It Works:**

1. User provides liquidity to pool (e.g., AEQ/stablecoin)
2. Gets LP tokens (liquidity provider tokens)
3. Stakes LP tokens in farm
4. Earns farm tokens as rewards
5. Can sell farm tokens for profit

**Example:**

- Alice provides ₦1,000,000 liquidity (5000 AEQ + 5000 tokens worth ₦500,000)
- Gets LP tokens worth ₦1,000,000
- Stakes in farm
- Earns 100 FARM tokens/day
- FARM tokens worth ₦100 each = ₦10,000/day
- Annual yield: ~365% (very high, high risk)

**Risks:**

- **Impermanent Loss**: Token prices change, you lose money
- **Smart Contract Risk**: Code could have bugs
- **Token Price Risk**: Farm tokens could drop to ₦0

#### 5.2.4 Staking

**What It Does:**

- Lock up tokens to secure the network
- Earn rewards

**On Yotquitas:**

- Stake AEQ to become validator
- Minimum: 1 million AEQ
- Earn: Transaction fees + inflation rewards
- Help secure the network

**Example:**

- Alice stakes 2 million AEQ
- Becomes validator
- Processes transactions
- Earns 0.001 AEQ per transaction
- 1000 transactions/day = 1 AEQ/day
- Plus inflation rewards (5% annually)
- Total: ~274 AEQ/year from staking + transaction fees

#### 5.2.5 Stable Value Tokens

**What It Does:**

- Tokens pegged to USD value (1 token = $1)
- Less volatile than AEQ
- Created by smart contracts on Yotquitas

**How It Works:**

- Smart contract maintains peg to USD
- Backed by collateral (AEQ or other assets)
- Users can mint/redeem at 1:1 ratio with USD

**Why Use:**

- Store value without AEQ volatility
- Easy to price things in USD terms
- Useful for DeFi applications requiring stable value

### 5.3 DeFi Risks

**Smart Contract Risk:**

- Code has bugs
- Hackers exploit
- You lose money

**Impermanent Loss:**

- Providing liquidity to pools
- Token prices change
- You lose money vs. just holding

**Liquidation Risk:**

- Borrowing with collateral
- Price drops
- Collateral gets liquidated
- You lose money

**Rug Pull:**

- Developer creates scam project
- Takes all money
- Disappears
- You lose everything

**Regulatory Risk:**

- Government bans DeFi
- Can't use anymore
- Value drops

---

## Part VI: The Complete User Journey {#part-vi}

### 6.1 Journey 1: First-Time User Wants to Use DeFi

**Step 1: Get a Wallet**

- User downloads wallet app (e.g., Yotquitas Wallet)
- Creates new wallet
- Gets seed phrase (12-24 words) - MUST SAVE THIS
- Wallet generates address

**Step 2: Get Some AEQ**

- **Option A (Purchase Portal)**: User visits yotquitas.dev/buy, pays NGN with bank transfer/card, receives AEQ instantly
- **Option B (Faucet)**: User visits faucet, gets free AEQ to start (small amount)
- **Option C (Airdrop)**: User signed up for airdrop before launch, receives free AEQ
- **Option D (DEX)**: User swaps tokens on built-in Yotquitas DEX for AEQ
- AEQ arrives in Yotquitas wallet
- Confirmation takes seconds (Yotquitas is fast!)

**Step 3: Connect to DeFi App**

- User opens DEX website
- Clicks "Connect Wallet"
- Approves connection
- Wallet is now connected

**Step 4: Make First Trade**

- User wants to swap 100 AEQ for another token
- Enters amount in DEX
- Clicks "Swap"
- Wallet pops up asking to confirm
- User approves
- Transaction sent to Yotquitas network
- Waits for confirmation (~2-5 seconds)
- Trade completes
- User now has the other token

**Step 5: Provide Liquidity**

- User wants to earn fees
- Goes to liquidity pool
- Deposits 1000 AEQ + equivalent value of another token
- Gets LP tokens
- Starts earning trading fees

**Total Time:** ~30 minutes for first-time user
**Cost:** Transaction fees (~0.001-0.01 AEQ per transaction)

### 6.2 Journey 2: Developer Deploys Smart Contract

**Step 1: Write Contract**

- Developer writes Move smart contract
- Tests locally
- Fixes bugs

**Step 2: Deploy to Testnet**

- Deploys to Yotquitas testnet
- Tests with test tokens (free)
- Verifies everything works

**Step 3: Deploy to Mainnet**

- Deploys to Yotquitas mainnet
- Pays gas fee (~100 AEQ)
- Contract is now live
- Gets contract address

**Step 4: Users Interact**

- Users find contract
- Start using it
- Developer earns fees

**Total Time:** Days/weeks of development, minutes to deploy
**Cost:** Gas fees for deployment and interactions

### 6.3 Journey 3: Becoming a Validator

**Step 1: Get Enough AEQ**

- Need 1 million AEQ minimum
- Buy on exchange or earn over time

**Step 2: Set Up Node**

- Download Yotquitas node software
- Install on server
- Configure settings

**Step 3: Stake AEQ**

- Lock up 1M+ AEQ
- Become validator
- Start processing transactions

**Step 4: Earn Rewards**

- Process transactions (earn fees)
- Earn inflation rewards (5% annually)
- Withdraw rewards periodically

**Total Time:** Hours to set up, ongoing operation
**Cost:** Server costs + 1M AEQ staked

---

## Part VII: Technical Flow: What Happens Behind the Scenes {#part-vii}

### 7.1 Complete Transaction Flow

**User Action:** Alice wants to send 100 AEQ to Bob

#### Step 1: User Creates Transaction

```
Alice's Wallet:
- From: 0xAlice...
- To: 0xBob...
- Amount: 100 AEQ
- Fee: 0.001 AEQ
- Nonce: 42 (transaction number)
```

#### Step 2: Wallet Signs Transaction

- Wallet uses Alice's private key
- Creates digital signature
- Proves Alice authorized transaction
- Transaction is now signed

#### Step 3: Transaction Broadcast

- Wallet sends to Yotquitas node
- Node validates signature
- Node adds to mempool (pending transactions)
- Node broadcasts to other nodes via Gossipsub

#### Step 4: Validator Picks Up Transaction

- Validator sees transaction in mempool
- Validator verifies:
  - Signature is valid
  - Alice has enough balance (100.001 AEQ)
  - Nonce is correct (prevents replay attacks)
- Validator includes in next block

#### Step 5: Block Creation (Level 2+)

- Validator creates block with transactions
- Block includes:
  - Block header (index, timestamp, previous hash)
  - List of transactions
  - Merkle root (hash of all transactions)
- Validator signs block

#### Step 6: Consensus (Level 2+)

- **Level 2 (PoA)**: Authorized validators agree on block
- **Level 3 (Fino-Narwhal-Tusk)**:
  - Transactions encrypted with Fino
  - Narwhal creates DAG of batches
  - Tusk orders batches (still encrypted)
  - Validators decrypt after ordering
  - Prevents front-running

#### Step 7: Block Added to Chain

- Block is finalized
- Added to blockchain
- All nodes update their state
- Transaction is now permanent

#### Step 8: State Update

- Alice's balance: -100.001 AEQ
- Bob's balance: +100 AEQ
- Validator's balance: +0.001 AEQ (fee)
- State saved in RocksDB

#### Step 9: User Sees Confirmation

- Alice's wallet queries node
- Sees transaction confirmed
- Balance updated
- Transaction complete

**Total Time:**

- Level 1: Instant (single node)
- Level 2: ~5 seconds (PoA consensus)
- Level 3: ~2-5 seconds (Fino-Narwhal-Tusk)

### 7.2 Smart Contract Execution Flow

**User Action:** Alice calls a function on a DEX contract to swap tokens

#### Step 1: Transaction Created

- Alice's wallet creates transaction
- To: DEX contract address
- Data: Function call + parameters
- Fee: Higher (smart contract execution costs more)

#### Step 2: Transaction Reaches Node

- Node receives transaction
- Validates signature
- Adds to mempool

#### Step 3: Validator Executes

- Validator includes in block
- MoveVM loads contract code
- Executes function:
  - Checks Alice's balance
  - Calculates swap amount
  - Updates token balances
  - Transfers tokens
- Records state changes

#### Step 4: State Updated

- Alice's AEQ balance: -100
- Alice's token balance: +95 (after fees)
- DEX contract balance: +5 (fees)
- Liquidity pool updated

#### Step 5: Block Finalized

- Block added to chain
- Transaction confirmed
- Alice sees updated balances

**Gas Cost:**

- Simple transfer: ~21,000 gas
- Smart contract call: ~100,000-1,000,000 gas
- Paid in AEQ

### 7.3 What Happens During Network Congestion?

**Scenario:** Many users sending transactions at once

#### Problem:

- Mempool fills up
- More transactions than can fit in blocks
- Some transactions wait

#### Solution (Level 3):

- **Narwhal**: Handles high throughput
- **Parallel Execution (Level 4)**: Executes multiple transactions simultaneously
- **Fee Market**: Higher fees = faster processing

#### User Experience:

- Low fee transaction: Might wait minutes/hours
- High fee transaction: Processes quickly
- Network scales to handle load

---

## Part VIII: Token Economics Deep Dive {#part-viii}

### 8.1 Initial Token Distribution

**Total Supply:** 1,000,000,000 AEQ (1 billion)

**Distribution Example:**

- **Team/Founders (20%)**: 200,000,000 AEQ
  - Vested over 4 years
  - 25% after 1 year, then monthly
- **Investors (15%)**: 150,000,000 AEQ
  - Vested over 2-3 years
  - Early backers who funded development
- **Community/Airdrop (10%)**: 100,000,000 AEQ
  - Distributed to early supporters
  - Marketing campaigns
  - Developer grants
- **Treasury (15%)**: 150,000,000 AEQ
  - Future development
  - Partnerships
  - Emergency fund
- **Validators/Staking (40%)**: 400,000,000 AEQ
  - Distributed via inflation (5% annually)
  - Rewards for securing network
  - Over ~20 years

### 8.2 Inflation and Deflation

**Inflation:**

- 5% annually
- New AEQ created each year
- Goes to validators
- Incentivizes staking

**Example:**

- Year 1: 1,000,000,000 AEQ
- Year 2: 1,050,000,000 AEQ (+50M)
- Year 3: 1,102,500,000 AEQ (+52.5M)
- Year 4: 1,157,625,000 AEQ (+55.125M)

**Deflation Mechanisms:**

- Transaction fees burned (destroyed)
- Reduces supply
- Can offset inflation

**Example:**

- 1 million transactions/day
- Average fee: 0.001 AEQ
- Daily burn: 1,000 AEQ
- Annual burn: 365,000 AEQ
- If burn > inflation, supply decreases

### 8.3 Token Value Determination

**Factors Affecting AEQ Price:**

1. **Utility:**

   - How useful is Yotquitas?
   - More users = more demand
   - More demand = higher price

2. **Network Effects:**

   - More developers = more apps
   - More apps = more users
   - More users = more value

3. **Speculation:**

   - People buy expecting price to rise
   - Can cause bubbles
   - Can cause crashes

4. **Competition:**

   - Other blockchains may compete
   - Yotquitas' unique features (MEV resistance, speed) = more value
   - Better user experience = more adoption

5. **Regulation:**

   - Government support = price up
   - Government ban = price down

6. **Technology:**
   - Faster = better
   - More secure = better
   - Lower fees = better

### 8.4 Staking Economics

**Staking Requirements:**

- Minimum: 1,000,000 AEQ
- Locked while staking
- Can unstake (with delay)

**Rewards:**

- Transaction fees: Variable (depends on network usage)
- Inflation rewards: ~5% annually
- Total: Depends on network activity

**Example Calculation:**

- Alice stakes 2,000,000 AEQ
- Network processes 1M transactions/day
- Average fee: 0.001 AEQ
- Total fees: 1,000 AEQ/day
- 100 validators share fees
- Alice's share: ~10 AEQ/day from fees
- Plus inflation: ~274 AEQ/year
- Total: ~3,650 AEQ/year
- Annual return: ~0.18% from fees + 5% from inflation = ~5.18%

**Risks:**

- Slashing: Lose stake if misbehave
- Price risk: AEQ price could drop
- Opportunity cost: Can't use staked AEQ

---

## Part IX: Common Questions Answered {#part-ix}

### Q1: How do I actually make money as a user?

**Answer:**

1. **Buy and Hold**: Buy AEQ, price goes up, sell for profit
2. **Staking**: Lock up AEQ, earn rewards
3. **Provide Liquidity**: Earn fees from trading
4. **Yield Farming**: Earn high yields (high risk)
5. **Trading**: Buy low, sell high
6. **Airdrops**: Get free tokens from projects

### Q2: What if I lose my private key?

**Answer:**

- **You lose everything** - no way to recover
- This is why seed phrases are critical
- Store securely (hardware wallet, paper backup)
- Never share with anyone

### Q3: How do I know a smart contract is safe?

**Answer:**

- **Audits**: Check if audited by security firms
- **Open Source**: Review code yourself
- **Reputation**: Check developer history
- **Start Small**: Test with small amounts first
- **Never invest more than you can lose**

### Q4: What's the difference between AEQ and tokens on Yotquitas?

**Answer:**

- **AEQ**: Native token, like the "dollar" of Yotquitas
- **Other Tokens**: Created by smart contracts, like "gift cards"
- AEQ is needed for fees
- Other tokens are for specific applications

### Q5: How fast are transactions really?

**Answer:**

- **Level 1**: Instant (single node, centralized)
- **Level 2**: ~5 seconds (PoA consensus)
- **Level 3**: ~2-5 seconds (Fino-Narwhal-Tusk)
- **Level 4**: ~1-2 seconds (with parallel execution)

### Q6: What happens if validators go offline?

**Answer:**

- Network needs 2/3 of validators online
- If too many offline, network slows/stops
- Validators lose rewards when offline
- Incentivizes staying online

### Q7: Can the government shut down Yotquitas?

**Answer:**

- **Technically**: No (decentralized, no single point of control)
- **Practically**: Can ban in their country
- Can block exchanges
- Can make it illegal to use
- But blockchain itself keeps running

### Q8: How do I convert AEQ back to Naira?

**Answer:**

1. Visit yotquitas.dev/sell
2. Connect your wallet
3. Enter amount of AEQ to sell
4. Confirm transaction
5. Receive NGN via bank transfer (1-3 business days)

**Alternative:**

1. Swap AEQ for stablecoins on Yotquitas DEX
2. Redeem stablecoins for NGN through the portal
3. Faster but requires DEX liquidity

### Q9: What's the difference between a wallet and an exchange?

**Answer:**

- **Wallet**: You control keys, you control money
- **Exchange**: They control keys, they control money
- Wallet: More secure, more responsibility
- Exchange: Easier, less control

### Q10: How much does it cost to use Yotquitas?

**Answer:**

- **Transaction fees**: ~0.001-0.01 AEQ (depends on network load)
- **Smart contract calls**: ~0.01-0.1 AEQ (more complex = more expensive)
- **Deployment**: ~100 AEQ (one-time cost)
- Fees are paid in AEQ only
- Very low cost compared to traditional payment systems

---

## Conclusion

This guide covered the complete lifecycle from end-user to blockchain. You now understand:

- How users interact with blockchains
- How money flows through the system
- How developers monetize
- What happens at launch
- How DeFi works
- The technical flow behind the scenes
- Token economics

**Next Steps:**

- Start building Level 0
- Test with real transactions
- Deploy test contracts
- Experience the user journey yourself

**Remember:** The best way to understand blockchain is to use it. Create a wallet, make transactions, deploy a contract, and experience the full cycle yourself.

---

**Document Version:** 1.0  
**Last Updated:** 2024  
**For:** Yotquitas Development Team
