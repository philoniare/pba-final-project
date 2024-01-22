# FRAME Assignment Instructions

This assignment is a project covering the material you've learned about writing Substrate runtimes with FRAME.
To complete this project, **select one of the following challenges**, and implement the runtime described using FRAME.

The [pallets included](./pallets/) may be used as a base to work in for the option you select.

## Main Challenges

These are the same challenges we have used for the previous academies. We have reviewed and graded many of these projects, and many students have be successful with these projects.

### Challenge 1: Decentralized Exchange

- Use a multi-asset pallet to represent many tokens.
  - You may create your own or use the included `pallet_assets`.
- Create a Uniswap Version 2 style DEX to allow users to trustlessly exchange tokens with one another.
  - Implement rewards to incentivize users to create pools.
  - Expose an API from the pallet which acts as a “price oracle” based on the existing liquidity pools.

#### _Optional_

- Create some kind of asset or marketplace where users can use any token to purchase resources, using the price oracle to make sure users pay enough.
- Integrate other DeFi utilities on top of your DEX.

### Challenge 2: Quadratic Voting

- Create a simple identity system to "de-sybil" your users.
  - Could be integrated into your pallet, a secondary pallet, or use the existing identity pallet provided by FRAME.
- Create a voting system where users with identities can reserve / lock an amount of token, which then weights their vote on a quadratic scale.
- Proposals should be a simple on chain text or hash, votes can be simply aye or nay.

#### _Optional_

- Create a more complex proposal system where users can vote on multiple things at once, and have to consider how they want to distribute their votes across them.
- Allow proposals to dispatch on-chain calls and make changes to your runtime.

### Challenge 3: Direct Delegation Proof of Stake

- The pallet should have logic to manage "validators" and "delegators".
  - Validators register themselves as potential block producers.
  - Any other user can use their tokens to delegate (vote) for the set of validators they want.
- Where every N blocks (a session), the current “winners” are selected, and Aura is updated.
- Block rewards should be given to the current block producer and the delegators who backed them.

#### _Optional_

- Try to support delegation chains, where a delegator can delegate to another delegator.
- Think about and implement some kind of slashing for validators if they “misbehave”.
- Integrate the Session pallet rather than using Aura directly.

## Beta Challenges

These are new challenges that will hopefully rotate in and replace the existing challenges. As such, this is the first time we are asking students to try out these challenges, and there may be some unforeseen issues or complexities when building them. HOWEVER, we would love for you to try them out if they sound interesting to you, and we will take into consideration any of these "complexities" when grading and reviewing.

Be our beta testers, and give us feedback :)

### Beta Challenge 1: Stateful Multisig

- Create a pallet which allows users to create, manage, and submit multisig transactions.
  - In my mind, the original Gnosis Multisig Contract is a good north star: https://github.com/gnosis/MultiSigWallet
- Multisigs should generate a unique address representing that multisig.
- Full transaction lifecycle:
  - Create a new multisig transaction
  - Vote on the outcome of that transaction
  - Submit that transaction
- There should be a sensible process to destroy a multisig when it is no longer needed / used.
- There should be configurable call filter support limiting what multisigs have access to.

#### _Optional_

- Create a basic user interface for managing the multisig
  - I think this is a great project to choose if you want to work on UI/UX
  - This is a nice overview of an end to end experience with Gnosis: https://www.covalenthq.com/docs/unified-api/guides/how-to-set-up-a-gnosis-safe-wallet-step-by-step/
- Create a migration story from stateless multisig on Polkadot to your stateful multisig

### Beta Challenge 2: Free Transaction Pallet

- Create a pallet which can enable users to make transactions without paying transaction fees.
- Free transactions should be based on Weight of the call.
- Users need to lock tokens, which gives them "weight credits". The more they lock, the more credit they get.
- Every time period (let's say 24 hours), users can use up to their weight credit in free transactions. At the end of the time period, their available credits reset.
- The lock period on their tokens is equal to the credit time period, to prevent basic spam attacks.
- Free transaction pallet has a "global limit" to the amount of free transactions it supports in some time period.
  - For example, all users cannot submit a free transaction within the same block, or even the same hour, or something like this.

#### _Optional_

- Additional entry points for accessing free transactions:
  - Fee fallback: try a free transaction, if needed pay the fee (if not enough credits or the global limit is reached)
  - Only free: Use a signed extension to verify the free transaction is possible before actually executing the extrinsic
- Allow someone else to pay the transaction fee for another user
  - One approach: meta transactions https://github.com/paritytech/polkadot-sdk/issues/266

### Beta Challenge 3: Multi-Token Treasury

- Create a pallet which manages a multi-token treasury system.
- Use new/existing governance APIs and/or special origins to manage access to the treasury.
  - Spending tracks similar to open governance.
- Handle the allocation of funds. (treasury spending)
  - Handle cases where there is not enough of some token for a spend.
  - Handle existential deposit logic across the different assets

#### _Optional_

- Support multiple spending periods:
  - For example, some funds upfront, some funds periodically, the rest of the funds once fully complete.
- Support APIs to balance the amount of funds in the treasury.
  - For example, given some token price API, converting DOT to USD within some limit to ensure that enough of each token is present in the treasury.

## Grading

Your implementation will be reviewed for code quality and implementation details.

- Implementation
  - Correctness and accuracy of implementation
  - Evidence of using various techniques used in class
  - As close to production ready as possible
- Code Quality
  - Tests and code coverage
  - Use of best practices and efficient code
  - Well documented, with considerations and compromises noted
- Bonus Points
  - Integrate this into a working node
  - UI to interact with the runtime code
    - Value functionality over beauty
  - Add and run benchmarking and use the generated weights
  - Optional objectives

### Review Process

Here is an overview of how graders will assess your project.

#### 1. README

Graders will start by reviewing your updated [README.md](./README.md) that should describe at a minimum:

- The project option you selected
- Important details, background, and considerations you researched
- How you designed your state transition function
- Any compromises you made, and things you would improve if you had time
- How to run the project

Graders should have no issue in running and testing this project, and be able to understand what to expect to see before reviewing any of your code.

_Optionally:_

- Embedded diagrams
- Video demo of operation and/or walkthrough of source code
- How the different parts function in the context of the whole project
- Considerations of the bigger picture for this project in context of Substrate ecosystem

#### 2. Code

- Graders will scan through your extrinsics and try to map the code your wrote to the state transition function you described in `README.md`.
- Graders will look to make sure you are following best practices and that your code is safe / non-exploitable by malicious actors.
- Finally, graders will look at cleanliness and code quality.

It should go without saying, but graders will also make sure that the code written is your own, and if graders ask, you should be able to explain to them how things are working, and why you chose to implement things in the way you did.

#### 3. Tests

You should have a comprehensive test suite which covers all the successful and "error" flows through your state transition function.

- All of the mainline paths of your state transition function should be covered and shown to execute correctly, with checks of your final state.
- Any paths which are not successful for your state transition function should have well handled errors, and tests should show that is the case.

#### 4. Bonus Points

You should prioritize getting your project working above all else.
However, if you find yourself with extra time and ambition, you will find that all of the project choices have a lot of room for you to design beyond what graders have asked for.
Graders will assess any work and ideas that above and beyond the basic project requirements, including optional objectives.

Use your knowledge of cryptography, game theory, blockchain, and existing useful applications to extend and improve your project.

Make your project as close to "production ready" as possible, for example considering deeply the economics of your system, punishments for bad actors, proper benchmarking, etc....

Consider breaking your project into multiple modular parts which work together and are more reusable.

If you do not have time to actually program all of your ambitions, spend the time to describe what you would like to do, and how you would have done it.
