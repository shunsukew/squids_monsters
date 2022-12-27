use crate::traits::*;
use openbrush::{
    contracts::{
        psp34::{Id, self}
    },
    storage::{
        Mapping,
    },
    traits::{
        Storage,
        AccountId,
    },
};

pub const DATA_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(DATA_STORAGE_KEY)]
pub struct Data {
    pub next_squid_id: u32,
    pub squids: Mapping<Id, Squid>,
    pub sire_allowed_to_account_id: Mapping<Id, AccountId>,
    // The address of the ClockAuction contract that handles sales of Squids.
    pub sale_clock_auction: AccountId,  
    // The address of a custom ClockAuction subclassed contract that handles siring auctions.
    pub siring_clock_auction: AccountId,
}

impl<T: Storage<Data> + Internal> SquidBase for T {
    default fn get_squid(&self, id: Id) -> Result<Squid, SquidBaseError> {
        self.data().squids.get(&id).ok_or(SquidBaseError::SquidNotFound)
    }
}

pub trait Internal {
    fn _create_squid(
        &mut self,
        _matron_id: Id,
        _sire_id: Id,
        _generation: u16,
        _genes: u64,
        _owner: AccountId,
    ) -> Result<Id, SquidBaseError>;

    fn _emit_birth_event(&self, _owner: AccountId, _new_squid_id: &Id, _matron_id: &Id, _sire_id: &Id, _genes: u64);
}

impl<T> Internal for T
where
    T: Storage<Data>,
    T: psp34::Internal,
{
    default fn _create_squid(
        &mut self,
        _matron_id: Id,
        _sire_id: Id,
        _generation: u16,
        _genes: u64,
        _owner: AccountId,
    ) -> Result<Id, SquidBaseError> {
        let mut cool_down_index = _generation / 2;
        if cool_down_index > 13 {
            cool_down_index = 13;
        }

        let squid = Squid{
            genes: _genes,
            birth_time: Self::env().block_timestamp(),
            cooldown_end_block: 0,
            matron_id: _matron_id.clone(),
            sire_id: _sire_id.clone(),
            siring_with_id: 0,
            cooldown_index: cool_down_index,
            generation: _generation,
        };

        let next_squid_id = self.data().next_squid_id;
        // just in case
        if next_squid_id + 1 == u32::MAX {
            return Err(SquidBaseError::TooManySquidsCreated)
        }
        self.data().next_squid_id += 1;

        let new_squid_id = Id::U32(next_squid_id);
        self.data().squids.insert(&new_squid_id, &squid);

        self._emit_birth_event(_owner, &new_squid_id, &_matron_id, &_sire_id, _genes);

        // mint new PSP34 token
        self._mint_to(_owner, new_squid_id.clone())?;

        Ok(new_squid_id)
    }

    default fn _emit_birth_event(&self, _owner: AccountId, _new_squid_id: &Id, _matron_id: &Id, _sire_id: &Id, _genes: u64) {}
}
