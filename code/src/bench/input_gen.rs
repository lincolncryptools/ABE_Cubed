use std::{cmp, collections::HashSet};

use crate::policy::{Policy, UserAttribute};

struct Fixed {
    policy_lens: Vec<usize>,
    neg_degree: usize,
    curr: usize,
}

impl Fixed {
    fn new(policy_lens: Vec<usize>, neg_degree: usize) -> Self {
        Fixed {
            policy_lens,
            neg_degree,
            curr: 0,
        }
    }
}

impl Iterator for Fixed {
    type Item = (usize, Vec<UserAttribute>, Policy, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.policy_lens.len() {
            return None;
        }
        let policy_len = self.policy_lens[self.curr];
        self.curr += 1;

        let mut universe = Vec::new();
        for j in 0..policy_len {
            let auth = format!("auth_{}", j);
            let lbl = format!("lbl_{}", j);
            let attr = format!("attr_{}", j);
            universe.push(UserAttribute::new(&auth, &lbl, &attr));
        }
        let num_negs = if self.neg_degree == 0 {
            0
        } else {
            universe.len()
        };
        let policy = Policy::conjunction_of(&universe, num_negs);
        let mut user_attrs = Vec::new();
        for attr in universe {
            if self.neg_degree == 0 {
                user_attrs.push(attr);
            } else {
                for i in 0..self.neg_degree {
                    let alt_attr = format!("{}_{}", &attr.attr, i);
                    user_attrs.push(UserAttribute::new(&attr.auth, &attr.lbl, &alt_attr));
                }
            }
        }
        return Some((policy_len, user_attrs, policy, self.neg_degree));
    }
}

struct UnaryVariation {
    test_item_counts: Vec<usize>,
    test_items: Vec<String>,
    universe: Vec<(String, String)>,
    num_negs: usize,
    neg_degree: usize,
    curr: usize,
    merge_fn: fn(Vec<(String, String)>, String) -> Vec<UserAttribute>,
}

impl UnaryVariation {
    fn new(
        universe_size: usize,
        neg_degree: usize,
        universe_lbls: (&str, &str),
        test_item_counts: Vec<usize>,
        item_lbl: &str,
        merge_fn: fn(Vec<(String, String)>, String) -> Vec<UserAttribute>,
    ) -> Self {
        let max_item_count = *test_item_counts.iter().max().unwrap();
        let max_num = cmp::max(universe_size, max_item_count);
        let fmt_width = ((max_num as f64).log10() + 1.0) as usize;
        let (s1, s2) = universe_lbls;
        let universe = UnaryVariation::build_binary_universe(universe_size, s1, s2, fmt_width);
        let test_items =
            UnaryVariation::build_unary_test_items(max_item_count, item_lbl, fmt_width);
        let num_negs = if neg_degree != 0 { universe_size } else { 0 };
        UnaryVariation {
            test_item_counts,
            test_items,
            universe,
            num_negs,
            neg_degree,
            curr: 0,
            merge_fn,
        }
    }

    fn build_unary_test_items(count: usize, s: &str, width: usize) -> Vec<String> {
        let mut test_items = Vec::new();
        for j in 0..count {
            let x = format!("{}_{:0width$}", s, j, width = width);
            test_items.push(x);
        }
        test_items
    }

    fn build_binary_universe(
        size: usize,
        s1: &str,
        s2: &str,
        width: usize,
    ) -> Vec<(String, String)> {
        let mut universe = Vec::new();
        for j in 0..size {
            let x = format!("{}_{:0width$}", s1, j, width = width);
            let y = format!("{}_{:0width$}", s2, j, width = width);
            universe.push((x, y));
        }
        universe
    }
}

impl Iterator for UnaryVariation {
    type Item = (usize, Vec<UserAttribute>, Policy, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.test_item_counts.len() {
            return None;
        }
        let n = self.test_item_counts[self.curr];
        self.curr += 1;
        let test_items = self.test_items.clone().into_iter().take(n).collect();
        let user_attrs = distribute(&self.universe, &test_items, self.merge_fn);
        let policy = Policy::conjunction_of(&user_attrs, self.num_negs);
        let mut user_attrs = Vec::new();
        let mut auth_lbls_done: HashSet<(String, String)> = HashSet::new();
        for i in 0..policy.len() {
            let (ua, neg) = policy.get(i);
            if neg && !auth_lbls_done.contains(&(ua.auth.clone(), ua.lbl.clone())) {
                for j in 0..self.neg_degree {
                    let alt_attr = format!("{}_{}", ua.attr, j);
                    let ua = UserAttribute::new(&ua.auth, &ua.lbl, &alt_attr);
                    user_attrs.push(ua);
                }
                auth_lbls_done.insert((ua.auth.clone(), ua.lbl.clone()));
            } else if !neg {
                user_attrs.push(ua);
            }
        }
        return Some((n, user_attrs, policy, self.neg_degree));
    }
}

