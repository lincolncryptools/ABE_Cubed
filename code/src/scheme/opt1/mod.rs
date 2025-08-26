mod decrypt;
mod encrypt;
mod keygen;
mod setup;

use super::common::Scheme;
use crate::curve::Gt;

pub struct Opt1 {
    pub name: String,
}

impl Scheme for Opt1 {
    type MSK = super::common::MSK;

    type PartialMSK = super::common::PartialMSK;

    type MPK = super::common::MPK;

    type PartialMPK = super::common::PartialMPK;

    type USK = super::common::USK;

    type PartialUSK = super::common::PartialUSK;

    type Ciphertext = super::common::Ciphertext;

    fn new() -> Self {
        Opt1 {
            name: String::from("opt1"),
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
