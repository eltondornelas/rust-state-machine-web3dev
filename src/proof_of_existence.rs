use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call,
    ) -> crate::support::DispatchResult {
        match call {
            Call::CreateClaim { claim: content } => self.create_claim(caller, content)?,
            Call::RevokeClaim { claim: content } => self.revoke_claim(caller, content)?,
        }
        Ok(())
    }
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Pallet {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }

        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.claims.get(&claim).ok_or("claim not found")?;

        if owner != &caller {
            return Err("this claim is owned by someone else");
        }

        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn create_and_claim_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();
        let alice = "alice";
        let content = "Hello";

        assert!(poe.get_claim(&content).is_none());

        let _ = poe.create_claim(alice, content);
        assert_eq!(poe.get_claim(&content), Some(&alice));
    }

    #[test]
    fn cant_claim_existing_claim() {
        let alice = "alice";
        let bob = "bob";
        let content = "Hello";
        let mut poe = super::Pallet::<TestConfig>::new();

        let _ = poe.create_claim(alice, content);
        let cant_claim = poe.create_claim(bob, content);
        assert!(cant_claim.is_err());
        assert_eq!(cant_claim.unwrap_err(), "this content is already claimed");
    }

    #[test]
    fn revoke_claim() {
        let alice = "alice";
        let bob = "bob";
        let mut poe = super::Pallet::<TestConfig>::new();
        let content = "Hello";

        let _ = poe.create_claim(alice, content);
        let cant_revoke_others_claim = poe.revoke_claim(bob, content);

        assert!(cant_revoke_others_claim.is_err());
        assert_eq!(
            cant_revoke_others_claim.unwrap_err(),
            "this claim is owned by someone else"
        );
        assert!(poe.revoke_claim(alice, content).is_ok());
        assert!(poe.get_claim(&content).is_none());
        assert_eq!(
            poe.revoke_claim(bob, content).unwrap_err(),
            "claim not found"
        );
    }
}
