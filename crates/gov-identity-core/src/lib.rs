pub mod did;
pub mod credential;
pub mod error;

pub use did::{GovDid, DidDocument};
pub use credential::{VerifiableCredential, DataGrantCredential, CredentialIssuer};
pub use error::IdentityError;
