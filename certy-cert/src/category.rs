use crate::*;
pub type CategoryId = String; //We use 128-bit integers for the category ID, can be changed later

#[derive(BorshDeserialize, Clone, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // ex. "AWS Certified Solutions Architect" or "Negative COVID19 PCR TEST CERTIFICATE"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // free-form description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<u64>, // When Category was created, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<u64>, // When Category was last updated, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>, // Stringified JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>, // anything extra the Category wants to store on-chain. Can be stringified JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    #[serde(skip_serializing_if = "Option::is_none")]
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
