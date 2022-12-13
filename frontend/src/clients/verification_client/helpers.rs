use anyhow::Result;

pub fn get_exist_proof<'a>(
    proof: &'a ics23::CommitmentProof,
    key: &[u8],
) -> Option<&'a ics23::ExistenceProof> {
    match &proof.proof {
        Some(ics23::commitment_proof::Proof::Exist(ex)) => Some(ex),
        Some(ics23::commitment_proof::Proof::Batch(batch)) => {
            for entry in &batch.entries {
                if let Some(ics23::batch_entry::Proof::Exist(ex)) = &entry.proof {
                    if ex.key == key {
                        return Some(ex);
                    }
                }
            }
            None
        }
        _ => None,
    }
}

pub fn get_default_path(key: &[u8]) -> Vec<&[u8]> {
    vec![key, "blawgd".as_bytes()]
}

pub fn get_cosmos_specs() -> Vec<ics23::ProofSpec> {
    vec![ics23::iavl_spec(), ics23::tendermint_spec()]
}

pub fn convert_tm_to_ics_merkle_proof(
    tm_proof: tendermint_proto::crypto::ProofOps,
) -> Result<Vec<ics23::CommitmentProof>> {
    let mut proofs = vec![];

    for op in &tm_proof.ops {
        let parsed = prost::Message::decode(op.data.as_slice())?;
        proofs.push(parsed);
    }

    Ok(proofs)
}
