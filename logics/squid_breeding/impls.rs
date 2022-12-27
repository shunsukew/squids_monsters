use crate::traits::*;
use openbrush::{
    contracts::psp34::{Id, self},
    storage::{
        Mapping,
    },
    traits::{
        Storage,
        Balance,
        AccountId,
    },
};
use squid_base::traits::Squid;
use squid_base::impls::Data as SquidBaseData;

pub const DATA_STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(DATA_STORAGE_KEY)]
pub struct Data {
    pub auto_birth_fee: Balance,
    pub pregnant_squids: u64,
    pub gene_science: AccountId,
}

impl<T: Storage<Data> + Internal> SquidBreeding for T {
    default fn set_gene_science_account_id(&mut self, account_id: AccountId) {

        // TODO
        // account_id validation by cross contract call

        self.data().gene_science = account_id;
    }
}

pub trait Internal {
    fn _is_ready_to_breed(&self, _squid: Squid) -> bool;

    fn _is_siring_permitted(&self, _sire_id: Id, _matron_id: Id) -> bool;
}

impl<T: Storage<Data> + Storage<SquidBaseData> + psp34::Internal> Internal for T {
    default fn _is_ready_to_breed(&self, _squid: Squid) -> bool {
        _squid.siring_with_id == 0 && _squid.cooldown_end_block <= Self::env().block_number()
    }

    default fn _is_siring_permitted(&self, _sire_id: Id, _matron_id: Id) -> bool {
        let matron_owner = match self._owner_of(&_matron_id) {
            Some(id) => id,
            None => return false,
        };
        let sire_owner = match self._owner_of(&_sire_id) {
            Some(id) => id,
            None => return false,
        };

        let allowed_matron_owner = match self.data::<SquidBaseData>().sire_allowed_to_account_id.get(&_sire_id) {
            Some(id) => id,
            None => return false,
        };

        matron_owner == sire_owner ||
            matron_owner == allowed_matron_owner
    }
}

