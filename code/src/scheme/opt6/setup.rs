use crate::curve::{ScalarField, G, H};
use crate::scheme::Scheme;

use ark_ec::Group;
use ark_ff::UniformRand;
use ark_std::ops::Mul;
use ark_std::rand::Rng;

type MSK = <super::Opt6 as Scheme>::MSK;
type PartialMSK = <super::Opt6 as Scheme>::PartialMSK;
type MPK = <super::Opt6 as Scheme>::MPK;
type PartialMPK = <super::Opt6 as Scheme>::PartialMPK;

pub fn setup(mut rng: impl Rng, auths: &Vec<&str>) -> (MSK, MPK) {
    let mut msk = MSK::new();
    let mut mpk = MPK::new();
    for auth in auths {
        let (partial_msk, partial_mpk) = setup_partial(&mut rng, auth);
        msk.add_partial_key(partial_msk);
        mpk.add_partial_key(partial_mpk);
    }
    (msk, mpk)
}

pub fn setup_partial(mut rng: impl Rng, auth: &str) -> (PartialMSK, PartialMPK) {
    let beta = ScalarField::rand(&mut rng);
    let b = ScalarField::rand(&mut rng);
    let b_prime = ScalarField::rand(&mut rng);
    let b_not = ScalarField::rand(&mut rng);
    let b_not_prime = ScalarField::rand(&mut rng);
    let msk = PartialMSK {
        auth: String::from(auth),
        beta,
        b,
        b_prime,
        b_not,
        b_not_prime,
    };

    let a = H::generator().mul(beta);
    let b = H::generator().mul(b);
    let b_prime = G::generator().mul(b_prime);
    let b_not = H::generator().mul(b_not);
    let b_not_prime = G::generator().mul(b_not_prime);
    let mpk = PartialMPK {
        auth: String::from(auth),
        a,
        b,
        b_prime,
        b_not,
        b_not_prime,
    };
    return (msk, mpk);
}
