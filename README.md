## Orderbook Merkle Mountain Range Library

A general implementation of Merkle Mountain Range proofs for Orderbooks 


## Goals 

- Create a general Rust library for generating orderbook proofs using Merkle Mountain Ranges
- Create a RISC0 compatible library 


## Merkle Mountain Range Design 

The design choices around Merkle Mountain Ranges is fairly new, and is largely experimentative. The basic feature of MMR is that it is an append-only data structure with guarantees on
1. Ordering (No order that came after got filled before if the same price)
2. Fair Execution (No worse price got the trade at time of execution)

MMR designs allow for a bunch of choices: 
1. Append on the right only? Or left too? 

2. Updatability of existing elements ir immutable records? 
Immutable records might require extra data structures to hold some information. 

3. What data to reveal in the executions and to what level of orderbook obfuscation is needed. 
prove computation? Or prove privacy?  


## Implementation Attempt

The Merkle Mountain Range library used is: https://github.com/nervosnetwork/merkle-mountain-range

Create orderbook specific merkle mountain range structure and functions as a starting point. 

Workflow: 
1. User submits order received receipt showing leaf position. 
2. Orderbook hidden
3. Orders are matched off chain (off MMR) and executions are revealed.
4. Verification1: call `calculate_root` function using the executions as `proof_items` to check these orders belonged to the tree 
5. Verification2:  
6. Verification3: 




## References 
https://codyx.medium.com/over-the-proofs-a-world-of-trees-merkle-mountain-ranges-edition-%EF%B8%8F-dd4ac0e540fc

