use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};

pub use crate::job::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::manage_job::*;

mod job;
mod events;
mod internal;
mod manage_job;


pub const CECAREER_STANDARD_NAME: &str = "cecareer";
pub const CECAREER_VERSION: &str = "0.1.0";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,
    pub jobs_per_owner: LookupMap<AccountId, UnorderedSet<JobId>>,
    pub jobs_by_id: LookupMap<JobId, Job>,
    pub job_metadata_by_id: UnorderedMap<JobId, JobMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    JobsPerOwner,
    JobPerOwnerInner { account_id_hash: CryptoHash },
    JobsById,
    JobMetadataById,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            jobs_per_owner: LookupMap::new(
                StorageKey::JobsPerOwner.try_to_vec().unwrap(),
            ),
            jobs_by_id: LookupMap::new(StorageKey::JobsById.try_to_vec().unwrap()),
            job_metadata_by_id: UnorderedMap::new(
                StorageKey::JobMetadataById.try_to_vec().unwrap(),
            ),
            owner_id,
        };
        this
    }
}
