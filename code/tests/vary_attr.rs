use abe_cubed::{
    bench::InputGenerator,
    policy::{Policy, UserAttribute},
};

#[test]
fn test_vary_attr() {
    let size = 6;
    let num_auths = vec![1, 2, 3, 6];
    let neg_degree = 2;
    let vary_attr = InputGenerator::vary_attr(size, neg_degree, num_auths);
    let inputs: Vec<(usize, Vec<UserAttribute>, Policy, _)> = vary_attr.collect();
    assert_eq!(4, inputs.len());
    // 1 attribute
    let (size, user_attrs, policy, _) = &inputs[0];
    assert_eq!(1, *size);
    assert_eq!(12, user_attrs.len()); // 12 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_0_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_0_1"),
        user_attrs[5]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_0_0"),
        user_attrs[6]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_0_1"),
        user_attrs[7]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_0_0"),
        user_attrs[8]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_0_1"),
        user_attrs[9]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_0_0"),
        user_attrs[10]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_0_1"),
        user_attrs[11]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_1.lbl_1:attr_0 &
        !auth_2.lbl_2:attr_0 &
        !auth_3.lbl_3:attr_0 &
        !auth_4.lbl_4:attr_0 &
        !auth_5.lbl_5:attr_0"
        )
        .unwrap(),
        *policy
    );

    // 2 attributes
    let (size, user_attrs, policy, _) = &inputs[1];
    assert_eq!(2, *size);
    assert_eq!(12, user_attrs.len()); // 12 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_0_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_0_1"),
        user_attrs[5]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_1_0"),
        user_attrs[6]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_1_1"),
        user_attrs[7]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_1_0"),
        user_attrs[8]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_1_1"),
        user_attrs[9]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_1_0"),
        user_attrs[10]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_1_1"),
        user_attrs[11]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_1.lbl_1:attr_0 &
        !auth_2.lbl_2:attr_0 &
        !auth_3.lbl_3:attr_1 &
        !auth_4.lbl_4:attr_1 &
        !auth_5.lbl_5:attr_1"
        )
        .unwrap(),
        *policy
    );

    // 3 attributes
    let (size, user_attrs, policy, _) = &inputs[2];
    assert_eq!(3, *size);
    assert_eq!(12, user_attrs.len()); // 12 = size * neg_degree
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_0"),
        user_attrs[0]
    );
    assert_eq!(
        UserAttribute::new("auth_0", "lbl_0", "attr_0_1"),
        user_attrs[1]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_0"),
        user_attrs[2]
    );
    assert_eq!(
        UserAttribute::new("auth_1", "lbl_1", "attr_0_1"),
        user_attrs[3]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_1_0"),
        user_attrs[4]
    );
    assert_eq!(
        UserAttribute::new("auth_2", "lbl_2", "attr_1_1"),
        user_attrs[5]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_1_0"),
        user_attrs[6]
    );
    assert_eq!(
        UserAttribute::new("auth_3", "lbl_3", "attr_1_1"),
        user_attrs[7]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_2_0"),
        user_attrs[8]
    );
    assert_eq!(
        UserAttribute::new("auth_4", "lbl_4", "attr_2_1"),
        user_attrs[9]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_2_0"),
        user_attrs[10]
    );
    assert_eq!(
        UserAttribute::new("auth_5", "lbl_5", "attr_2_1"),
        user_attrs[11]
    );
    assert_eq!(
        Policy::parse(
            "
        !auth_0.lbl_0:attr_0 &
        !auth_1.lbl_1:attr_0 &
        !auth_2.lbl_2:attr_1 &
        !auth_3.lbl_3:attr_1 &
        !auth_4.lbl_4:attr_2 &
        !auth_5.lbl_5:attr_2"
        )
        .unwrap(),
        *policy
    );

    // 6 attributes
    let (size, user_attrs, policy, _) = &inputs[3];
    assert_eq!(6, *size);
    assert_eq!(12, user_attrs.len()); // 12 = size * neg_degree
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
