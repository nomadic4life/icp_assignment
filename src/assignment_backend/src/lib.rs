use candid::Principal;
use ic_cdk::api::time;
use ic_cdk::{query, update};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::ops::Bound::Included;

pub mod states;
pub use states::*;

// type IdStore = BTreeMap<String, Principal>;
// type ProfileTypeStore = BTreeMap<Principal, UserType>;
type ApplicantProfileStore = BTreeMap<Principal, ApplicantProfile>;
type CompanyProfileStore = BTreeMap<Principal, CompanyProfile>;

type JobStore = BTreeMap<u128, Job>;
type ApplicationStore = BTreeMap<u128, Application>;
type SkillStore = BTreeMap<u16, Skill>;

thread_local! {
    // static ID_STORE: RefCell<IdStore> = RefCell::default();
    // static PROFILE_TYPE_STORE: RefCell<ProfileTypeStore> = RefCell::default();
    static APPLICANT_PROFILE_STORE: RefCell<ApplicantProfileStore> = RefCell::default();
    static COMPANY_PROFILE_STORE: RefCell<CompanyProfileStore> = RefCell::default();

    static JOB_ID_STORE: Cell<u128> = Cell::new(0);
    static JOB_STORE: RefCell<JobStore> = RefCell::default();

    static APPLICATION_ID_STORE: Cell<u128> = Cell::new(0);
    static APPLICATION_STORE: RefCell<ApplicationStore> = RefCell::default();

    static SKILL_ID_STORE: Cell<u16> = Cell::new(0);
    static SKILL_STORE: RefCell<SkillStore> = RefCell::default();
}

fn is_valid_create_user() -> Result<(), String> {
    let principal_id = ic_cdk::api::caller();

    let mut is_exist = false;
    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
        };
    });

    if is_exist {
        return Err(String::from("User already exist as Company"));
    };

    let mut is_exist = false;
    APPLICANT_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
        };
    });

    if is_exist {
        return Err(String::from("User already exist as Applicant"));
    };

    Ok(())
}

fn is_valid_applicant() -> Result<(), String> {
    let principal_id = ic_cdk::api::caller();

    let mut is_exist = false;
    APPLICANT_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
        };
    });

    if is_exist {
        return Err(String::from("User already exist as Applicant"));
    };

    Ok(())
}

fn is_valid_company() -> Result<(), String> {
    let principal_id = ic_cdk::api::caller();

    let mut is_exist = false;
    COMPANY_PROFILE_STORE.with(|profile_store| {
        if !profile_store.borrow().get(&principal_id).is_none() {
            is_exist = true;
        };
    });

    if !is_exist {
        return Err(String::from("Invalid User"));
    };

    Ok(())
}

fn update_skill(skills: Vec<Skill>) -> BTreeMap<u16, Skill> {
    let mut updated_skills = BTreeMap::<u16, Skill>::new();

    // not handling validations of skills,
    // verify by id is easy, but to verify to string via on chain would require a bit of work with some limits imposed.
    // because it could be an attacked vector if no limits.
    // could use a different skill type for input and for storing, so don't need to store id as an option
    SKILL_STORE.with(|skill_store| {
        for skill in skills.iter() {
            SKILL_ID_STORE.with(|id_store| {
                if !skill.id.is_none() {
                    updated_skills.insert(skill.id.unwrap(), skill.clone());
                    return;
                }

                // fail safe to not overwrite existing skills if overflow
                if id_store.get() != u16::MAX {
                    return;
                }

                let id = id_store.get();
                skill_store.borrow_mut().insert(
                    id.to_owned(),
                    Skill {
                        id: Some(id.to_owned()),
                        name: skill.name.clone(),
                    },
                );

                updated_skills.insert(
                    id.to_owned(),
                    Skill {
                        id: Some(id.to_owned()),
                        name: skill.name.clone(),
                    },
                );

                id_store.set(id + 1);
            });
        }
    });

    return updated_skills;
}

