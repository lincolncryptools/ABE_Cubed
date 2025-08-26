use std::collections::HashMap;

use crate::policy::UserAttribute;

use crate::curve::{Gt, ScalarField, G, H};

mod decrypt;
mod encrypt;
mod keygen;
mod setup;

use super::common::Scheme;
use super::common::{FullKey, PartialKey};

pub struct Opt6 {
    pub name: String,
}

pub struct PartialMSK {
    pub auth: String,
    pub beta: ScalarField,
    pub b: ScalarField,
    pub b_prime: ScalarField,
    pub b_not: ScalarField,
    pub b_not_prime: ScalarField,
}

impl PartialKey for PartialMSK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub type MSK = FullKey<PartialMSK>;

pub struct PartialMPK {
    pub auth: String,
    pub a: H,
    pub b: H,
    pub b_prime: G,
    pub b_not: H,
    pub b_not_prime: G,
}

impl PartialKey for PartialMPK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub type MPK = FullKey<PartialMPK>;

pub struct PartialUSK {
    pub auth: String,
    pub k_1_1_vec: Vec<G>,
    pub k_1_2_map: HashMap<(String, String), G>,
    pub k_2_1: G,
    pub k_2_2_map: HashMap<String, G>,
    pub k_3_map: HashMap<(String, String), G>,
    pub k_4_vec: Vec<H>,
    pub k_5_vec: Vec<H>,
    pub k_6: H,
}

impl PartialKey for PartialUSK {
    fn get_auth(&self) -> String {
        self.auth.clone()
    }
}

pub type USK = FullKey<PartialUSK>;

impl USK {
    pub fn get_user_attributes(&self) -> Vec<UserAttribute> {
        let mut user_attrs = Vec::new();
        for (auth, usk) in self.partial_keys.iter() {
            for (lbl, attr) in usk.k_1_2_map.keys().into_iter() {
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
    pub c_5_vec: Vec<G>,
}

impl Scheme for Opt6 {
    type MSK = MSK;

    type PartialMSK = PartialMSK;

    type MPK = MPK;

    type PartialMPK = PartialMPK;

    type USK = USK;

    type PartialUSK = PartialUSK;

    type Ciphertext = Ciphertext;

    fn new() -> Self {
        Opt6 {
            name: String::from("opt6"),
        }
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn setup(&self, rng: impl rand::Rng, auths: &Vec<&str>) -> (Self::MSK, Self::MPK) {
        setup::setup(rng, auths)
    }

    fn keygen(
        &self,
        rng: impl rand::Rng,
        gid: &str,
        msk: &Self::MSK,
        user_attrs: &Vec<crate::policy::UserAttribute>,
        iota: &super::Iota,
    ) -> Self::USK {
        keygen::keygen(rng, gid, msk, user_attrs, iota)
    }

    fn encrypt(
        &self,
        rng: impl rand::Rng,
        mpk: &Self::MPK,
        policy: &crate::policy::Policy,
        tau: &super::Tau,
    ) -> (Gt, Self::Ciphertext) {
        encrypt::encrypt(rng, mpk, policy, tau)
    }

    fn decrypt(
        &self,
        usk: &Self::USK,
        gid: &str,
        iota: &super::Iota,
        tau: &super::Tau,
        policy: &crate::policy::Policy,
        ct: &Self::Ciphertext,
    ) -> Option<Gt> {
        decrypt::decrypt(usk, gid, iota, tau, policy, ct)
    }
}
