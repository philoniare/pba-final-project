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
