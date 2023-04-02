use ckb_merkle_mountain_range::{Error, Result};
use ckb_merkle_mountain_range::{leaf_index_to_mmr_size, leaf_index_to_pos};
use ckb_merkle_mountain_range::Merge;
use ckb_merkle_mountain_range::{MerkleProof, MMR};
use ckb_merkle_mountain_range::{MMRStoreReadOps, MMRStoreWriteOps};

use std::collections::HashMap; 
use sha2::{Digest, Sha256};


#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub quantity: u64,
    pub price: u64,
    pub side: Side,
}


pub struct MyMMRStore<Elem> {
    storage: HashMap<u64, Elem>,
}

impl<Elem: Clone> MyMMRStore<Elem> {
    pub fn new() -> Self {
        MyMMRStore {
            storage: HashMap::new(),
        }
    }
}

impl<Elem: Clone> MMRStoreReadOps<Elem> for MyMMRStore<Elem> {
    fn get_elem(&self, pos: u64) -> Result<Option<Elem>> {
        Ok(self.storage.get(&pos).cloned())
    }
}

impl<Elem: Clone> MMRStoreWriteOps<Elem> for MyMMRStore<Elem> {
    fn append(&mut self, pos: u64, elems: Vec<Elem>) -> Result<()> {
        for (index, elem) in elems.into_iter().enumerate() {
            self.storage.insert(pos + index as u64, elem);
        }
        Ok(())
    }
}



#[derive(Debug)]
pub struct AddMerge;

impl Merge for AddMerge {
    type Item = Order;

    fn merge(left: &Self::Item, right: &Self::Item) -> Result<Self::Item> {
        if left.side != right.side {
            return Err(Error::MergeError("Merging different order sides is not supported".into()));
        }

        let mut hasher = Sha256::new();
        
        hasher.update(&left.quantity.to_le_bytes());
        hasher.update(&right.quantity.to_le_bytes());
        hasher.update(&left.price.to_le_bytes());
        hasher.update(&right.price.to_le_bytes());

        let left_side = match left.side {
            Side::Bid => 0,
            Side::Ask => 1,
        };
        let right_side = match right.side {
            Side::Bid => 0,
            Side::Ask => 1,
        };

        hasher.update([left_side]);
        hasher.update([right_side]);

        let hash = hasher.finalize();
        let new_quantity = u64::from_le_bytes(hash[0..8].try_into().unwrap());
        let new_price = u64::from_le_bytes(hash[8..16].try_into().unwrap());

        Ok(Order {
            quantity: new_quantity,
            price: new_price,
            side: left.side.clone(),
        })
    }
}
