use std::vec;

use abe_cubed::{
    curve,
    policy::{Policy, UserAttribute},
    scheme::{Iota, Opt0, Scheme, Tau},
};

fn main() {
    let mut rng = ark_std::test_rng();

    let scheme = Opt0::new();

    let auths = vec!["vacation"];

    let (msk, mpk) = scheme.setup(&mut rng, &auths);

    let user_id = "surfer";
    let user_attrs = vec![
        UserAttribute::new("vacation", "weather", "sunny"),
        UserAttribute::new("vacation", "location", "beach"),
    ];
    let iota = Iota::new(&user_attrs);

    let usk = scheme.keygen(&mut rng, user_id, &msk, &user_attrs, &iota);

    let policy =
        Policy::parse("vacation.weather:sunny & vacation.location:beach & !vacation.weather:windy")
            .unwrap();

    let tau = Tau::new(&policy);
    let (k_enc, ct) = scheme.encrypt(&mut rng, &mpk, &policy, &tau);

    let k_dec = scheme.decrypt(&usk, user_id, &iota, &tau, &policy, &ct);

    assert!(k_dec.is_some_and(|k| curve::Gt::eq(&k_enc, &k)));

    let user_id = "hiker";
    let user_attrs = vec![
        UserAttribute::new("vacation", "weather", "sunny"),
        UserAttribute::new("vacation", "location", "mountain"),
    ];
    let iota = Iota::new(&user_attrs);

    let usk = scheme.keygen(&mut rng, user_id, &msk, &user_attrs, &iota);

    let k_dec = scheme.decrypt(&usk, user_id, &iota, &tau, &policy, &ct);

    assert!(k_dec.is_none());
}
