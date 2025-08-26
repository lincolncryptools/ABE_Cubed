use abe_cubed::{
    bench::InputGenerator,
    policy::{Policy, UserAttribute},
};

#[test]
fn test_vary_size() {
    let universe_sizes = vec![1, 2, 3, 4];
    let neg_degree = 2;
    let vary_size = InputGenerator::vary_size(universe_sizes, neg_degree);
    let inputs: Vec<(usize, Vec<UserAttribute>, Policy, _)> = vary_size.collect();
    assert_eq!(4, inputs.len());
    // 1 entry
    let (size, user_attrs, policy, _) = &inputs[0];
    assert_eq!(1, *size);
    assert_eq!(2, user_attrs.len()); // 2 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(Policy::parse("!auth_0.lbl_0:attr_0").unwrap(), *policy);

    // 2 entries
    let (size, user_attrs, policy, _) = &inputs[1];
    assert_eq!(2, *size);
    assert_eq!(4, user_attrs.len()); // 4 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_1"),
        user_attrs[3]
    );
    assert_eq!(
        Policy::parse("!auth_0.lbl_0:attr_0 & !auth_1.lbl_1:attr_1").unwrap(),
        *policy
    );

    // 3 entries
    let (size, user_attrs, policy, _) = &inputs[2];
    assert_eq!(3, *size);
    assert_eq!(6, user_attrs.len()); // 6 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_2_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_2_1"),
        user_attrs[5]
    );
    assert_eq!(
        Policy::parse("!auth_0.lbl_0:attr_0 & !auth_1.lbl_1:attr_1 & !auth_2.lbl_2:attr_2")
            .unwrap(),
        *policy
    );

    // 4 entries
    let (size, user_attrs, policy, _) = &inputs[3];
    assert_eq!(4, *size);
    assert_eq!(8, user_attrs.len()); // 8 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_1_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_2_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_2_1"),
        user_attrs[5]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_3_0"),
        user_attrs[6]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_3_1"),
        user_attrs[7]
    );
    assert_eq!(
        Policy::parse("!auth_0.lbl_0:attr_0 & !auth_1.lbl_1:attr_1 & !auth_2.lbl_2:attr_2 & !auth_3.lbl_3:attr_3")
        .unwrap(),
        *policy
    );
}
