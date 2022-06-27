use crate::*;
pub type JobId = String; //We use 128-bit integers for the job ID, can be changed later

#[derive(BorshDeserialize, Clone, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JobMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<u64>, // When Job was created, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<u64>, // When Job was last updated, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>, // anything extra the Job wants to store on-chain. Can be stringified JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Job {
    //owner of the job
    pub owner_id: AccountId,
}

//The Json job is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonJob {
    //token ID
    pub job_id: JobId,
    //owner of the job
    pub owner_id: AccountId,
    //job metadata
    pub metadata: JobMetadata,
}
