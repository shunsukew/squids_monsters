use openbrush::{
    contracts::{
        access_control::{*, RoleType},
    },
};

// - Admin: The Admin can change access controls and change the addresses of our dependent smart
//         contracts. It is also the only role that can pause/unpause the smart contract. It is initially
//         set to the address that created the smart contract in the SquidCore constructor.
//
// - Withdrawer: The Withdrawer can withdraw funds from SquidCore and its auction contracts.
//
// - MINTER: The Minter can release gen0 squids to auction, and mint promo squids.

pub const ADMIN: RoleType = DEFAULT_ADMIN_ROLE;
pub const WITHDRAWER: RoleType = ink_lang::selector_id!("Withdrawer");
pub const MINTER: RoleType = ink_lang::selector_id!("Minter");
