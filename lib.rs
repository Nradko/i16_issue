#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod i16_issue {
    use ink::env::debug_println;
    use ink::prelude::vec::Vec;

    #[derive(Debug, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct I16Pair {
        value1: i16,
        value2: i16,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct I16Issue {
        values_i16_tuple: Vec<(i16, i16)>,
        values_i16_pair: Vec<I16Pair>,
    }

    impl I16Issue {
        #[ink(constructor)]
        pub fn default() -> Self {
            I16Issue {
                values_i16_tuple: Vec::<(i16, i16)>::default(),
                values_i16_pair: Vec::<I16Pair>::default(),
            }
        }

        #[ink(message)]
        pub fn push_i16(&mut self, value1: i16, value2: i16) {
            self.values_i16_tuple.push((value1, value2));
            self.values_i16_pair.push(I16Pair { value1, value2 });
        }

        #[ink(message)]
        pub fn get_i16_tuples(&self) -> Vec<(i16, i16)> {
            debug_println!("{:?}", self.values_i16_tuple.clone());
            self.values_i16_tuple.clone()
        }

        #[ink(message)]
        pub fn get_i16_pairs(&self) -> Vec<I16Pair> {
            debug_println!("{:?}", self.values_i16_pair.clone());
            self.values_i16_pair.clone()
        }
    }
}
