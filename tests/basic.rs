use blawgd_solana::{
    instructions::{
        instantiate::InstantiateArgs, update_profile::UpdateProfileArgs, BlawgdInstruction,
    },
    state::{account::Profile, account::UserAccount, ProgramState},
};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction, system_program,
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
    let (client, mint, _) = pt.start().await;

    let user = Keypair::new();
    request_airdrop(client.clone(), &mint, &user, LAMPORTS_PER_SOL * 10).await?;

    instantiate_program(client.clone(), program_id, &user).await?;
    println!("instantiated smart contract");

    let profile = Profile {
        name: "Johnny".to_string(),
        image: "example image".to_string(),
        bio: "eating sugar".to_string(),
    };
    update_profile(client.clone(), program_id, &user, profile).await?;
    println!("update user profile");

    Ok(())
}

async fn update_profile(
    mut client: BanksClient,
    program_id: Pubkey,
    user: &Keypair,
    profile: Profile,
) -> Result<(), Box<dyn std::error::Error>> {
    let (account_addr, _) = Pubkey::find_program_address(
        &[UserAccount::seed(user.pubkey()).as_slice()],
        &program_id,
    );

    let update_profile_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(account_addr, false),
            AccountMeta::new(user.pubkey(), true),
            AccountMeta::new(system_program::id(), false),
        ],
        data: BlawgdInstruction::UpdateProfile(UpdateProfileArgs {
            profile: profile.clone(),
        })
        .try_to_vec()?,
    };

    create_and_send_tx(
        client.clone(),
        vec![update_profile_instruction],
        vec![user],
        Some(&user.pubkey()),
    )
    .await?;

    let account_acc = client
        .get_account(account_addr)
        .await?
        .ok_or("could not find program state account")?;

    let updated_profile = UserAccount::deserialize(&mut account_acc.data.as_slice())?.profile;
    assert_eq!(updated_profile.name, profile.name);
    assert_eq!(updated_profile.image, profile.image);
    assert_eq!(updated_profile.bio, profile.bio);

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

async fn request_airdrop(
    client: BanksClient,
    mint: &Keypair,
    user: &Keypair,
    lamports: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = system_instruction::transfer(&mint.pubkey(), &user.pubkey(), lamports);
    create_and_send_tx(client, vec![ix], vec![mint], Some(&mint.pubkey())).await?;
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
