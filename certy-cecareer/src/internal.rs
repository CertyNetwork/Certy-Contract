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
    //used to make sure the user is the owner of the job
    pub(crate) fn assert_job_owner(&self, owner_id: AccountId, job_id: &JobId) {
        let job = self.jobs_by_id.get(job_id).expect("No Job");
        assert_eq!(owner_id, job.owner_id, "Not job owner");
    }
    //create new job
    pub(crate) fn internal_job_create(
        &mut self,
        job_id: JobId,
        owner_id: AccountId,
        metadata: JobMetadata,
    ) {
        //specify the job struct that contains the owner ID
        let job = Job {
            //set the owner ID equal to the owner ID passed into the function
            owner_id,
        };

        //insert the job ID and job struct and make sure that the job doesn't exist
        assert!(
            self.jobs_by_id
                .insert(&job_id, &job)
                .is_none(),
            "Job already exists"
        );
        let mut job_metadata = metadata.clone();
        job_metadata.issued_at = Some(env::block_timestamp_ms());
        job_metadata.updated_at = Some(env::block_timestamp_ms());
        //insert the token ID and metadata
        self.job_metadata_by_id.insert(&job_id, &job_metadata);
        //call the internal method for adding the job to the owner
        self.internal_job_add_to_owner(&job.owner_id, &job_id);
        
         let job_create_log: EventLog = EventLog {
             standard: CECAREER_STANDARD_NAME.to_string(),
             version: CECAREER_VERSION.to_string(),
             event: EventLogVariant::JobCreate(vec![JobCreateLog {
                 authorized_id: Some(env::predecessor_account_id().to_string()),
                 owner_id: job.owner_id.to_string(),
                 job_ids: vec![job_id.to_string()],
                 job_metadatas: vec![job_metadata],
             }]),
         };
         job_create_log.emit();
    }

    //update job
    pub(crate) fn internal_job_update(
        &mut self,
        job_id: &JobId,
        metadata: &JobMetadata,
    ) {
        let old_job_metadata = self.job_metadata_by_id.get(job_id).unwrap();
        let mut job_metadata = metadata.clone();
        job_metadata.updated_at = Some(env::block_timestamp_ms());
        self.job_metadata_by_id.insert(&job_id, &job_metadata);
        let job_update_log: EventLog = EventLog {
            standard: CECAREER_STANDARD_NAME.to_string(),
            version: CECAREER_VERSION.to_string(),
            event: EventLogVariant::JobUpdate(vec![JobUpdateLog {
                authorized_id: Some(env::predecessor_account_id().to_string()),
                job_ids: vec![job_id.to_string()],
                old_job_metadatas: vec![old_job_metadata],
                new_job_metadatas: vec![job_metadata],
            }]),
        };

        job_update_log.emit();
    }
    //delete job
    pub(crate) fn internal_job_delete(&mut self, job_id: JobId) {
        self.internal_job_remove_from_owner(
            &self.jobs_by_id.get(&job_id).unwrap().owner_id,
            &job_id,
        );
        self.jobs_by_id.remove(&job_id);
        self.job_metadata_by_id.remove(&job_id);
        let job_delete_log: EventLog = EventLog {
            standard: CECAREER_STANDARD_NAME.to_string(),
            version: CECAREER_VERSION.to_string(),
            event: EventLogVariant::JobDelete(vec![JobDeleteLog {
                authorized_id: Some(env::predecessor_account_id().to_string()),
                job_ids: vec![job_id.to_string()],
            }]),
        };

        job_delete_log.emit();
    }
   
    //add a job to the set of jobs an owner has
    pub(crate) fn internal_job_add_to_owner(
        &mut self,
        account_id: &AccountId,
        job_id: &JobId,
    ) {
        let mut jobs_set = self
            .jobs_per_owner
            .get(account_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::JobPerOwnerInner {
                        account_id_hash: hash_account_id(&account_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        jobs_set.insert(job_id);
        self.jobs_per_owner
            .insert(account_id, &jobs_set);
    }
    //remove a job from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_job_remove_from_owner(
        &mut self,
        account_id: &AccountId,
        job_id: &JobId,
    ) {
        let mut jobs_set = self
            .jobs_per_owner
            .get(account_id)
            .expect("Job should be owned by the sender");
        jobs_set.remove(job_id);

        if jobs_set.is_empty() {
            self.jobs_per_owner.remove(account_id);
        } else {
            self.jobs_per_owner
                .insert(account_id, &jobs_set);
        }
    }
}
