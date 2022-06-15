use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};

pub use crate::category::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::manage_category::*;
pub use crate::manage_cert::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;

mod category;
mod enumeration;
mod events;
mod internal;
mod manage_category;
mod manage_cert;
mod metadata;
mod nft_core;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

pub const CERTY_CERT_STANDARD_NAME: &str = "cecert";
pub const CERTY_CERT_VERSION: &str = "0.1.0";


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    pub count_token_id: u128,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of all the category IDs for a given account
    pub categories_per_owner: LookupMap<AccountId, UnorderedSet<CategoryId>>,

    //keeps track of all the token IDs for a given category
    pub tokens_per_category: LookupMap<CategoryId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the category struct for a given category ID
    pub categories_by_id: LookupMap<CategoryId, Category>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the token metadata for a given token ID
    pub category_metadata_by_id: UnorderedMap<CategoryId, CategoryMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    CategoriesPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    CategoryPerOwnerInner { account_id_hash: CryptoHash },
    TokensPerCategory,
    TokenPerCategoryInner { category_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    CategoriesById,
    CategoryMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "certy-1.0.0".to_string(),
                name: "Certy NFT".to_string(),
                symbol: "Certy".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            categories_per_owner: LookupMap::new(
                StorageKey::CategoriesPerOwner.try_to_vec().unwrap(),
            ),
            tokens_per_category: LookupMap::new(
                StorageKey::TokensPerCategory.try_to_vec().unwrap(),
            ),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            categories_by_id: LookupMap::new(StorageKey::CategoriesById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            category_metadata_by_id: UnorderedMap::new(
                StorageKey::CategoryMetadataById.try_to_vec().unwrap(),
            ),
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            count_token_id: 0,
        };
        this
    }
}