pub struct InputGenerator {
    descr: String,
    variation: Result<Result<UnaryVariation, BinaryVariation>, Fixed>,
}

impl InputGenerator {
    pub fn vary_size(universe_sizes: Vec<usize>, neg_degree: usize) -> Self {
        InputGenerator {
            descr: String::from("vary_size"),
            variation: Err(Fixed::new(universe_sizes, neg_degree)),
        }
    }

    pub fn vary_auth(
        universe_size: usize,
        neg_degree: usize,
        test_item_counts: Vec<usize>,
    ) -> Self {
        fn merge_fn(xy: Vec<(String, String)>, z: String) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            for (x, y) in xy {
                let ua = UserAttribute::new(&z, &x, &y);
                res.push(ua);
            }
            res
        }
        let unary_var = UnaryVariation::new(
            universe_size,
            neg_degree,
            ("lbl", "attr"),
            test_item_counts,
            "auth",
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_auth"),
            variation: Ok(Ok(unary_var)),
        }
    }

    pub fn vary_lbl(universe_size: usize, neg_degree: usize, test_item_counts: Vec<usize>) -> Self {
        fn merge_fn(xy: Vec<(String, String)>, z: String) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            for (x, y) in xy {
                let ua = UserAttribute::new(&x, &z, &y);
                res.push(ua);
            }
            res
        }
        let unary_var = UnaryVariation::new(
            universe_size,
            neg_degree,
            ("auth", "attr"),
            test_item_counts,
            "lbl",
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_lbl"),
            variation: Ok(Ok(unary_var)),
        }
    }

    pub fn vary_attr(
        universe_size: usize,
        neg_degree: usize,
        test_item_counts: Vec<usize>,
    ) -> Self {
        fn merge_fn(xy: Vec<(String, String)>, z: String) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            for (x, y) in xy {
                let ua = UserAttribute::new(&x, &y, &z);
                res.push(ua);
            }
            res
        }
        let unary_var = UnaryVariation::new(
            universe_size,
            neg_degree,
            ("auth", "lbl"),
            test_item_counts,
            "attr",
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_attr"),
            variation: Ok(Ok(unary_var)),
        }
    }

    pub fn vary_auth_and_attr(
        universe_size: usize,
        neg_degree: usize,
        test_item_counts: Vec<usize>,
    ) -> Self {
        fn merge_fn(zs: Vec<String>, xy: (String, String)) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            let (x, y) = xy;
            for z in zs {
                let ua = UserAttribute::new(&x, &z, &y);
                res.push(ua);
            }
            res
        }
        let binary_var = BinaryVariation::new(
            universe_size,
            neg_degree,
            "lbl",
            test_item_counts,
            ("auth", "attr"),
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_auth_attr"),
            variation: Ok(Err(binary_var)),
        }
    }

    pub fn vary_auth_and_lbl(
        universe_size: usize,
        neg_degree: usize,
        test_item_counts: Vec<usize>,
    ) -> Self {
        fn merge_fn(zs: Vec<String>, xy: (String, String)) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            let (x, y) = xy;
            for z in zs {
                let ua = UserAttribute::new(&x, &y, &z);
                res.push(ua);
            }
            res
        }
        let binary_var = BinaryVariation::new(
            universe_size,
            neg_degree,
            "attr",
            test_item_counts,
            ("auth", "lbl"),
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_auth_lbl"),
            variation: Ok(Err(binary_var)),
        }
    }

    pub fn vary_lbl_and_attr(
        universe_size: usize,
        neg_degree: usize,
        test_item_counts: Vec<usize>,
    ) -> Self {
        fn merge_fn(zs: Vec<String>, xy: (String, String)) -> Vec<UserAttribute> {
            let mut res = Vec::new();
            let (x, y) = xy;
            for z in zs {
                let ua = UserAttribute::new(&z, &x, &y);
                res.push(ua);
            }
            res
        }
        let binary_var = BinaryVariation::new(
            universe_size,
            neg_degree,
            "auth",
            test_item_counts,
            ("lbl", "attr"),
            merge_fn,
        );
        InputGenerator {
            descr: String::from("vary_lbl_attr"),
            variation: Ok(Err(binary_var)),
        }
    }

    pub fn get_descr(&self) -> String {
        self.descr.clone()
    }
}

impl Iterator for InputGenerator {
    type Item = (usize, Vec<UserAttribute>, Policy, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.variation.as_mut() {
            Ok(Ok(var)) => var.next(),
            Ok(Err(var)) => var.next(),
            Err(var) => var.next(),
        }
    }
}

struct BinaryVariation {
    test_item_counts: Vec<usize>,
    test_items: Vec<(String, String)>,
    universe: Vec<String>,
    num_negs: usize,
    neg_degree: usize,
    curr: usize,
    merge_fn: fn(Vec<String>, (String, String)) -> Vec<UserAttribute>,
}

