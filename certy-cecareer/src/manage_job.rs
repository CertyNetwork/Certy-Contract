use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn job_create(&mut self, job_id: JobId, metadata: JobMetadata) {
        assert_at_least_one_yocto();
        let initial_storage_usage = env::storage_usage();
        self.internal_job_create(job_id, env::predecessor_account_id(), metadata);
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        refund_deposit(required_storage_in_bytes);
    }
    #[payable]
    pub fn job_update(&mut self, job_id: JobId, metadata: JobMetadata) {
        assert_at_least_one_yocto();
        self.assert_job_owner(env::predecessor_account_id(), &job_id);
        let mut job_metadata = self.job_metadata_by_id.get(&job_id).unwrap();
        // updatable fields
        job_metadata.extra = metadata.extra;
        job_metadata.reference = metadata.reference;
        job_metadata.reference_hash = metadata.reference_hash;

        let initial_storage_usage = env::storage_usage();
        self.internal_job_update(&job_id, &job_metadata);
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
    pub fn job_delete(&mut self, job_id: JobId) {
        assert_one_yocto();
        self.assert_job_owner(env::predecessor_account_id(), &job_id);
        let initial_storage_usage = env::storage_usage();
        self.internal_job_delete(job_id);
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
    //Query for all the jobs of an owner
    pub fn jobs_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonJob> {
        let jobs_for_owner_set = self.jobs_per_owner.get(&account_id);
        let jobs = if let Some(jobs_for_owner_set) = jobs_for_owner_set {
            jobs_for_owner_set
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        jobs
            .iter()
            .skip(start as usize)
            .take(limit.unwrap_or(50) as usize)
            .map(|job_id| self.job_info(job_id.clone()).unwrap())
            .collect()
    }
    pub fn job_info(&self, job_id: JobId) -> Option<JsonJob> {
        if let Some(job) = self.jobs_by_id.get(&job_id) {
            let metadata = self.job_metadata_by_id.get(&job_id).unwrap();
            Some(JsonJob {
                job_id,
                owner_id: job.owner_id,
                metadata,
            })
        } else {
            None
        }
    }
}
