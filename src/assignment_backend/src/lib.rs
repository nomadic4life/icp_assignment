use candid::{CandidType, Deserialize, Principal};
use ic_cdk::{api::call::ManualReply, query, update};
// use serde::de::value::Error;
use std::cell::RefCell;
use std::collections::BTreeMap;

// type IdStore = BTreeMap<String, Principal>;
// type ProfileTypeStore = BTreeMap<Principal, UserType>;
type ApplicantProfileStore = BTreeMap<Principal, ApplicantProfile>;
type CompanyProfileStore = BTreeMap<Principal, CompanyProfile>;

thread_local! {
    // static ID_STORE: RefCell<IdStore> = RefCell::default();
    // static PROFILE_TYPE_STORE: RefCell<ProfileTypeStore> = RefCell::default();
    static APPLICAT_PROFILE_STORE: RefCell<ApplicantProfileStore> = RefCell::default();
    static COMPANY_PROFILE_STORE: RefCell<CompanyProfileStore> = RefCell::default();

}

#[query(manual_reply = true)]
fn get_self() -> ManualReply<ApplicantProfile> {
    let id = ic_cdk::api::caller();
    APPLICAT_PROFILE_STORE.with(|profile_store| {
        if let Some(profile) = profile_store.borrow().get(&id) {
            ManualReply::one(profile)
        } else {
            ManualReply::one(ApplicantProfile::default())
        }
    })

    // APPLICAT_PROFILE_STORE.with(|profile_store| {
    //     let data = profile_store.borrow();
    //     let data = data.get(&id);

    //     if data.is_none() {
    //         return None;
    //     } else {
    //         return Some(data.unwrap().to_owned());
    //     }
    // })
}

#[query]
fn get_principal() -> Principal {
    ic_cdk::api::caller()
}

#[update]
fn create_applicant_profile(input: ApplicantProfile) -> u8 {
    let principal_id = ic_cdk::api::caller();
    let mut is_exist = false;

    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };
    });

    // return error or ok response
    if is_exist {
        return 1;
    };

    APPLICAT_PROFILE_STORE.with(|profile_store| {
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

    // return error or ok response
    if is_exist {
        return 1;
    };
    return 0;
}

#[update]
fn create_company_profile(input: CompanyProfile) -> u8 {
    let principal_id = ic_cdk::api::caller();
    let mut is_exist = false;

    APPLICAT_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
            return;
        };
    });

    // return error or ok response
    if is_exist {
        return 1;
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

    // return error or ok response
    if is_exist {
        return 1;
    };
    return 0;
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

struct Job {
    id: u64,
    company_id: Principal,
    position: String,
    description: String,
    bounty: u64,
    status: JobStatus,
    required_skills: Vec<Skill>,
}

enum UserType {
    Company,
    Applicant,
}

enum JobStatus {}

ic_cdk::export_candid!();