#[update(guard = "is_valid_create_user")]
fn create_applicant_profile(params: ApplicantParams, skills: Vec<Skill>) {
    let principal_id = ic_cdk::api::caller();

    APPLICANT_PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(
            principal_id,
            ApplicantProfile {
                id: Some(principal_id),
                first_name: params.first_name,
                last_name: params.last_name,
                nickname: params.nickname,
                bio: params.bio,

                // blocktime() -> is this a thing? will come back to this later
                created_at: time(),
                skills: update_skill(skills),
            },
        );
    });
}

#[update(guard = "is_valid_create_user")]
fn create_company_profile(params: CompanyParams) {
    let principal_id = ic_cdk::api::caller();

    COMPANY_PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(
            principal_id,
            CompanyProfile {
                id: Some(principal_id),
                name: params.name,
                logo: params.logo,
                twitter: params.twitter,
                website: params.website,

                // blocktime() -> is this a thing? will come back to this later
                created_at: time(),
            },
        );
    });
}

#[update(guard = "is_valid_company")]
fn create_job(params: JobParams, skills: Vec<Skill>) {
    let principal_id = ic_cdk::api::caller();

    JOB_ID_STORE.with(|id| {
        JOB_STORE.with(|job_store| {
            id.set(id.get() + 1);

            job_store.borrow_mut().insert(
                id.get(),
                Job {
                    id: id.get(),
                    company_id: Some(principal_id),
                    position: params.position,
                    description: params.description,
                    bounty: params.bounty,
                    status: JobStatus::Open,
                    required_skills: update_skill(skills),
                },
            );
        });
    });
}

#[update(guard = "is_valid_applicant")]
fn apply_to_job(params: ApplicationParams) {
    let principal_id = ic_cdk::api::caller();

    // VALIDATIONS
    let mut is_exist_job_ = false;
    JOB_STORE.with(|job_store| {
        if job_store.borrow().get(&params.job_id).is_none() {
            return;
        }

        is_exist_job_ = true;
    });

    if !is_exist_job_ {
        return;
    }

    APPLICATION_ID_STORE.with(|id| {
        APPLICATION_STORE.with(|application_store| {
            id.set(id.get() + 1);

            application_store.borrow_mut().insert(
                id.get(),
                Application {
                    id: id.get(),
                    applicant_id: Some(principal_id),
                    job_id: params.job_id,
                    status: ApplicationStatus::Applied,
                    contact_email: params.contact_email,
                    salary_from: params.salary_from,
                    salary_to: params.salary_to,
                },
            );
        });
    });
}

// make offer to existing application
// make offer directly to applicant
// decline application to job
// accept offer
// reject offer

#[update]
fn cancel_job(id: u128) {
    let principal_id = ic_cdk::api::caller();

    JOB_STORE.with(|job_store| {
        let job = job_store.borrow();
        let job = job.get(&id);

        if job.is_none() {
            // job doesn't exist
            return;
        }

        let job = job.unwrap().to_owned();

        if job.company_id.unwrap() != principal_id {
            // invalid authority
            return;
        }

        job_store.borrow_mut().insert(
            id,
            Job {
                status: JobStatus::Canceled,
                ..job
            },
        );
    });
}

#[update]
fn withdraw_application(id: u128) {
    let principal_id = ic_cdk::api::caller();
    APPLICATION_STORE.with(|application_store| {
        let application = application_store.borrow();
        let application = application.get(&id);

        if application.is_none() {
            // application doesn't exist
            return;
        }

        let application = application.unwrap().to_owned();

        if application.applicant_id.unwrap() != principal_id {
            // invalid authority
            return;
        }

        application_store.borrow_mut().insert(
            id,
            Application {
                status: ApplicationStatus::Withdraw,
                ..application
            },
        );
    });
}

#[query]
fn get_principal() -> Principal {
    // get principle is used for testing
    ic_cdk::api::caller()
}

