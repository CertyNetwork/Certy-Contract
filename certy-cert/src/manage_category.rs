use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn category_create(&mut self, metadata: CategoryMetadata) {
        assert_at_least_one_yocto();
        let initial_storage_usage = env::storage_usage();
        self.internal_category_create(env::predecessor_account_id(), metadata);
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }
    #[payable]
    pub fn category_update(&mut self, category_id: CategoryId, metadata: CategoryMetadata) {
        assert_at_least_one_yocto();
        self.assert_category_owner(env::predecessor_account_id(), &category_id);
        let mut category_metadata = self.category_metadata_by_id.get(&category_id).unwrap();
        // updatable fields
        category_metadata.title = metadata.title;
        category_metadata.description = metadata.description;
        category_metadata.media = metadata.media;
        category_metadata.media_hash = metadata.media_hash;
        category_metadata.extra = metadata.extra;
        category_metadata.reference = metadata.reference;
        category_metadata.reference_hash = metadata.reference_hash;

        let initial_storage_usage = env::storage_usage();
        self.internal_category_update(&category_id, &category_metadata);
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
    // pub fn category_transfer_ownership(
    //     &mut self,
    //     category_id: CategoryId,
    //     new_owner_id: AccountId,
    // ) {
    //TO-DO
    // }
    #[payable]
    pub fn category_delete(&mut self, category_id: CategoryId) {
        assert_one_yocto();
        self.assert_category_owner(env::predecessor_account_id(), &category_id);
        let initial_storage_usage = env::storage_usage();
        self.internal_category_delete(category_id);
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
    //Query for all the categories of an owner
    pub fn categories_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonCategory> {
        let categories_for_owner_set = self.categories_per_owner.get(&account_id);
        let categories = if let Some(categories_for_owner_set) = categories_for_owner_set {
            categories_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        categories
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|category_id| self.category_info(category_id.clone()).unwrap())
            .collect()
    }
    pub fn category_info(&self, category_id: CategoryId) -> Option<JsonCategory> {
        if let Some(category) = self.categories_by_id.get(&category_id) {
            let metadata = self.category_metadata_by_id.get(&category_id).unwrap();
            Some(JsonCategory {
                category_id,
                owner_id: category.owner_id,
                metadata,
            })
        } else {
            None
        }
    }
}
