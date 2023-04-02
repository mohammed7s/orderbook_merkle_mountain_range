use ckb_merkle_mountain_range::{Error, Result};
use ckb_merkle_mountain_range::{leaf_index_to_mmr_size, leaf_index_to_pos};
use ckb_merkle_mountain_range::Merge;
use ckb_merkle_mountain_range::{MerkleProof, MMR};
use ckb_merkle_mountain_range::{MMRStoreReadOps, MMRStoreWriteOps};
use std::collections::HashMap; 
mod lib; 
use lib::MyMMRStore;
use lib::Order;
use lib::Side;
use lib::AddMerge;
fn main() {

    // #[derive(Debug, Clone, PartialEq)]
    // pub enum Side {
    //     Bid,
    //     Ask,
    // }
    
    // #[derive(Debug, Clone)]
    // pub struct Order {
    //     quantity: u64,
    //     price: u64,
    //     side: Side,
    // }
    

    let store: MyMMRStore<Order>= MyMMRStore::new();
    
    let mut ob: MMR<Order, AddMerge, MyMMRStore<Order>> = MMR::new(0u64,store);
    
    let elem_to_add  = Order{
        quantity: 14,
        price:1567,
        side: Side::Bid
    };    

    let new_order_position = ob.push(elem_to_add); 
    match new_order_position {
        Ok(position) => {
            println!("Element added at position: {}", position);
        }
        Err(e) => {
            println!("Error adding element: {:?}", e);
        }
    }



    let positions = vec![0];
    let result = ob.gen_proof(positions); 
    match result {
        Ok(merkle_proof) => {
            println!("Generated Merkle proof: {:?}", merkle_proof);
        }
        Err(e) => {
            println!("Error generating Merkle proof: {:?}", e);
        }
    }


    let mmr_size = 10;
    let proof_items = vec![
        Order {
            quantity: 1,
            price: 1000,
            side: Side::Bid,
        },
        Order {
            quantity: 2,
            price: 2000,
            side: Side::Bid,
    },
];

let merkle_proof: MerkleProof<Order, AddMerge> = MerkleProof::new(mmr_size, proof_items);




}

