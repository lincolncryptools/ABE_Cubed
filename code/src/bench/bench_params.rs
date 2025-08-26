pub struct BenchParams {
    pub policy_len: usize,
    pub test_sizes: Vec<usize>,
    pub neg_degrees: Vec<usize>,
}

impl BenchParams {
    pub fn micro() -> Self {
        BenchParams {
            policy_len: 6,
            test_sizes: vec![1, 2, 3, 6],
            neg_degrees: vec![0, 1],
        }
    }

    pub fn small() -> Self {
        BenchParams {
            policy_len: 12,
            test_sizes: vec![1, 2, 3, 4, 6, 12],
            neg_degrees: vec![0, 2],
        }
    }

    pub fn medium() -> Self {
        BenchParams {
            policy_len: 30,
            test_sizes: vec![1, 2, 3, 5, 6, 10, 15, 30],
            neg_degrees: vec![0, 3, 6, 9],
        }
    }

    pub fn large_strat_01a() -> Self {
        BenchParams {
            policy_len: 100,
            test_sizes: vec![],  // ignored
            neg_degrees: vec![], // ignored
        }
    }

    pub fn large_strat_01b() -> Self {
        BenchParams {
            policy_len: 50,
            test_sizes: vec![],  // ignored
            neg_degrees: vec![], // ignored
        }
    }

    pub fn large_strat_02() -> Self {
        BenchParams {
            policy_len: 60,
            test_sizes: vec![1, 2, 3, 4, 5, 6, 10, 12, 30, 60],
            neg_degrees: vec![0, 1, 4, 7],
        }
    }

    pub fn huge() -> Self {
        BenchParams {
            policy_len: 120,
            test_sizes: vec![1, 2, 3, 4, 5, 6, 8, 10, 12, 15, 24, 30, 40, 60, 120],
            neg_degrees: vec![0, 20],
        }
    }

    pub fn describe(
        &self,
        scheme_variant: &str,
        test_param: &str,
        test_size: usize,
        neg_degree: usize,
    ) -> String {
        format!(
            "{}_{}_{}_{}_{}",
            scheme_variant, test_param, test_size, self.policy_len, neg_degree
        )
    }
}
