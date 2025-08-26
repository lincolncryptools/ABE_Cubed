use std::{collections::HashSet, vec};

use abe_cubed::{
    curve,
    policy::{Policy, UserAttribute},
};

const USER_ID: &str = "TEST_USER_ID";

fn prepare_test(user_attrs: &Vec<&str>, policy: &str) -> (Vec<String>, Vec<UserAttribute>, Policy) {
    let policy = Policy::parse(policy).unwrap();
    let user_attrs: Vec<UserAttribute> = user_attrs
        .iter()
        .map(|ua| UserAttribute::parse(ua).unwrap())
        .collect();
    let mut auths: HashSet<String> = HashSet::new();
    for ua in user_attrs.iter() {
        auths.insert(ua.auth.clone());
    }
    for idx in 0..policy.len() {
        auths.insert(policy.get(idx).0.auth.clone());
    }
    if auths.is_empty() {
        panic!(
            "Fatal error: cannot execute test case if both user attributes and policy are empty"
        );
    }
    (auths.into_iter().collect(), user_attrs, policy)
}

fn test_scheme(user_attrs: Vec<&str>, policy: &str) -> (curve::Gt, Option<curve::Gt>) {
    use abe_cubed::scheme::{Iota, Opt5, Scheme, Tau};
    let (auths, user_attrs, policy) = prepare_test(&user_attrs, &policy);
    let mut rng = ark_std::test_rng();
    let scheme = Opt5::new();
    let auths: Vec<&str> = auths.iter().map(|s| s as &str).collect();
    let iota = Iota::new(&user_attrs);
    let (msk, mpk) = scheme.setup(&mut rng, &auths);
    let usk = scheme.keygen(&mut rng, USER_ID, &msk, &user_attrs, &iota);
    let tau = Tau::new(&policy);
    let (k_enc, ct) = scheme.encrypt(&mut rng, &mpk, &policy, &tau);
    let k_dec = scheme.decrypt(&usk, USER_ID, &iota, &tau, &policy, &ct);
    (k_enc, k_dec)
}

fn assert_decryption_ok(user_attrs: Vec<&str>, policy: &str) {
    let (k_enc, k_dec) = test_scheme(user_attrs, policy);
    assert!(k_dec.is_some_and(|k| curve::Gt::eq(&k_enc, &k)));
}

fn assert_decryption_fail(user_attrs: Vec<&str>, policy: &str) {
    let (_, k_dec) = test_scheme(user_attrs, policy);
    assert!(k_dec.is_none());
}

// Handcrafted test cases (single auth)

