use std::collections::HashMap;

use crate::policy::UserAttribute;

use crate::curve::{Gt, G, H};

mod decrypt;
mod encrypt;
mod keygen;
mod setup;

use super::common::Scheme;
use super::common::{FullKey, PartialKey};

pub struct Opt2 {
    pub name: String,
}

pub struct PartialUSK {
    pub auth: String,
    pub k_1_map: HashMap<(String, String), G>,
    pub k_2_map: HashMap<String, G>,
    pub k_3_map: HashMap<(String, String), G>,
    pub k_4_vec: Vec<H>,
    pub k_5_vec: Vec<H>,
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
            for (lbl, attr) in usk.k_1_map.keys().into_iter() {
                user_attrs.push(UserAttribute::new(auth, lbl, attr));
            }
        }
        user_attrs
    }
}

impl Scheme for Opt2 {
    type MSK = super::common::MSK;

    type PartialMSK = super::common::PartialMSK;

    type MPK = super::common::MPK;

    type PartialMPK = super::common::PartialMPK;

    type USK = USK;

    type PartialUSK = PartialUSK;

    type Ciphertext = super::common::Ciphertext;

    fn new() -> Self {
        Opt2 {
            name: String::from("opt2"),
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
