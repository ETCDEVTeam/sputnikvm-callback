extern crate sputnikvm;
extern crate bigint;

use sputnikvm::{VM, AccountCommitment, RequireError, VMStatus, AccountChange, Log};
use bigint::{Address, U256, M256, H256, Gas};
use std::collections::HashSet;
use std::collections::hash_map::Values;

pub trait Callback {
    fn balance(&self, Address) -> U256;
    fn nonce(&self, Address) -> U256;
    fn code(&self, Address) -> Vec<u8>;
    fn storage(&self, Address, U256) -> M256;
    fn exists(&self, Address) -> bool;
    fn blockhash(&self, U256) -> H256;
}

pub struct CallbackVM<'a, V: VM, C: Callback + 'a>(V, &'a C);

impl<'a, V: VM, C: Callback + 'a> CallbackVM<'a, V, C> {
    pub fn new(vm: V, callback: &'a C) -> Self {
        CallbackVM(vm, callback)
    }

    pub fn fire(&mut self) {
        loop {
            match self.0.fire() {
                Ok(()) => break,
                Err(RequireError::Account(address)) => {
                    if self.1.exists(address) {
                        let commit = AccountCommitment::Full {
                            address,
                            nonce: self.1.nonce(address),
                            balance: self.1.balance(address),
                            code: self.1.code(address),
                        };
                        self.0.commit_account(commit).unwrap();
                    } else {
                        self.0.commit_account(AccountCommitment::Nonexist(address)).unwrap();
                    }
                },
                Err(RequireError::AccountCode(address)) => {
                    if self.1.exists(address) {
                        let commit = AccountCommitment::Code {
                            address,
                            code: self.1.code(address),
                        };
                        self.0.commit_account(commit).unwrap();
                    } else {
                        self.0.commit_account(AccountCommitment::Nonexist(address)).unwrap();
                    }
                },
                Err(RequireError::AccountStorage(address, index)) => {
                    if self.1.exists(address) {
                        let commit = AccountCommitment::Storage {
                            address, index,
                            value: self.1.storage(address, index),
                        };
                        self.0.commit_account(commit).unwrap();
                    } else {
                        self.0.commit_account(AccountCommitment::Nonexist(address)).unwrap();
                    }
                },
                Err(RequireError::Blockhash(number)) => {
                    self.0.commit_blockhash(number, self.1.blockhash(number)).unwrap();
                },
            }
        }
    }

    pub fn status(&self) -> VMStatus {
        self.0.status()
    }

    pub fn accounts(&self) -> Values<Address, AccountChange> {
        self.0.accounts()
    }

    pub fn used_addresses(&self) -> HashSet<Address> {
        self.0.used_addresses()
    }

    pub fn out(&self) -> &[u8] {
        self.0.out()
    }

    pub fn available_gas(&self) -> Gas {
        self.0.available_gas()
    }

    pub fn refunded_gas(&self) -> Gas {
        self.0.refunded_gas()
    }

    pub fn logs(&self) -> &[Log] {
        self.0.logs()
    }

    pub fn removed(&self) -> &[Address] {
        self.0.removed()
    }
}
