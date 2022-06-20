use near_sdk::{
    env,
    serde::{Deserialize, Serialize},
    serde_json,
};

use crate::{JobMetadata, JobId};

/// Enum that represents the data type of the EventLog.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[serde(crate = "near_sdk::serde")]
#[non_exhaustive]
pub enum EventLogVariant {
    JobCreate(Vec<JobCreateLog>),
    JobUpdate(Vec<JobUpdateLog>),
    JobDelete(Vec<JobDeleteLog>),
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
        serde_json::to_string(self)
            .ok()
            .unwrap_or_else(|| env::abort())
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

/// An event log to capture job create
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `owner_id`: "owner.near"
/// * `job_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JobCreateLog {
    pub authorized_id: Option<String>,
    pub owner_id: String,
    pub job_ids: Vec<JobId>,
    pub job_metadatas: Vec<JobMetadata>,
}

/// An event log to capture job update
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `job_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JobUpdateLog {
    pub authorized_id: Option<String>,
    pub job_ids: Vec<JobId>,
    pub old_job_metadatas: Vec<JobMetadata>,
    pub new_job_metadatas: Vec<JobMetadata>,
}

/// An event log to capture job delete
///
/// Arguments
/// * `authorized_id`: the account called the method
/// * `job_ids`: ["1", "12345abc"]
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JobDeleteLog {
    pub authorized_id: Option<String>,
    pub job_ids: Vec<JobId>,
}
