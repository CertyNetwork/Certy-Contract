use crate::*;
use near_sdk::CryptoHash;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_category_id(category_id: &CategoryId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(category_id.as_bytes()));
    hash
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    )
}

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    )
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    //used to make sure the user is the owner of the category
    pub(crate) fn assert_cert_provider(&self, account_id: AccountId, token_id: &TokenId) {
        let token = self.tokens_by_id.get(token_id).expect("No token");
        let category = self
            .categories_by_id
            .get(&token.category_id)
            .expect("No Category");
        assert_eq!(account_id, category.owner_id, "Not cert provider");
    }
    //used to make sure the user is the owner of the category
    pub(crate) fn assert_category_owner(&self, owner_id: AccountId, category_id: &CategoryId) {
        let category = self.categories_by_id.get(category_id).expect("No Category");
        assert_eq!(owner_id, category.owner_id, "Not category owner");
    }
    //create new category
    pub(crate) fn internal_category_create(
        &mut self,
        category_id: CategoryId,
        owner_id: AccountId,
        metadata: CategoryMetadata,
    ) {
        //specify the category struct that contains the owner ID
        let category = Category {
            //set the owner ID equal to the owner ID passed into the function
            owner_id,
        };

        //insert the category ID and category struct and make sure that the category doesn't exist
        assert!(
            self.categories_by_id
                .insert(&category_id, &category)
                .is_none(),
            "Category already exists"
        );
        let mut category_metadata = metadata.clone();
        category_metadata.issued_at = Some(env::block_timestamp_ms());
        category_metadata.updated_at = Some(env::block_timestamp_ms());
        //insert the token ID and metadata
        self.category_metadata_by_id
            .insert(&category_id, &category_metadata);
        //call the internal method for adding the category to the owner
        self.internal_category_add_to_owner(&category.owner_id, &category_id);
        let category_create_log: EventLog = EventLog {
            standard: CERTY_CERT_STANDARD_NAME.to_string(),
            version: CERTY_CERT_VERSION.to_string(),
            event: EventLogVariant::CategoryCreate(vec![CategoryCreateLog {
                authorized_id: Some(env::predecessor_account_id().to_string()),
                owner_id: category.owner_id.to_string(),
                category_ids: vec![category_id.to_string()],
            }]),
        };
        env::log_str(&category_create_log.to_string());
    }

    //update category
    pub(crate) fn internal_category_update(
        &mut self,
        category_id: &CategoryId,
        metadata: &CategoryMetadata,
    ) {
        let mut category_metadata = metadata.clone();
        category_metadata.updated_at = Some(env::block_timestamp_ms());
        self.category_metadata_by_id
            .insert(&category_id, &category_metadata);
        let category_update_log: EventLog = EventLog {
            standard: CERTY_CERT_STANDARD_NAME.to_string(),
            version: CERTY_CERT_VERSION.to_string(),
            event: EventLogVariant::CategoryUpdate(vec![CategoryUpdateLog {
                authorized_id: Some(env::predecessor_account_id().to_string()),
                category_ids: vec![category_id.to_string()],
            }]),
        };

        env::log_str(&category_update_log.to_string());
    }
    //delete category
    pub(crate) fn internal_category_delete(&mut self, category_id: CategoryId) {
        assert!(
            !self.tokens_per_category.contains_key(&category_id),
            "Not empty category"
        );
        self.internal_category_remove_from_owner(
            &self.categories_by_id.get(&category_id).unwrap().owner_id,
            &category_id,
        );
        self.categories_by_id.remove(&category_id);
        self.category_metadata_by_id.remove(&category_id);
        let category_delete_log: EventLog = EventLog {
            standard: CERTY_CERT_STANDARD_NAME.to_string(),
            version: CERTY_CERT_VERSION.to_string(),
            event: EventLogVariant::CategoryDelete(vec![CategoryDeleteLog {
                authorized_id: Some(env::predecessor_account_id().to_string()),
                category_ids: vec![category_id.to_string()],
            }]),
        };

        env::log_str(&category_delete_log.to_string());
    }
    //mint new token
    pub(crate) fn internal_mint_token(
        &mut self,
        receiver_id: AccountId,
        metadata: TokenMetadata,
        category_id: CategoryId,
    ) {
        //measure the initial storage being used on the contract
        let token_id: TokenId = self.count_token_id.to_string();

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            category_id,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );
        self.count_token_id += 1;
        let mut cert_metadata = metadata.clone();
        cert_metadata.issued_at = Some(env::block_timestamp_ms());
        cert_metadata.updated_at = Some(env::block_timestamp_ms());
        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &cert_metadata);

        //call the internal method for adding the token to the owner
        self.internal_token_add_to_owner(&token.owner_id, &token_id);

        //call the internal method for adding the token to the category
        self.internal_token_add_to_category(&token.category_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());
    }
    //update token
    pub(crate) fn internal_token_update(&mut self, token_id: &TokenId, metadata: &TokenMetadata) {
        let mut cert_metadata = metadata.clone();
        cert_metadata.updated_at = Some(env::block_timestamp_ms());
        self.token_metadata_by_id.insert(&token_id, &cert_metadata);
        //TO-DO: add the event log.
    }
    //add a category to the set of categories an owner has
    pub(crate) fn internal_category_add_to_owner(
        &mut self,
        account_id: &AccountId,
        category_id: &CategoryId,
    ) {
        //get the set of tokens for the given account
        let mut categories_set = self
            .categories_per_owner
            .get(account_id)
            .unwrap_or_else(|| {
                //if the account doesn't have any tokens, we create a new unordered set
                UnorderedSet::new(
                    StorageKey::CategoryPerOwnerInner {
                        //we get a new unique prefix for the collection
                        account_id_hash: hash_account_id(&account_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        //we insert the token ID into the set
        categories_set.insert(category_id);

        //we insert that set for the given account ID.
        self.categories_per_owner
            .insert(account_id, &categories_set);
    }
    //delete token
    pub(crate) fn internal_token_delete(&mut self, token_id: TokenId) {
        let token = self.tokens_by_id.get(&token_id).expect("No Token");
        self.internal_token_remove_from_owner(&token.owner_id, &token_id);
        self.internal_token_remove_from_category(&token.category_id, &token_id);
        self.tokens_by_id.remove(&token_id);
        self.token_metadata_by_id.remove(&token_id);
        //TO-DO: add the event log.
    }
    //remove a category from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_category_remove_from_owner(
        &mut self,
        account_id: &AccountId,
        category_id: &CategoryId,
    ) {
        //we get the set of tokens that the owner has
        let mut categories_set = self
            .categories_per_owner
            .get(account_id)
            //if there is no set of categories for the owner, we panic with the following message:
            .expect("Category should be owned by the sender");
        //we remove the the token_id from the set of tokens
        categories_set.remove(category_id);

        //if the category set is now empty, we remove the owner from the categories_per_owner collection
        if categories_set.is_empty() {
            self.categories_per_owner.remove(account_id);
        } else {
            //if the category set is not empty, we simply insert it back for the account ID.
            self.categories_per_owner
                .insert(account_id, &categories_set);
        }
    }

    //add a token to the set of tokens a category has
    pub(crate) fn internal_token_add_to_category(
        &mut self,
        category_id: &CategoryId,
        token_id: &TokenId,
    ) {
        //get the set of tokens for the given category
        let mut tokens_set = self
            .tokens_per_category
            .get(category_id)
            .unwrap_or_else(|| {
                //if the category doesn't have any tokens, we create a new unordered set
                UnorderedSet::new(
                    StorageKey::TokenPerCategoryInner {
                        //we get a new unique prefix for the collection
                        category_id_hash: hash_category_id(&category_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });

        //we insert the token ID into the set
        tokens_set.insert(token_id);

        //we insert that set for the given category ID.
        self.tokens_per_category.insert(category_id, &tokens_set);
    }
    //remove a token from a category (internal method and can't be called directly via CLI).
    pub(crate) fn internal_token_remove_from_category(
        &mut self,
        category_id: &CategoryId,
        token_id: &TokenId,
    ) {
        //we get the set of tokens that the category has
        let mut tokens_set = self
            .tokens_per_category
            .get(category_id)
            //if there is no set of tokens for the category, we panic with the following message:
            .expect("Token should be in the category");

        //we remove the the token_id from the set of tokens
        tokens_set.remove(token_id);

        //if the token set is now empty, we remove the category from the tokens_per_category collection
        if tokens_set.is_empty() {
            self.tokens_per_category.remove(category_id);
        } else {
            //if the token set is not empty, we simply insert it back for the category ID.
            self.tokens_per_category.insert(category_id, &tokens_set);
        }
    }
    //add a token to the set of tokens an owner has
    pub(crate) fn internal_token_add_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        //get the set of tokens for the given account
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::TokenPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        tokens_set.insert(token_id);

        //we insert that set for the given account ID.
        self.tokens_per_owner.insert(account_id, &tokens_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_token_remove_from_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ) {
        //we get the set of tokens that the owner has
        let mut tokens_set = self
            .tokens_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Token should be owned by the sender");

        //we remove the the token_id from the set of tokens
        tokens_set.remove(token_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if tokens_set.is_empty() {
            self.tokens_per_owner.remove(account_id);
        } else {
            //if the token set is not empty, we simply insert it back for the account ID.
            self.tokens_per_owner.insert(account_id, &tokens_set);
        }
    }

    //transfers the NFT to the receiver_id (internal method and can't be called directly via CLI).
    pub(crate) fn internal_transfer(
        &mut self,
        receiver_id: &AccountId,
        token_id: &TokenId,
        memo: Option<String>,
    ) -> Token {
        //get the token object by passing in the token_id
        let token = self.tokens_by_id.get(token_id).expect("No token");

        //we make sure that the sender isn't sending the token to themselves
        assert_ne!(
            &token.owner_id, receiver_id,
            "The token owner and the receiver should be different"
        );

        //we remove the token from it's current owner's set
        self.internal_token_remove_from_owner(&token.owner_id, token_id);
        //we then add the token to the receiver_id's set
        self.internal_token_add_to_owner(receiver_id, token_id);

        //we create a new token struct
        let new_token = Token {
            owner_id: receiver_id.clone(),
            category_id: token.category_id.clone(),
        };
        //insert that new token into the tokens_by_id, replacing the old entry
        self.tokens_by_id.insert(token_id, &new_token);

        //if there was some memo attached, we log it.
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        // Default the authorized ID to be None for the logs.
        let authorized_id = None;

        // Construct the transfer log as per the events standard.
        let nft_transfer_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                // The optional authorized account ID to transfer the token on behalf of the old owner.
                authorized_id,
                // The old owner's account ID.
                old_owner_id: token.owner_id.to_string(),
                // The account ID of the new owner of the token.
                new_owner_id: receiver_id.to_string(),
                // A vector containing the token IDs as strings.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_transfer_log.to_string());

        //return the preivous token object that was transferred.
        token
    }
}
