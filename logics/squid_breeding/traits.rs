use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type SquidBreedingRef = dyn SquidBreeding;

#[openbrush::trait_definition]
pub trait SquidBreeding {
    #[ink(message)]
    fn set_gene_science_account_id(&mut self, account_id: AccountId);
}
