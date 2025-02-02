use candid::{CandidType, Deserialize, Principal};
// use std::borrow::Borrow;
// use serde::de::value::Error;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct CompanyProfile {
    pub id: Option<Principal>,
    pub name: String,
    pub logo: String,
    pub twitter: String,
    pub website: String,
    pub created_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct CompanyParams {
    pub name: String,
    pub logo: String,
    pub twitter: String,
    pub website: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ApplicantProfile {
    pub id: Option<Principal>,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub bio: String,
    pub created_at: u64,

    // how to represent BTreeMap in candid?
    pub skills: BTreeMap<u16, Skill>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ApplicantProfileResponse {
    pub id: Option<Principal>,
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub bio: String,
    pub created_at: u64,
    pub skills: Vec<Skill>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ApplicantParams {
    pub first_name: String,
    pub last_name: String,
    pub nickname: String,
    pub bio: String,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Skill {
    pub id: Option<u16>,
    pub name: String,
}

#[derive(Clone, Default, CandidType, Deserialize)]
pub struct Job {
    pub id: u64,
    pub company_id: Option<Principal>,
    pub position: String,
    pub description: String,
    pub bounty: u64,
    pub status: JobStatus,

    // how to represent BTreeMap in candid?
    pub required_skills: BTreeMap<u16, Skill>,
}

#[derive(Clone, Default, CandidType, Deserialize)]
pub struct JobResponse {
    pub id: u64,
    pub company_id: Option<Principal>,
    pub position: String,
    pub description: String,
    pub bounty: u64,
    pub status: JobStatus,
    pub required_skills: Vec<Skill>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct JobParams {
    pub position: String,
    pub description: String,
    pub bounty: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Application {
    pub id: u64,
    pub applicant_id: Option<Principal>,
    pub job_id: u64,
    pub contact_email: String,
    pub status: ApplicationStatus,
    pub salary_from: u64,
    pub salary_to: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct ApplicationParams {
    pub job_id: u64,
    pub contact_email: String,
    pub salary_from: u64,
    pub salary_to: u64,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum JobStatus {
    Open,
    Closed,
    Canceled,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
pub enum ApplicationStatus {
    Applied,
    Withdraw,
    Offer,
    Accepted,
    Rejected,
}

impl Default for JobStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl Default for ApplicationStatus {
    fn default() -> Self {
        Self::Applied
    }
}
