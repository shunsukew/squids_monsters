#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
mod squid_core {
    use ink_lang::codegen::{
        EmitEvent,
        Env,
    };
    use ink_storage::traits::{SpreadAllocate};
    use openbrush::{
        contracts::{
            access_control::{*, RoleType},
            pausable::*,
            psp34::*,
        },
        modifiers,
        traits::{
            Storage,
        },
    };
    use squid_base::{
        traits::*,
    };

    #[ink(event)]
    pub struct BirthEvent {
        #[ink(topic)]
        pub squid_id: Id,
        pub owner: AccountId,
        pub matron_id: Id,
        pub sire_id: Id,
        pub genes: u64,
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, Storage)]
    pub struct SquidCore {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        pause: pausable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        base: squid_base::impls::Data,
    }

    // - Admin: The Admin can change access controls and change the addresses of our dependent smart
    //         contracts. It is also the only role that can pause/unpause the smart contract. It is initially
    //         set to the address that created the smart contract in the SquidCore constructor.
    //
    // - Withdrawer: The Withdrawer can withdraw funds from SquidCore and its auction contracts.
    //
    // - COO: The Minter can release gen0 squids to auction, and mint promo squids.

    const ADMIN: RoleType = DEFAULT_ADMIN_ROLE;
    const WITHDRAWER: RoleType = ink_lang::selector_id!("Withdrawer");
    const MINTER: RoleType = ink_lang::selector_id!("Minter");

    impl SquidCore {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            instance._init_with_admin(caller);
            instance.grant_role(WITHDRAWER, caller).expect("Should grant Withdrawer role");
            instance.grant_role(MINTER, caller).expect("Should grant Minter role");
            instance
        }

        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        pub fn pause(&mut self) -> Result<(), PSP34Error> {
            self._pause()
        }

        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        pub fn unpause(&mut self) -> Result<(), PSP34Error> {
            self._unpause()
        }

        #[ink(message)]
        #[modifiers(only_role(ADMIN))]
        pub fn change_state(&mut self) -> Result<(), PSP34Error> {
            self._switch_pause()
        }
    }

    impl PSP34 for SquidCore {}

    impl Pausable for SquidCore {}

    impl AccessControl for SquidCore {}

    // Squids are only mintable via _create_squids function
    // use _mint_to inside
    // impl PSP34Mintable for SquidCore {
        // #[ink(message)]
        // #[modifiers(only_role(MINTER))]
        // fn mint(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            // self._mint_to(account, id)
        // }
    // }

    impl SquidBase for SquidCore {}

    impl squid_base::impls::Internal for SquidCore {
        fn _emit_birth_event(&self, _owner: AccountId, _new_squid_id: &Id, _matron_id: &Id, _sire_id: &Id, _genes: u64) {
            self.env().emit_event(BirthEvent {
                squid_id: _new_squid_id.clone(),
                owner: _owner,
                matron_id: _matron_id.clone(),
                sire_id: _sire_id.clone(),
                genes: _genes,
            })
        }
    }

    #[cfg(test)]
    mod tests {
        // use super::*;

        // #[ink::test]
        // fn default_works() {
        // }
    }
}
