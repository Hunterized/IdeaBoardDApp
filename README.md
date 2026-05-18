# IdeaBoard DApp

**IdeaBoard DApp** — a blockchain-based decentralized idea submission and voting system

## Project Description

IdeaBoard DApp is a decentralized smart contract built on the Stellar blockchain using the Soroban SDK. It allows users to submit ideas, organize them by category, vote on them, leave comments, edit their own submissions, and close voting when an idea is finalized.

Each idea is stored on-chain with a unique ID and managed through smart contract functions. This removes the need for a centralized database and makes the idea board transparent, tamper-resistant, and fully controlled by contract logic.

## Project Vision

Our vision is to create a trustworthy, community-driven idea governance platform by:

- **Decentralizing idea management**: storing proposals directly on-chain instead of in a centralized server
- **Giving users ownership**: letting authors manage their own ideas through authenticated wallet actions
- **Supporting open discussion**: allowing comments and feedback on each idea
- **Enabling fair voting**: preventing duplicate votes and supporting both upvotes and downvotes
- **Providing controlled finalization**: allowing idea authors to close voting when an idea is ready

We want IdeaBoard DApp to feel like a practical on-chain discussion and proposal system rather than a simple post list.

## Key Features

### 1. **Idea Creation**
- Create a new idea with a title, description, and category
- Automatically assign a unique on-chain ID
- Store the idea permanently in contract storage
- Require wallet authorization from the author

### 2. **Idea Retrieval**
- Fetch all active ideas from storage
- Retrieve full idea data including votes and comments
- Keep an indexed list of idea IDs for efficient access

### 3. **Idea Editing**
- Allow only the original author to edit their idea
- Update title, description, and category
- Require authentication for security

### 4. **Voting System**
- Support upvotes, downvotes, and vote removal
- Prevent duplicate voting from the same wallet
- Store votes using a wallet-to-vote map
- Allow authors to close voting for an idea

### 5. **Comments**
- Add comments to ideas
- Store comment author and content on-chain
- Require wallet authorization for comment submission

### 6. **Idea Deletion**
- Allow only the author to delete their own idea
- Remove the idea from individual storage
- Remove the idea ID from the active idea list

## Contract Logic Overview

The contract uses three main storage keys:

- `Idea(u64)` — stores a single idea by its unique ID
- `IdeaList` — stores the list of active idea IDs
- `IdeaCounter` — keeps track of the next available ID

Each idea contains:

- `id`
- `title`
- `description`
- `category`
- `author`
- `votes` — a `Map<Address, i32>` for upvotes, downvotes, and vote removal
- `comments` — a `Vec<Comment>`
- `is_voting_closed` — a boolean that blocks further voting when enabled

## Future Scope

### Short-Term Enhancements
1. **Open voting and closing rules**  
   Add more voting permissions, such as allowing only the author or a moderator to close voting.

2. **Comment management**  
   Add comment deletion or comment editing for better moderation.

3. **Idea filtering**  
   Filter ideas by category, author, or voting status.

4. **Vote score helper**  
   Add a function to calculate total vote score from the vote map.

### Medium-Term Enhancements
5. **Idea moderation**
   Add admin or community moderation tools for spam and abuse control.

6. **Pinned or featured ideas**
   Highlight important ideas on the front page.

7. **Reputation system**
   Reward active authors and voters with reputation points.

8. **Idea version history**
   Store previous versions of edited ideas for transparency.

### Long-Term Vision
9. **Threaded discussions**
   Support nested replies under comments.

10. **DAO-style governance**
   Turn the idea board into a full decentralized proposal and voting platform.

11. **Cross-contract integration**
   Allow other contracts to reference or interact with submitted ideas.

12. **Frontend integration**
   Build a user interface for submitting, voting, and commenting on ideas.

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network

## Available Contract Functions

- `create_idea()` — create a new idea
- `get_ideas()` — retrieve all stored ideas
- `edit_idea()` — edit an existing idea
- `cast_vote()` — upvote, downvote, or remove a vote
- `close_voting()` — close voting for an idea
- `add_comment()` — add a comment to an idea
- `delete_idea()` — delete an idea owned by the caller

---

**IdeaBoard DApp** — Decentralized idea submission, discussion, and voting on Stellar

ID SmartContract: CC7NKCFR27SI6H2LGVXYCFBXW4CD2Z6DPEIKIJN2RRUYIJDQQMIZS2FX