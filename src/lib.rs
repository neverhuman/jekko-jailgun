/// Canonical identity for the jekko-jailgun split-family checkout.
pub const REPOSITORY: &str = "jekko-jailgun";

/// Role recorded in the split-family manifest.
pub const ROLE: &str = "web";

/// Profile recorded in the split-family manifest.
pub const PROFILE: &str = "rust-web";

/// Return the repo identity tuple used by the smoke tests.
pub fn identity() -> (&'static str, &'static str, &'static str) {
    (REPOSITORY, ROLE, PROFILE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_stable() {
        assert_eq!(identity(), (REPOSITORY, ROLE, PROFILE));
    }
}
