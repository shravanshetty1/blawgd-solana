use blawgd_solana::{
    instructions::{instantiate::InstantiateArgs, BlawgdInstruction},
    state::ProgramState,
};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    system_program,
};
use solana_program_test::{tokio, BanksClient};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let program_id = blawgd_solana::id();
    let pt = solana_program_test::ProgramTest::new(
        "blawgd_solana",
        program_id,
        solana_program_test::processor!(blawgd_solana::entrypoint::process_instruction),
    );
    let (client, payer, _) = pt.start().await;

    instantiate_program(client.clone(), program_id, &payer).await?;
    println!("instantiated smart contract");

    Ok(())
}

async fn instantiate_program(
    mut client: BanksClient,
    program_id: Pubkey,
    instantiater: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let (program_state, _) = Pubkey::find_program_address(
        &[blawgd_solana::state::ProgramState::seed().as_slice()],
        &program_id,
    );

    let instantiate_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state, false),
            AccountMeta::new(instantiater.pubkey(), true),
            AccountMeta::new(system_program::id(), false),
        ],
        data: BlawgdInstruction::Instantiate(InstantiateArgs {}).try_to_vec()?,
    };

    create_and_send_tx(
        client.clone(),
        vec![instantiate_instruction],
        vec![instantiater],
        Some(&instantiater.pubkey()),
    )
    .await?;

    let program_state_acc = client
        .get_account(program_state)
        .await?
        .ok_or("could not find program state account")?;

    let program_state = ProgramState::deserialize(&mut program_state_acc.data.as_slice())?;
    assert_eq!(program_state.post_count, 0);

    Ok(())
}

async fn create_and_send_tx(
    mut client: BanksClient,
    instructions: Vec<Instruction>,
    signers: Vec<&dyn Signer>,
    payer: Option<&Pubkey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::new(instructions.as_slice(), payer);
    let tx = Transaction::new(&signers, msg, client.get_latest_blockhash().await?);
    Ok(client.process_transaction(tx).await?)
}
