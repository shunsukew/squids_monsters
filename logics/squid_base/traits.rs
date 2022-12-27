use ink_storage::traits::PackedLayout;
use openbrush::contracts::{traits::psp34::PSP34Error, psp34::Id};

#[openbrush::wrapper]
pub type SquidBaseRef = dyn SquidBase;

#[openbrush::trait_definition]
pub trait SquidBase {
    #[ink(message)]
    fn get_squid(&self, id: Id) -> Result<Squid, SquidBaseError>;
}

pub const SQUID_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Squid);

#[derive(Default, Debug, scale::Encode, scale::Decode, PackedLayout)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[openbrush::upgradeable_storage(SQUID_STORAGE_KEY)]
pub struct Squid {
    pub genes: u64, // genetic code packed into 64 bits
    pub birth_time: u64,
    pub cooldown_end_block: u32,
    pub matron_id: Id,
    pub sire_id: Id,
    pub siring_with_id: u32, // if set then pregnant
    pub cooldown_index: u16,
    pub generation: u16,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum SquidBaseError {
    PSP34Error(PSP34Error),
    SquidNotFound,
    SquidAlreadyExists,
    TooManySquidsCreated,
}

impl From<PSP34Error> for SquidBaseError {
    fn from(error: PSP34Error) -> Self {
        SquidBaseError::PSP34Error(error)
    }
}