impl BinaryVariation {
    pub fn new(
        universe_size: usize,
        neg_degree: usize,
        universe_lbl: &str,
        test_item_counts: Vec<usize>,
        item_lbls: (&str, &str),
        merge_fn: fn(Vec<String>, (String, String)) -> Vec<UserAttribute>,
    ) -> Self {
        let max_item_count = *test_item_counts.iter().max().unwrap();
        let max_num = cmp::max(universe_size, max_item_count);
        let fmt_width = ((max_num as f64).log10() + 1.0) as usize;
        let (s1, s2) = item_lbls;
        let universe =
            BinaryVariation::build_unary_universe(universe_size, universe_lbl, fmt_width);
        let test_items =
            BinaryVariation::build_binary_test_items(max_item_count, s1, s2, fmt_width);
        let num_negs = if neg_degree != 0 { universe_size } else { 0 };
        BinaryVariation {
            test_item_counts,
            test_items,
            universe,
            num_negs,
            neg_degree,
            curr: 0,
            merge_fn,
        }
    }

    fn build_unary_universe(size: usize, s1: &str, width: usize) -> Vec<String> {
        let mut universe = Vec::new();
        for j in 0..size {
            let x = format!("{}_{:0width$}", s1, j, width = width);
            universe.push(x);
        }
        universe
    }

    fn build_binary_test_items(
        count: usize,
        s1: &str,
        s2: &str,
        width: usize,
    ) -> Vec<(String, String)> {
        let mut test_items = Vec::new();
        for j in 0..count {
            let x = format!("{}_{:0width$}", s1, j, width = width);
            let y = format!("{}_{:0width$}", s2, j, width = width);
            test_items.push((x, y));
        }
        test_items
    }
}

impl Iterator for BinaryVariation {
    type Item = (usize, Vec<UserAttribute>, Policy, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.test_item_counts.len() {
            return None;
        }
        let n = self.test_item_counts[self.curr];
        self.curr += 1;
        let test_items = self.test_items.clone().into_iter().take(n).collect();
        let user_attrs = distribute(&self.universe, &test_items, self.merge_fn);
        let policy = Policy::conjunction_of(&user_attrs, self.num_negs);
        let mut user_attrs = Vec::new();
        let mut auth_lbls_done: HashSet<(String, String)> = HashSet::new();
        for i in 0..policy.len() {
            let (ua, neg) = policy.get(i);
            if neg && !auth_lbls_done.contains(&(ua.auth.clone(), ua.lbl.clone())) {
                for j in 0..self.neg_degree {
                    let alt_attr = format!("{}_{}", ua.attr, j);
                    let ua = UserAttribute::new(&ua.auth, &ua.lbl, &alt_attr);
                    user_attrs.push(ua);
                }
                auth_lbls_done.insert((ua.auth.clone(), ua.lbl.clone()));
            } else if !neg {
                user_attrs.push(ua);
            }
        }
        return Some((n, user_attrs, policy, self.neg_degree));
    }
}

// fn distribute<T: Clone, V: Clone, U>(
//     items: &Vec<T>,
//     groups: &Vec<V>,
//     func: fn(Vec<T>, V) -> Vec<U>,
// ) -> Vec<U> {
//     if items.len() < groups.len() || items.len() == 0 {
//         panic!("Empty groups are not allowed");
//     }
//     let group_size = items.len() / groups.len(); // always >= 1
//     let remainder = items.len() % groups.len();
//     let mut res = Vec::new();
//     // first groups are slightly larger because of uneven division
//     for j in 0..remainder {
//         let group = groups[j].clone();
//         let group_size = group_size + 1; // additional element
//         let lower = j * group_size;
//         let upper = lower + group_size ;
//         let items = items[lower..upper].to_vec();
//         res.extend(func(items, group));
//     }
//     // last group(s) have the expected size
//     for j in 0..(groups.len() - remainder) {
//         let group = groups[j + remainder].clone();
//         let lower =  remainder * (group_size + 1) + j * group_size;
//         let upper = lower + group_size ;
//         let items = items[lower..upper].to_vec();
//         res.extend(func(items, group));
//     }
//     res
// }

fn distribute<T: Clone, V: Clone, U>(
    items: &Vec<T>,
    groups: &Vec<V>,
    func: fn(Vec<T>, V) -> Vec<U>,
) -> Vec<U> {
    if items.len() < groups.len() || items.len() == 0 {
        panic!("Empty groups are not allowed");
    } else if items.len() % groups.len() != 0 {
        panic!("Items cannot be distributed evenly over given groups");
    }
    let group_size = items.len() / groups.len(); // always >= 1
    items
        .chunks_exact(group_size)
        .into_iter()
        .zip(groups)
        .map(|(is, g)| func(is.to_vec(), g.clone()))
        .flatten()
        .collect()
}
