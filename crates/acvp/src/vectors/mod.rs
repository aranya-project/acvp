//! ACVP server internal projections.
//!
//! It's the combination of a prompt and the response.

use serde::{Deserialize, Serialize};

pub mod sha2;
pub mod sha3;

/// A prompt sent from the ACVP server.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vectors<G> {
    /// Identifies the test vectors.
    pub vs_id: usize,
    /// The algorithm being tested.
    pub algorithm: String,
    /// The protocol revision.
    pub revision: String,
    /// Are these sample vectors?
    pub is_sample: bool,
    /// Groups of test vectors.
    pub test_groups: Vec<G>,
}
