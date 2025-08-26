use crate::curve::{pairing, Gt, ScalarField, G, H};
use crate::scheme::common::group_pairs;

use ark_ec::{CurveGroup, Group, VariableBaseMSM};
use ark_ff::Field;
use ark_std::{ops::Neg, Zero};

type Ciphertext = <super::Opt1 as Scheme>::Ciphertext;
type USK = <super::Opt1 as Scheme>::USK;
use crate::hashing::{hash_attr, hash_gid};
use crate::policy::Policy;
use crate::scheme::common::{Iota, Scheme, Tau};

fn solve_lse(usk: &USK, policy: &Policy) -> Option<(Vec<usize>, Vec<usize>)> {
    let user_attrs = usk.get_user_attributes();
    let eps_all = policy.reconstruct_secret(&user_attrs)?;
    let (eps_not_vec, eps_vec) = eps_all.into_iter().partition(|i| policy.get(*i).1);
    Some((eps_vec, eps_not_vec))
}

pub fn decrypt(
    usk: &USK,
    gid: &str,
    _iota: &Iota,
    _tau: &Tau,
    policy: &Policy,
    ct: &Ciphertext,
) -> Option<Gt> {
    let (eps_vec, eps_not_vec) = solve_lse(&usk, &policy)?;
    let mut k = Gt::ONE;
    let mut c_1 = H::zero();
    let mut c_3 = H::zero();
    for j in eps_vec.iter().chain(eps_not_vec.iter()) {
        c_1 += ct.c_1_vec[*j];
        c_3 += ct.c_3_vec[*j];
    }
    k *= pairing(G::generator(), c_3).0;
    k *= pairing(hash_gid(gid), c_1).0;

    let eps_by_auth_lbl_attr = group_pairs(&eps_vec, |j| policy.get(j).0.auth_lbl_attr());
    let eps_by_auth_attr = group_pairs(&eps_vec, |j| policy.get(j).0.auth_attr());
    let eps_not_by_auth_lbl_attr = group_pairs(&eps_not_vec, |j| policy.get(j).0.auth_lbl_attr());

    for ((auth, lbl, attr), js) in eps_by_auth_lbl_attr {
        let mut c_4 = H::zero();
        for j in js {
            c_4 += ct.c_4_vec[j];
        }
        let usk = usk.get_partial_key(&auth).unwrap();
        let k_1 = usk.k_1_map.get(&(lbl, attr.clone())).unwrap().neg();
        k *= pairing(k_1, c_4).0;
    }

    for ((auth, attr), js) in eps_by_auth_attr {
        let mut c_2 = G::zero();
        for j in js {
            c_2 += ct.c_2_vec[j];
        }
        let usk = usk.get_partial_key(&auth).unwrap();
        let k_4 = usk.k_4_map.get(&attr).unwrap();
        k *= pairing(c_2, k_4).0;
    }

    for ((auth, lbl, attr), js) in eps_not_by_auth_lbl_attr {
        let mut c_2 = G::zero();
        let mut c_4 = H::zero();
        for j in js {
            c_2 += ct.c_2_vec[j];
            c_4 += ct.c_4_vec[j];
        }

        let x_attr_not = hash_attr(&attr);
        let usk = usk.get_partial_key(&auth).unwrap();

        let k_2 = usk.k_2_map.get(&lbl).unwrap().neg();

        let attrs: Vec<String> = usk
            .k_1_map
            .keys()
            .filter_map(|k| {
                if k.0.eq(&lbl) {
                    Some(k.1.clone())
                } else {
                    None
                }
            })
            .collect();
        let mut k_5_bases = Vec::with_capacity(attrs.len());
        let mut k_5_exps = Vec::with_capacity(attrs.len());
        let mut k_3_bases = Vec::with_capacity(attrs.len());
        let mut k_3_exps = Vec::with_capacity(attrs.len());
        let one = ScalarField::from(1);
        for attr in attrs {
            let x_attr = hash_attr(&attr);
            let e = one / (x_attr_not - x_attr);
            k_5_exps.push(e);
            k_5_bases.push(usk.k_5_map.get(&attr).unwrap().into_affine());
            let e = -one / (x_attr_not - x_attr);
            k_3_exps.push(e);
            k_3_bases.push(
                usk.k_3_map
                    .get(&(lbl.clone(), attr.clone()))
                    .unwrap()
                    .into_affine(),
            );
        }
        let k_3 = G::msm(&k_3_bases, &k_3_exps).unwrap();
        let k_5 = H::msm(&k_5_bases, &k_5_exps).unwrap();
        k *= pairing(c_2, k_5).0;
        k *= pairing(k_2 + k_3, c_4).0;
    }
    Some(k)
}