#[query]
fn get_company(id: Principal) -> Option<CompanyProfile> {
    // should repackage the data?
    COMPANY_PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .map(|profile| profile.to_owned())
    })
}

#[query]
fn get_applicant(id: Principal) -> Option<ApplicantProfile> {
    // should repackage the data?
    APPLICANT_PROFILE_STORE.with(|profile_store| {
        profile_store
            .borrow()
            .get(&id)
            .map(|profile| profile.to_owned())
    })
}

#[query]
fn get_job(id: u128) -> Option<Job> {
    // should repackage the data?
    JOB_STORE.with(|store| store.borrow().get(&id).map(|data| data.to_owned()))
}

#[query]
fn get_application(id: u128) -> Option<Application> {
    // should repackage the data?
    APPLICATION_STORE.with(|store| store.borrow().get(&id).map(|data| data.to_owned()))
}

#[query]
fn get_job_list(offset: u128) -> Option<Vec<Job>> {
    let mut data = Vec::<Job>::new();

    JOB_STORE.with(|store| {
        let len = store.borrow().len() as u128;
        let limit = 50;

        if len == 0 || offset >= len {
            return;
        }

        let start = offset;
        let end = if offset + limit > len {
            len - offset - 1
        } else {
            offset + limit
        };

        for (_, &ref value) in store.borrow().range((Included(&start), Included(&end))) {
            data.push(value.to_owned());
        }
    });

    if data.len() == 0 {
        None
    } else {
        Some(data)
    }
}

#[query]
fn get_application_list(offset: u128) -> Option<Vec<Application>> {
    let mut data = Vec::<Application>::new();

    APPLICATION_STORE.with(|store| {
        let len = store.borrow().len() as u128;
        let limit = 50;

        if len == 0 || offset >= len {
            return;
        }

        let start = offset;
        let end = if offset + limit > len {
            len - offset - 1
        } else {
            offset + limit
        };

        for (_, &ref value) in store.borrow().range((Included(&start), Included(&end))) {
            data.push(value.to_owned());
        }
    });

    if data.len() == 0 {
        None
    } else {
        Some(data)
    }
}

ic_cdk::export_candid!();

// MAIN TASK
//  create a job -> completed
//  cancel the job -> completed
//  apply to the job -> completed
//  withdraw application -> completed
//  make offer -> exsting application | create application by company for applicant?
//  aceptjob

//  :: paginated list ::
//  get list of jobs applied by user
//  get list of jobs created by company
//  get list of applicants
//  get list of companies
//  get list of jobs -> completed
//  get list of applications -> completed
//  get list of skills

//  :: search :: unoptimized search
//  find skill

//  :: filter :: unoptimized filter
//  list of jobs by skill
//  list of applicants by skill

// user profile
// register user

// udpates
//  job -> company authority
//  application -> applicant authority
//  applicant -> applicant authority
//  company -> applicant authority

// consideration
//  delete skill if no reference to skill -> could use a reference counter
//  delete job if no reference to job -> could use a reference counter
//  delete application if no reference to application -> could use a reference counter
//  delete applicant if no reference to applicant -> could use a reference counter
//  delete company if no reference to company -> could use a reference counter

// need to add sanitization  best to handle on chain, can't trust source
// skills
// first name
// last name
// bio
// company name
// logo
// twitter
// website
// position
// description
// contact email

// validations best to handle on chain, can't trust source
// email
// url [twitter, website, logo]
// salary_from < salary_to

// additional search functionality
//  job by position
//  job by bounty
//  applicant by location
//  application by status
//  job by status

// optimized data structures, for sorting, searching, and reference relationship
// better folder structure
// simple frontend handling core functionality
//  create a job
//  cancel the job
//  apply to the job
//  withdraw application
//  make offer -> exsting application | create application by company for applicant?
//  aceptjob
//  get jobs
//  get applications

// make test cases
