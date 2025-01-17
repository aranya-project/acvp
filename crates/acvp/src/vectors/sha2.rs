//! SHA-2 test vectors.

#![cfg(feature = "sha2")]
#![cfg_attr(docsrs, doc(cfg(feature = "sha2")))]

use serde::{Deserialize, Serialize};

/// A group of test vectors.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestGroup {
    /// Identifies the group.
    pub tg_id: usize,
    /// The SHA-2 function name.
    pub function: String,
    /// The size in bits of the digest.
    pub digest_size: String,
    /// MCT version.
    pub mct_version: MctVersion,
    /// The test vectors.
    #[serde(flatten)]
    pub tests: Tests,
}

/// The version of a monte carlo test.
///
/// See [draft-ietf-acvp-sub-sha-01]
///
/// [draft-ietf-acvp-sub-sha-01]: https://pages.nist.gov/ACVP/draft-celi-acvp-sha.html#name-monte-carlo-tests-for-sha-1
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MctVersion {
    /// Standard.
    Standard,
    /// Alternate.
    Alternate,
}

/// SHA-2 tests.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[serde(tag = "testType", content = "tests")]
pub enum Tests {
    /// Algorithm functional tests.
    Aft(Vec<Aft>),
    /// Monte carlo tests.
    Mct(Vec<Mct>),
    /// Large data tests.
    Ldt(Vec<Ldt>),
}

/// A SHA-2 AFT.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aft {
    /// Identifies the test case.
    pub tc_id: usize,
    /// The message to compute the hash of.
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,
    /// The length in bits of `msg`.
    pub len: usize,
    /// The expected digest.
    #[serde(with = "hex::serde")]
    pub md: Vec<u8>,
}

/// A SHA-2 AFT.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mct {
    /// Identifies the test case.
    pub tc_id: usize,
    /// The initial message (seed).
    #[serde(with = "hex::serde")]
    pub msg: Vec<u8>,
    /// The length in bits of `msg`.
    pub len: usize,
    /// Per-iteration results.
    pub results_array: Vec<MctResult>,
}

/// The result of a monte carlo test iteration.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MctResult {
    /// The expected digest.
    #[serde(with = "hex::serde")]
    pub md: Vec<u8>,
    /// The size in bits of `md`.
    pub out_len: usize,
}

/// A SHA-2 LDT.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ldt {
    /// Identifies the test case.
    pub tc_id: usize,
    /// Always zero.
    pub len: usize,
    /// The expected digest.
    #[serde(with = "hex::serde")]
    pub md: Vec<u8>,
    /// Large message parameters.
    pub large_msg: LargeMsg,
}

/// For large message tests.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LargeMsg {
    /// Data that is expanded into the full message.
    #[serde(with = "hex::serde")]
    pub content: Vec<u8>,
    /// The length of `content` in bits.
    pub content_length: usize,
    /// The total number of bits written to the hash.
    pub full_length: usize,
    /// The technique for expanding `content` into the full
    /// message.
    pub expansion_technique: ExpansionTechnique,
}

/// The technique for expanding `content` into the full message.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpansionTechnique {
    /// Appends the number of bits specified in `content_length`
    /// until a string of exactly `full_length` has been reached.
    Repeating,
}
