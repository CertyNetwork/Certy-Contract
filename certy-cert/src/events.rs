use near_sdk::{serde::{Deserialize, Serialize}, serde_json, env};

use crate::CategoryMetadata;

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    NftMint(Vec<NftMintLog>),
    NftTransfer(Vec<NftTransferLog>),
    CategoryCreate(Vec<CategoryCreateLog>),
    CategoryUpdate(Vec<CategoryUpdateLog>),
    CategoryDelete(Vec<CategoryDeleteLog>),
}

/// Interface to capture data about an event
///
/// Arguments:
/// * `standard`: name of standard e.g. nep171
/// * `version`: e.g. 1.0.0
/// * `event`: associate event data
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EventLog {
    pub standard: String,
    pub version: String,

    // `flatten` to not have "event": {<EventLogVariant>} in the JSON, just have the contents of {<EventLogVariant>}.
    #[serde(flatten)]
    pub event: EventLogVariant,
}

impl EventLog {
    fn to_json_string(&self) -> String {
        // Events cannot fail to serialize so fine to panic on error
        #[allow(clippy::redundant_closure)]
        serde_json::to_string(self).ok().unwrap_or_else(|| env::abort())
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub(crate) fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}

/// An event log to capture token minting
///
/// Arguments
/// * `owner_id`: "account.near"
/// * `token_ids`: ["1", "abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftMintLog {
    pub owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture token transfer
///
/// Arguments
/// * `authorized_id`: approved account to transfer
/// * `old_owner_id`: "owner.near"
/// * `new_owner_id`: "receiver.near"
/// * `token_ids`: ["1", "12345abc"]
/// * `memo`: optional message
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct NftTransferLog {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_id: Option<String>,

    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

/// An event log to capture category create
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `owner_id`: "owner.near"
/// * `category_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryCreateLog {
    pub authorized_id: Option<String>,
    pub owner_id: String,
    pub category_ids: Vec<String>,
    pub category_metadatas: Vec<CategoryMetadata>,
}

/// An event log to capture category update
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `category_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryUpdateLog {
    pub authorized_id: Option<String>,
    pub category_ids: Vec<String>,
    pub old_category_metadatas: Vec<CategoryMetadata>,
    pub new_category_metadatas: Vec<CategoryMetadata>,
}

/// An event log to capture category delete
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `category_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoryDeleteLog {
    pub authorized_id: Option<String>,
    pub category_ids: Vec<String>,
}