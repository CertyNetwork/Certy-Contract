use crate::*;
pub type TokenId = String;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mosaics"
    pub symbol: String,            // required, ex. "MOSIAC"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Centralized gateway known to have reliable access to decentralized storage assets referenced by `reference` or `media` URLs
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, Clone, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // free-form description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    //owner of the token
    pub owner_id: AccountId,
    pub category_id: CategoryId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub category_id: CategoryId,
    pub metadata: TokenMetadata,
}

pub trait NonFungibleTokenMetadata {
    fn nft_metadata(&self) -> NFTContractMetadata;
}

#[near_bindgen]
impl NonFungibleTokenMetadata for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
