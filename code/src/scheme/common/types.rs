use std::collections::HashMap;

use rand::Rng;

use crate::curve::{Gt, ScalarField, G, H};
use crate::policy::{Policy, UserAttribute};

use super::{Iota, Tau};

pub trait PartialKey {
    fn get_auth(&self) -> String;
}

pub struct FullKey<T> {
    pub partial_keys: HashMap<String, T>,
}

impl<T: PartialKey> FullKey<T> {
    pub fn new() -> Self {
        FullKey {
            partial_keys: HashMap::new(),
        }
    }

    pub fn add_partial_key(&mut self, new_key: T) {
        if self.partial_keys.contains_key(&new_key.get_auth()) {
            panic!(
                "Partial key for authority '{}' already exists",
                new_key.get_auth()
            );
        } else {
            self.partial_keys.insert(new_key.get_auth(), new_key);
        }
    }

    pub fn get_partial_key(&self, auth: &str) -> Option<&T> {
        self.partial_keys.get(auth)
    }
}

pub struct PartialMSK {
    pub auth: String,
    pub beta: ScalarField,
    pub b: ScalarField,
    pub b_not: ScalarField,
}

impl PartialKey for PartialMSK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub struct PartialMPK {
    pub auth: String,
    pub a: H,
    pub b: H,
    pub b_not: H,
}

impl PartialKey for PartialMPK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub struct PartialUSK {
    pub auth: String,
    pub k_1_map: HashMap<(String, String), G>,
    pub k_2_map: HashMap<String, G>,
    pub k_3_map: HashMap<(String, String), G>,
    pub k_4_map: HashMap<String, H>,
    pub k_5_map: HashMap<String, H>,
}

impl PartialKey for PartialUSK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub type MSK = FullKey<PartialMSK>;
pub type MPK = FullKey<PartialMPK>;
pub type USK = FullKey<PartialUSK>;

impl USK {
    pub fn get_user_attributes(&self) -> Vec<UserAttribute> {
        let mut user_attrs = Vec::new();
        for (auth, usk) in self.partial_keys.iter() {
            for (lbl, attr) in usk.k_1_map.keys().into_iter() {
                user_attrs.push(UserAttribute::new(auth, lbl, attr));
            }
        }
        user_attrs
    }
}

pub struct Ciphertext {
    pub c_1_vec: Vec<H>,
    pub c_2_vec: Vec<G>,
    pub c_3_vec: Vec<H>,
    pub c_4_vec: Vec<H>,
}

pub trait Scheme {
    type MSK;
    type PartialMSK;
    type MPK;
    type PartialMPK;
    type USK;
    type PartialUSK;
    type Ciphertext;

    fn new() -> Self;

    fn get_name(&self) -> String;

    fn setup(&self, rng: impl Rng, auths: &Vec<&str>) -> (Self::MSK, Self::MPK);

    fn keygen(
        &self,
        rng: impl Rng,
        gid: &str,
        msk: &Self::MSK,
        user_attrs: &Vec<UserAttribute>,
        iota: &Iota,
    ) -> Self::USK;

    fn encrypt(
        &self,
        rng: impl Rng,
        mpk: &Self::MPK,
        policy: &Policy,
        tau: &Tau,
    ) -> (Gt, Self::Ciphertext);

    fn decrypt(
        &self,
        usk: &Self::USK,
        gid: &str,
        iota: &Iota,
        tau: &Tau,
        policy: &Policy,
        ct: &Self::Ciphertext,
    ) -> Option<Gt>;
}
