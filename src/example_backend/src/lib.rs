use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
#[derive(CandidType,Deserialize)]
struct Exam {
    out_of: u8,
    course: String,
    curve: u8,
}

impl Storable for Exam {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

// Implement BoundedStorable for Proposal
impl BoundedStorable for Exam {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; // Adjust the size as needed
    const IS_FIXED_SIZE: bool = false;
}


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static EXAM_MAP: RefCell<StableBTreeMap<u64, Exam, Memory>> =
        RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))));
    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> =
        RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))));


#[ic_cdk::query]
fn get_participation(key: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p: &RefCell<StableBTreeMap<u64, u64, Memory>>| {
        p.borrow().get(&key)
    })
}

#[ic_cdk::query]
fn get_exam(key: u64) -> Option<Exam> {
    EXAM_MAP.with(|p: &RefCell<StableBTreeMap<u64, Exam, Memory>>| {
        p.borrow().get(&key)
    })
}

#[ic_cdk::update]
fn insert_exam(key: u64, value: Exam) -> Option<Exam> {
    EXAM_MAP.with(|p: &RefCell<StableBTreeMap<u64, Exam, Memory>>| {
        p.borrow_mut().insert(key, value)
    })
}

#[ic_cdk::update]
fn insert_participation(key: u64, value: u64) -> Option<u64> {
    PARTICIPATION_PERCENTAGE_MAP.with(|p: &RefCell<StableBTreeMap<u64, u64, Memory>>| {
        p.borrow_mut().insert(key, value)
    })
}