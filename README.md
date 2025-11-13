# 🗳️ Soroban Voting Contract

This project implements a fundamental, decentralized voting system on the Stellar/Soroban Testnet. It allows for the creation of unique polls, secure voting, and retrieval of real-time results.

## ⚙️ Contract Usage and Functions

The `VotingContract` exposes four public functions. Interaction is done via the Soroban CLI, typically using the `stellar contract invoke` command.

---

### 1. `create_poll`

Initializes a new poll in the contract's persistent storage.

| Argument | Type | Description |
| :--- | :--- | :--- |
| **creator** | Address | The account address that creates and must authorize the poll. |
| **poll_id** | Symbol | A unique identifier for the poll (e.g., `"POLL1"`). |
| **title** | String | The human-readable name of the poll. |
| **options** | Vec<String> | A list of choices for voters. Initialized with zero votes each. |

**🔐 Authorization:** The `creator` address must sign the transaction.

#### CLI Example (JSON Args Method)

```bash
# Replace <CONTRACT_ID> or 'voting_contract' with your deployed contract ID/alias
# Replace <CREATOR_ADDRESS> with the address of your new_creator account (GB6QW...)
stellar contract invoke \
    --id voting_contract \
    --source-account new_creator \
    --network testnet \
    -- \
    create_poll \
    --args '{"creator":"<CREATOR_ADDRESS>", "poll_id":"POLL1", "title":"Best_Topic_Poll", "options":["Rust", "Scaffolding", "Deployment"]}'
