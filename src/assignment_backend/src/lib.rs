use candid::types::principal;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::ManualReply, query, update};
use std::borrow::Borrow;
// use serde::de::value::Error;
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;

// type IdStore = BTreeMap<String, Principal>;
// type ProfileTypeStore = BTreeMap<Principal, UserType>;
type ApplicantProfileStore = BTreeMap<Principal, ApplicantProfile>;
type CompanyProfileStore = BTreeMap<Principal, CompanyProfile>;

type JobStore = BTreeMap<u128, Job>;
type ApplicationStore = BTreeMap<u128, Application>;

thread_local! {
    // static ID_STORE: RefCell<IdStore> = RefCell::default();
    // static PROFILE_TYPE_STORE: RefCell<ProfileTypeStore> = RefCell::default();
    static APPLICANT_PROFILE_STORE: RefCell<ApplicantProfileStore> = RefCell::default();
    static COMPANY_PROFILE_STORE: RefCell<CompanyProfileStore> = RefCell::default();

    static JOB_ID_STORE: Cell<u128> = Cell::new(0);
    static JOB_STORE: RefCell<JobStore> = RefCell::default();

    static APPLICATION_ID_STORE: Cell<u128> = Cell::new(0);
    static APPLICATION_STORE: RefCell<ApplicationStore> = RefCell::default();

}

#[query]
fn get_principal() -> Principal {
    ic_cdk::api::caller()
}

#[update]
fn create_applicant_profile(input: ApplicantProfile) -> CreationStatus {
    let principal_id = ic_cdk::api::caller();
    let mut is_exist = false;

    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };
    });

    if is_exist {
        return CreationStatus::Fail;
    };

    APPLICANT_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };

        let profile = ApplicantProfile {
            id: Some(principal_id),
            ..input
        };

        profile_store.borrow_mut().insert(principal_id, profile);
    });

    if is_exist {
        return CreationStatus::Fail;
    };

    return CreationStatus::Success;
}

#[update]
fn create_company_profile(input: CompanyProfile) -> CreationStatus {
    let principal_id = ic_cdk::api::caller();
    let mut is_exist = false;

    APPLICANT_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };
    });

    if is_exist {
        return CreationStatus::Fail;
    };

    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };

        let profile = CompanyProfile {
            id: Some(principal_id),
            ..input
        };

        profile_store.borrow_mut().insert(principal_id, profile);
    });

    if is_exist {
        return CreationStatus::Fail;
    };
    return CreationStatus::Success;
}

#[update]
fn create_job(input: CreateJob) -> CreationStatus {
    let principal_id = ic_cdk::api::caller();
    let mut is_exist = false;

    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };
    });

    if !is_exist {
        return CreationStatus::Fail;
    }

    JOB_ID_STORE.with(|id| {
        JOB_STORE.with(|job_store| {
            let job = Job {
                id: id.get(),
                company_id: Some(principal_id),
                position: input.position,
                description: input.description,
                bounty: input.bounty,
                status: JobStatus::Open,
                // required_skills: input.required_skills.clone(),
            };

            id.set(id.get() + 1);

            job_store.borrow_mut().insert(id.get(), job);
        });
    });

    return CreationStatus::Success;
}

#[update]
fn apply_to_job(input: Application) -> CreationStatus {
    let principal_id = ic_cdk::api::caller();
    // should use flag
    let mut is_exist_applicant = false;
    let mut is_exist_job_ = false;

    APPLICANT_PROFILE_STORE.with(|profile_store| {
        if profile_store.borrow().get(&principal_id).is_none() {
            return;
        };

        is_exist_applicant = true;
    });

    if !is_exist_applicant {
        return CreationStatus::Fail;
    }

    JOB_STORE.with(|job_store| {
        if job_store.borrow().get(&input.job_id).is_none() {
            return;
        }

        is_exist_job_ = true;
    });

    if !is_exist_job_ {
        return CreationStatus::Fail;
    }

    APPLICATION_ID_STORE.with(|id| {
        APPLICATION_STORE.with(|application_store| {
            let application = Application {
                id: id.get(),
                applicant_id: Some(principal_id),
                job_id: input.job_id,
                status: ApplicationStatus::Applied,
                contact_email: input.contact_email,
                salary_from: input.salary_from,
                salary_to: input.salary_to,
            };

            id.set(id.get() + 1);

            application_store.borrow_mut().insert(id.get(), application);
        });
    });

    return CreationStatus::Success;
}

#[query]
fn get_company(id: Principal) -> Option<CompanyProfile> {
    COMPANY_PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .map(|profile| profile.to_owned())
    })
}

#[query]
fn get_applicant(id: Principal) -> Option<ApplicantProfile> {
    APPLICANT_PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .map(|profile| profile.to_owned())
    })
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct CompanyProfile {
    id: Option<Principal>,
    name: String,
    logo: String,
    twitter: String,
    website: String,
    created_at: u64,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ApplicantProfile {
    id: Option<Principal>,
    first_name: String,
    last_name: String,
    nickname: String,
    bio: String,
    created_at: u64,
    // skills: Vec<Skill>,
    skills: Skill,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Skill {
    id: u32,
    name: String,
}

#[derive(Clone, Default, CandidType, Deserialize)]
struct Job {
    id: u128,
    company_id: Option<Principal>,
    position: String,
    description: String,
    bounty: u128,
    status: JobStatus,
    // required_skills: Vec<Skill>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct CreateJob {
    position: String,
    description: String,
    bounty: u128,
    // required_skills: Vec<Skill>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct ApplyToJob {
    job_id: u128,
    contact_email: String,
    salary_from: u128,
    salary_to: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Application {
    id: u128,
    applicant_id: Option<Principal>,
    job_id: u128,
    contact_email: String,
    status: ApplicationStatus,
    salary_from: u128,
    salary_to: u128,
}

enum UserType {
    Company,
    Applicant,
}

#[derive(Clone, CandidType, Deserialize)]
enum CreationStatus {
    Success,
    Fail,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum JobStatus {
    Open,
    Canceled,
    Closed,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum ApplicationStatus {
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

ic_cdk::export_candid!();
