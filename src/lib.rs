use ckb_merkle_mountain_range::{
    Error, Result, leaf_index_to_mmr_size, leaf_index_to_pos, Merge, MerkleProof, MMR,
    MMRStoreReadOps, MMRStoreWriteOps,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

// Define the two sides of an order: Bid and Ask

#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Bid,
    Ask,
}


// Define the Order struct with quantity, price, and side
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub quantity: u64,
    pub price: u64,
    pub side: Side,
}

// Implement the constructor for MyMMRStore
pub struct MyMMRStore<Elem> {
    storage: HashMap<u64, Elem>,
}

// Implement the MMRStoreReadOps trait for MyMMRStore
impl<Elem: Clone> MyMMRStore<Elem> {
    pub fn new() -> Self {
        MyMMRStore {
            storage: HashMap::new(),
        }
    }
}

// Implement the MMRStoreReadOps trait for MyMMRStore
impl<Elem: Clone> MMRStoreReadOps<Elem> for MyMMRStore<Elem> {
    fn get_elem(&self, pos: u64) -> Result<Option<Elem>> {
        Ok(self.storage.get(&pos).cloned())
    }
}

// Implement the MMRStoreWriteOps trait for MyMMRStore
impl<Elem: Clone> MMRStoreWriteOps<Elem> for MyMMRStore<Elem> {
    fn append(&mut self, pos: u64, elems: Vec<Elem>) -> Result<()> {
        for (index, elem) in elems.into_iter().enumerate() {
            self.storage.insert(pos + index as u64, elem);
        }
        Ok(())
    }
}


// Define the AddMerge struct for custom merge strategy

#[derive(Debug)]
pub struct AddMerge;

// Implement the Merge trait for AddMerge
impl Merge for AddMerge {
    type Item = Order;

    // Merge two Order structs using the custom strategy (SHA256)
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

        // Finalize the hash and extract new quantity and price values
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

