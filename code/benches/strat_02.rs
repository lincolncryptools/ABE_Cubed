use std::time::Duration;

use abe_cubed::bench::{BenchParams, InputGenerator};
use abe_cubed::scheme::{Opt0, Opt1, Opt2, Opt3, Opt4, Opt5, Opt6, Scheme};
use criterion::measurement::Measurement;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion};

#[path = "./common.rs"]
mod common;
use common::{common_bench_decrypt, common_bench_encrypt, common_bench_keygen, common_bench_setup};

fn get_params() -> BenchParams {
    BenchParams::large_strat_02()
}

fn get_input_generators(params: &BenchParams) -> Vec<InputGenerator> {
    let mut gens = Vec::new();
    for &degree in params.neg_degrees.iter() {
        let mut tmp = vec![
            InputGenerator::vary_auth(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            ),
            InputGenerator::vary_lbl(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            ),
            InputGenerator::vary_attr(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            ),
            InputGenerator::vary_auth_and_lbl(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            ),
            InputGenerator::vary_auth_and_attr(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            ),
            InputGenerator::vary_lbl_and_attr(
                params.policy_len,
                degree,
                params.test_sizes.clone(),
            )
        ];
        gens.append(&mut tmp);
    }
    gens
}

fn config_benchmarks<'a, M: Measurement>(group: &mut BenchmarkGroup<'a, M>) {
    group.measurement_time(Duration::new(3, 0));
    group.sample_size(40);
    group.warm_up_time(Duration::new(1, 0));
    group.sampling_mode(criterion::SamplingMode::Flat);
}

pub fn bench_setup<T: Scheme>(scheme: &T, c: &mut Criterion) {
    let mut group = c.benchmark_group("setup");
    config_benchmarks(&mut group);
    let params = get_params();
    let input_gens = get_input_generators(&params);
    common_bench_setup(scheme, params, input_gens, &mut group);
    group.finish();
}

pub fn bench_keygen<T: Scheme>(scheme: &T, c: &mut Criterion) {
    let mut group = c.benchmark_group("keygen");
    config_benchmarks(&mut group);
    let params = get_params();
    let input_gens = get_input_generators(&params);
    common_bench_keygen(scheme, params, input_gens, &mut group);
    group.finish();
}

pub fn bench_encrypt<T: Scheme>(scheme: &T, c: &mut Criterion) {
    let mut group = c.benchmark_group("encrypt");
    config_benchmarks(&mut group);
    let params = get_params();
    let input_gens = get_input_generators(&params);
    common_bench_encrypt(scheme, params, input_gens, &mut group);
    group.finish();
}

pub fn bench_decrypt<T: Scheme>(scheme: &T, c: &mut Criterion) {
    let mut group = c.benchmark_group("decrypt");
    config_benchmarks(&mut group);
    let params = get_params();
    let input_gens = get_input_generators(&params);
    common_bench_decrypt(scheme, params, input_gens, &mut group);
    group.finish();
}

pub fn bench_setup_all(c: &mut Criterion) {
    bench_setup(&Opt0::new(), c);
    bench_setup(&Opt1::new(), c);
    bench_setup(&Opt2::new(), c);
    bench_setup(&Opt3::new(), c);
    bench_setup(&Opt4::new(), c);
    bench_setup(&Opt5::new(), c);
    bench_setup(&Opt6::new(), c);
}

pub fn bench_keygen_all(c: &mut Criterion) {
    bench_keygen(&Opt0::new(), c);
    bench_keygen(&Opt1::new(), c);
    bench_keygen(&Opt2::new(), c);
    bench_keygen(&Opt3::new(), c);
    bench_keygen(&Opt4::new(), c);
    bench_keygen(&Opt5::new(), c);
    bench_keygen(&Opt6::new(), c);
}

pub fn bench_encrypt_all(c: &mut Criterion) {
    bench_encrypt(&Opt0::new(), c);
    bench_encrypt(&Opt1::new(), c);
    bench_encrypt(&Opt2::new(), c);
    bench_encrypt(&Opt3::new(), c);
    bench_encrypt(&Opt4::new(), c);
    bench_encrypt(&Opt5::new(), c);
    bench_encrypt(&Opt6::new(), c);
}

pub fn bench_decrypt_all(c: &mut Criterion) {
    bench_decrypt(&Opt0::new(), c);
    bench_decrypt(&Opt1::new(), c);
    bench_decrypt(&Opt2::new(), c);
    bench_decrypt(&Opt3::new(), c);
    bench_decrypt(&Opt4::new(), c);
    bench_decrypt(&Opt5::new(), c);
    bench_decrypt(&Opt6::new(), c);
}

criterion_group!(
    benches,
    bench_setup_all,
    bench_keygen_all,
    bench_encrypt_all,
    bench_decrypt_all
);
criterion_main!(benches);
