use abe_cubed::{
    bench::InputGenerator,
    policy::{Policy, UserAttribute},
};

#[test]
fn test_vary_auth_lbl() {
    let size = 6;
    let num_auths = vec![1, 2, 3, 6];
    let neg_degree = 2;
    let vary_auth_lbl = InputGenerator::vary_auth_and_lbl(size, neg_degree, num_auths);
    let inputs: Vec<(usize, Vec<UserAttribute>, Policy, _)> = vary_auth_lbl.collect();
    assert_eq!(4, inputs.len());
    // 1 authority-label pair
    let (size, user_attrs, policy, _) = &inputs[0];
    assert_eq!(1, *size);
    assert_eq!(2, user_attrs.len()); // 2 (since alternatives for one negated attribute can be reused)
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_0.lbl_0:attr_1 &
        !auth_0.lbl_0:attr_2 &
        !auth_0.lbl_0:attr_3 &
        !auth_0.lbl_0:attr_4 &
        !auth_0.lbl_0:attr_5"
        )
        .unwrap(),
        *policy
    );

    // 2 authority-label pairs
    let (size, user_attrs, policy, _) = &inputs[1];
    assert_eq!(2, *size);
    assert_eq!(4, user_attrs.len()); // 4 (since alternatives can now only be reused in half as many cases)
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_3_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_3_1"),
        user_attrs[3]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_0.lbl_0:attr_1 &
        !auth_0.lbl_0:attr_2 &
        !auth_1.lbl_1:attr_3 &
        !auth_1.lbl_1:attr_4 &
        !auth_1.lbl_1:attr_5"
        )
        .unwrap(),
        *policy
    );

    // 3 authority-label pairs
    let (size, user_attrs, policy, _) = &inputs[2];
    assert_eq!(3, *size);
    assert_eq!(6, user_attrs.len()); // 6 = (since alternatives can now only be reused in a third of the cases)
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_2_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_2_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_4_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_4_1"),
        user_attrs[5]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_0.lbl_0:attr_1 &
        !auth_1.lbl_1:attr_2 &
        !auth_1.lbl_1:attr_3 &
        !auth_2.lbl_2:attr_4 &
        !auth_2.lbl_2:attr_5"
        )
        .unwrap(),
        *policy
    );

    // 6 authority-label pairs
    let (size, user_attrs, policy, _) = &inputs[3];
    assert_eq!(6, *size);
    assert_eq!(12, user_attrs.len()); // 12 = size * neg_degree (no reuse possible)
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
        UserAttribute::new("auth_4", "lbl_4", "attr_4_0"),
        user_attrs[8]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_4_1"),
        user_attrs[9]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_5_0"),
        user_attrs[10]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_5_1"),
        user_attrs[11]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_1.lbl_1:attr_1 &
        !auth_2.lbl_2:attr_2 &
        !auth_3.lbl_3:attr_3 &
        !auth_4.lbl_4:attr_4 &
        !auth_5.lbl_5:attr_5"
        )
        .unwrap(),
        *policy
    );
}
