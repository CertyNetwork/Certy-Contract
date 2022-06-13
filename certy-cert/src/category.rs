use crate::*;
pub type CategoryId = String; //We use 128-bit integers for the category ID, can be changed later

#[derive(BorshDeserialize, Clone, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryMetadata {
    pub title: Option<String>, // ex. "AWS Certified Solutions Architect" or "Negative COVID19 PCR TEST CERTIFICATE"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub issued_at: Option<u64>, // When Category was created, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When Category was last updated, Unix epoch in milliseconds
    pub fields: Option<String>, // Stringified JSON.
    pub extra: Option<String>, // anything extra the Category wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Category {
    //owner of the category
    pub owner_id: AccountId,
}

//The Json category is what will be returned from view calls. 
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonCategory {
    //token ID
    pub category_id: CategoryId,
    //owner of the category
    pub owner_id: AccountId,
    //category metadata
    pub metadata: CategoryMetadata,
}
