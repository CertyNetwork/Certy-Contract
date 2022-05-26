use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        metadata: TokenMetadata,
        receiver_id: AccountId,
        category_id: CategoryId,
    ) {
        assert_at_least_one_yocto();
        self.assert_category_owner(env::predecessor_account_id(), &category_id);
        let initial_storage_usage = env::storage_usage();
        self.internal_mint_token(receiver_id, metadata, category_id);
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        refund_deposit(required_storage_in_bytes);
    }
    #[payable]
    pub fn nft_bulk_mint(
        &mut self,
        metadatas: Vec<TokenMetadata>,
        receiver_ids: Vec<AccountId>,
        category_id: CategoryId,
    ) {
        assert_at_least_one_yocto();
        self.assert_category_owner(env::predecessor_account_id(), &category_id);
        assert_eq!(
            metadatas.len(),
            receiver_ids.len(),
            "Metadatas and receiver_ids must be the same length"
        );
        let initial_storage_usage = env::storage_usage();
        for (pos, receiver_id) in receiver_ids.iter().enumerate() {
            self.internal_mint_token(
                receiver_id.clone(),
                metadatas[pos].clone(),
                category_id.clone(),
            );
        }
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        refund_deposit(required_storage_in_bytes);
    }
    #[payable]
    pub fn cert_update(&mut self, metadata: TokenMetadata, token_id: TokenId) {
        assert_at_least_one_yocto();
        self.assert_cert_provider(env::predecessor_account_id(), &token_id);
        let initial_storage_usage = env::storage_usage();
        let mut cert_metadata = self.token_metadata_by_id.get(&token_id).unwrap();
        // updatable fields
        cert_metadata.title = metadata.title;
        cert_metadata.description = metadata.description;
        cert_metadata.media = metadata.media;
        cert_metadata.media_hash = metadata.media_hash;
        cert_metadata.expires_at = metadata.expires_at;
        cert_metadata.starts_at = metadata.starts_at;
        cert_metadata.extra = metadata.extra;
        cert_metadata.reference = metadata.reference;
        cert_metadata.reference_hash = metadata.reference_hash;


        self.internal_token_update(&token_id, &cert_metadata);
        let mut required_storage_in_bytes = 0;
        if env::storage_usage() < initial_storage_usage {
            let released_storage = initial_storage_usage - env::storage_usage();
            Promise::new(env::predecessor_account_id())
                .transfer(Balance::from(released_storage) * env::storage_byte_cost());
        } else {
            required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        }
        refund_deposit(required_storage_in_bytes);
    }
    #[payable]
    pub fn cert_delete(&mut self, token_id: TokenId) {
        assert_one_yocto();
        self.assert_cert_provider(env::predecessor_account_id(), &token_id);
        let initial_storage_usage = env::storage_usage();

        self.internal_token_delete(token_id);

        let mut required_storage_in_bytes = 0;
        if env::storage_usage() < initial_storage_usage {
            let released_storage = initial_storage_usage - env::storage_usage();
            Promise::new(env::predecessor_account_id())
                .transfer(Balance::from(released_storage) * env::storage_byte_cost());
        } else {
            required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        }
        refund_deposit(required_storage_in_bytes);
    }
    //Cert by category
    pub fn cert_get_by_category(
        &self,
        category_id: CategoryId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_of_category_set = self.tokens_per_category.get(&category_id);
        let tokens = if let Some(tokens_of_category_set) = tokens_of_category_set {
            tokens_of_category_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        tokens
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            .collect()
    }
}