#[test]
fn opt5_single_auth_single_ok() {
    let user_attrs = vec!["A.a:0"];
    let policy = "A.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_single_multi_attr_ok() {
    let user_attrs = vec!["A.a:0", "A.a:0"];
    let policy = "A.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_single_fail() {
    let user_attrs = vec![];
    let policy = "A.a:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_ok() {
    let user_attrs = vec!["A.a:0", "A.b:0"];
    let policy = "A.a:0 & A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_no_left_fail() {
    let user_attrs = vec!["A.b:0"];
    let policy = "A.a:rainy & A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_no_right_fail() {
    let user_attrs = vec!["A.a:0"];
    let policy = "A.a:0 & A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_negation_ok() {
    let user_attrs = vec!["A.a:0"];
    let policy = "!A.a:1";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_negation_multi_alternative_ok() {
    let user_attrs = vec!["A.a:1", "A.a:2", "A.a:3"];
    let policy = "!A.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_negation_contradiction_fail() {
    let user_attrs = vec!["A.a:1"];
    let policy = "!A.a:1";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_negation_no_alternative_fail() {
    let user_attrs = vec!["A.b:0"];
    let policy = "!A.a:1";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_left_ok() {
    let user_attrs = vec!["A.a:0"];
    let policy = "A.a:0 | A.a:1";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_right_ok() {
    let user_attrs = vec!["A.a:1"];
    let policy = "A.a:0 | A.a:1";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_both_ok() {
    let user_attrs = vec!["A.a:0", "A.a:1"];
    let policy = "A.a:0 | A.a:1";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_fail() {
    let user_attrs = vec![];
    let policy = "A.a:0 | A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_left_negated_ok() {
    let user_attrs = vec!["A.a:1", "A.b:0"];
    let policy = "!A.a:0 & A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_left_negated_contradiction_fail() {
    let user_attrs = vec!["A.a:0", "A.b:0"];
    let policy = "!A.a:0 & A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_left_negated_no_alternative_fail() {
    let user_attrs = vec!["A.b:0"];
    let policy = "!A.a:0 & A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_left_negated_no_right_fail() {
    let user_attrs = vec!["A.a:2"];
    let policy = "!A.a:0 & A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_conjunction_both_negated_ok() {
    let user_attrs = vec!["A.a:2", "A.b:1"];
    let policy = "!A.a:0 & !A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_right_negated_ok() {
    let user_attrs = vec!["A.b:1"];
    let policy = "A.a:1 | !A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_right_negated_both_ok() {
    let user_attrs = vec!["A.a:1", "A.b:0"];
    let policy = "A.a:1 | !A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_right_negated_contradiction_fail() {
    let user_attrs = vec!["A.b:0"];
    let policy = "A.a:1 | !A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_right_negated_no_alternative_fail() {
    let user_attrs = vec![];
    let policy = "A.a:1 | !A.b:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_single_auth_disjunction_both_negated_ok() {
    let user_attrs = vec!["A.a:1", "A.b:0"];
    let policy = "!A.a:0 | !A.b:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_complex_1_ok() {
    let user_attrs = vec!["A.a:0", "A.c:0"];
    let policy = "A.a:0 | (!A.b:0 & A.a:2) & !(A.c:1 | A.c:2)";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_complex_2_ok() {
    let user_attrs = vec!["A.a:2", "A.b:1", "A.c:0"];
    let policy = "A.a:0 | (!A.b:0 & A.a:2) & !(A.c:1 | A.c:2)";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_single_auth_complex_fail() {
    let user_attrs = vec!["A.a:2", "A.c:2"];
    let policy = "A.a:0 | (!A.b:0 & A.a:2) & !(A.c:1 | A.c:2)";
    assert_decryption_fail(user_attrs, policy);
}

// Handcrafted test cases (multi auth)

#[test]
fn opt5_multi_auth_disjunction_left_ok() {
    let user_attrs = vec!["A.a:0"];
    let policy = "A.a:0 | B.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_disjunction_right_ok() {
    let user_attrs = vec!["B.a:0"];
    let policy = "A.a:0 | B.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_disjunction_both_ok() {
    let user_attrs = vec!["A.a:0", "B.a:0"];
    let policy = "A.a:0 | B.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_disjunction_wrong_auth_fail() {
    let user_attrs = vec!["C.a:0"];
    let policy = "A.a:0 | B.a:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_conjunction_ok() {
    let user_attrs = vec!["A.a:0", "B.a:0"];
    let policy = "A.a:0 & B.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_conjunction_missing_left_fail() {
    let user_attrs = vec!["B.a:0"];
    let policy = "A.a:0 & B.a:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_conjunction_missing_right_fail() {
    let user_attrs = vec!["A.a:0"];
    let policy = "A.a:0 & B.a:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_negation_ok() {
    let user_attrs = vec!["A.a:1", "A.a:2", "A.a:3", "A.a:4", "A.a:5", "B.a:0"];
    let policy = "!A.a:0";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_negation_cross_auth_fail() {
    let user_attrs = vec!["B.a:0"];
    let policy = "!A.a:0";
    assert_decryption_fail(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_complex_1_ok() {
    let user_attrs = vec!["A.a:0", "A.b:2", "A.c:1", "B.b:0", "B.b:1"];
    let policy = "A.a:1 | (!A.a:1 & A.b:2) & !(B.b:2 | A.c:2)";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_complex_2_ok() {
    let user_attrs = vec!["A.a:2", "A.b:1", "A.c:0", "B.c:0", "C.c:0", "C.c:1"];
    let policy = "A.a:0 | (!A.b:0 & A.a:2) & !(B.c:1 | A.c:2)";
    assert_decryption_ok(user_attrs, policy);
}

#[test]
fn opt5_multi_auth_complex_fail() {
    let user_attrs = vec!["A.a:2", "A.c:1", "B.c:2"];
    let policy = "A.a:0 | (!A.b:0 & A.a:2) & !(A.c:1 | A.c:2)";
    assert_decryption_fail(user_attrs, policy);
}
