use std::collections::HashSet;

use abe_cubed::bench::{BenchParams, InputGenerator};
use abe_cubed::scheme::{Iota, Scheme, Tau};
use criterion::measurement::WallTime;
use criterion::{black_box, BenchmarkGroup};

const USER_ID: &str = "GLOBAL_USER_ID";

pub fn common_bench_setup<T: Scheme>(
    scheme: &T,
    params: BenchParams,
    input_gens: Vec<InputGenerator>,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let mut rng = ark_std::test_rng();

    for test_cases in input_gens {
        let test_param = test_cases.get_descr();

        for (test_size, user_attrs, _, neg_degree) in test_cases {
            let auths = user_attrs
                .clone()
                .iter()
                .map(|ua| ua.auth.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

            let auths = auths.iter().map(|s| &s as &str).collect();

            let descr = params.describe(&scheme.get_name(), &test_param, test_size, neg_degree);

            group.bench_function(descr, |b| {
                b.iter_with_large_drop(|| scheme.setup(black_box(&mut rng), black_box(&auths)))
            });
        }
    }
}

pub fn common_bench_keygen<T: Scheme>(
    scheme: &T,
    params: BenchParams,
    input_gens: Vec<InputGenerator>,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let mut rng = ark_std::test_rng();

    for test_cases in input_gens {
        let test_param = test_cases.get_descr();

        for (test_size, user_attrs, _, neg_degree) in test_cases {
            let auths = user_attrs
                .clone()
                .iter()
                .map(|ua| ua.auth.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

            let auths = auths.iter().map(|s| &s as &str).collect();

            let (msk, _) = scheme.setup(&mut rng, &auths);

            let iota = Iota::new(&user_attrs);

            let descr = params.describe(&scheme.get_name(), &test_param, test_size, neg_degree);
            group.bench_function(descr, |b| {
                b.iter_with_large_drop(|| {
                    scheme.keygen(
                        black_box(&mut rng),
                        black_box(USER_ID),
                        black_box(&msk),
                        black_box(&user_attrs),
                        black_box(&iota),
                    )
                })
            });
        }
    }
}

pub fn common_bench_encrypt<T: Scheme>(
    scheme: &T,
    params: BenchParams,
    input_gens: Vec<InputGenerator>,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let mut rng = ark_std::test_rng();

    for test_cases in input_gens {
        let test_param = test_cases.get_descr();

        for (test_size, user_attrs, policy, neg_degree) in test_cases {
            let auths = user_attrs
                .clone()
                .iter()
                .map(|ua| ua.auth.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

            let auths = auths.iter().map(|s| &s as &str).collect();

            let (_, mpk) = scheme.setup(&mut rng, &auths);

            let tau = Tau::new(&policy);

            let descr = params.describe(&scheme.get_name(), &test_param, test_size, neg_degree);
            group.bench_function(descr, |b| {
                b.iter_with_large_drop(|| {
                    scheme.encrypt(
                        black_box(&mut rng),
                        black_box(&mpk),
                        black_box(&policy),
                        black_box(&tau),
                    )
                })
            });
        }
    }
}

pub fn common_bench_decrypt<T: Scheme>(
    scheme: &T,
    params: BenchParams,
    input_gens: Vec<InputGenerator>,
    group: &mut BenchmarkGroup<'_, WallTime>,
) {
    let mut rng = ark_std::test_rng();

    for test_cases in input_gens {
        let test_param = test_cases.get_descr();

        for (test_size, user_attrs, policy, neg_degree) in test_cases {
            let auths = user_attrs
                .clone()
                .iter()
                .map(|ua| ua.auth.clone())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

            let auths = auths.iter().map(|s| &s as &str).collect();

            let (msk, mpk) = scheme.setup(&mut rng, &auths);
            let iota = Iota::new(&user_attrs);
            let usk = scheme.keygen(&mut rng, USER_ID, &msk, &user_attrs, &iota);
            let tau = Tau::new(&policy);
            let (_, ct) = scheme.encrypt(&mut rng, &mpk, &policy, &tau);

            let descr = params.describe(&scheme.get_name(), &test_param, test_size, neg_degree);
            group.bench_function(descr, |b| {
                b.iter_with_large_drop(|| {
                    scheme.decrypt(
                        black_box(&usk),
                        black_box(USER_ID),
                        black_box(&iota),
                        black_box(&tau),
                        black_box(&policy),
                        black_box(&ct),
                    )
                })
            });
        }
    }
}
