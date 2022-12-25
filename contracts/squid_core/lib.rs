#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod squid_core {
    #[ink(storage)]
    pub struct SquidCore {
        value: bool,
    }

    impl SquidCore {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let squid_core = SquidCore::default();
            assert_eq!(squid_core.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut squid_core = SquidCore::new(false);
            assert_eq!(squid_core.get(), false);
            squid_core.flip();
            assert_eq!(squid_core.get(), true);
        }
    }
}
