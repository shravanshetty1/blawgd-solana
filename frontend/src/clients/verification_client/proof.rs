use crate::clients::verification_client::helpers::{get_cosmos_specs, get_default_path};
use anyhow::anyhow;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;

pub fn verify_membership(
    proofs: Vec<ics23::CommitmentProof>,
    root: &[u8],
    key: &[u8],
    value: &[u8],
) -> Result<()> {
    let specs = get_cosmos_specs();
    let path = get_default_path(key);
    validate_membership_args(specs.clone(), proofs.clone(), root, path.clone())?;
    ensure!(
        !value.is_empty() && value.len() != 0,
        "value cannot be empty"
    );

    let mut value = value.to_vec();
    for i in 0..proofs.len() {
        let key = path[i];
        let subroot = ics23::calculate_existence_root(
            super::helpers::get_exist_proof(&proofs[i], key).ok_or(anyhow!(
                "could not convert commitment proof to existence proof"
            ))?,
        )?;
        if !ics23::verify_membership(&proofs[i], &specs[i], &subroot, key, value.as_slice()) {
            bail!("failed to verify proof at index {}", i)
        }
        value = subroot
    }

    let given_root = hex::encode_upper(root);
    let calculated_root = hex::encode_upper(value);

    ensure!(
        given_root == calculated_root,
        format!(
            "given root did not much calculated root - given {} calculated {}",
            given_root, calculated_root
        )
    );

    Ok(())
}

pub fn verify_non_membership(
    proofs: Vec<ics23::CommitmentProof>,
    root: &[u8],
    key: &[u8],
) -> Result<()> {
    let specs = get_cosmos_specs();
    let path = get_default_path(key);
    validate_membership_args(specs.clone(), proofs.clone(), root, path.clone())?;

    // TODO

    Ok(())
}

pub fn validate_membership_args(
    specs: Vec<ics23::ProofSpec>,
    proofs: Vec<ics23::CommitmentProof>,
    root: &[u8],
    path: Vec<&[u8]>,
) -> Result<()> {
    ensure!(!proofs.is_empty(), "proof cannot be empty");
    ensure!(!root.is_empty() && root.len() != 0, "root cannot be empty");
    ensure!(
        proofs.len() == specs.len(),
        "length of specs not equal to length of proof"
    );
    ensure!(
        path.len() == specs.len(),
        "path length not the same as proof"
    );

    Ok(())
}
