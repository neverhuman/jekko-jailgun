use jekko_jailgun::{identity, validate_identity};

#[test]
fn public_identity_contract_is_stable() {
    validate_identity().expect("identity validates");
    let (repo, role, profile) = identity();
    assert_eq!(repo, "jekko-jailgun");
    assert_eq!(role, "web");
    assert_eq!(profile, "rust-web");
}
