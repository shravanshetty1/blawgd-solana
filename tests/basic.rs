
use blawgd_solana::{
    instructions::{
        create_post::CreatePostArgs, instantiate::InstantiateArgs,
        update_profile::UpdateProfileArgs, BlawgdInstruction,
    },
    state::{
        account::Profile,
        account::{AccountPost, UserAccount},
        post::Post,
        program_state::ProgramState,
    },
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

    if instantiate_program(client.clone(), program_id, &user)
        .await
        .is_err()
    {
        println!("success - failed to instantiate smart contract twice");
    } else {
        println!("failed - instantiated smart contract twice");
    }

    let profile = Profile {
        name: "Johnny".to_string(),
        image: "example image".to_string(),
        bio: "eating sugar".to_string(),
    };
    update_profile(client.clone(), program_id, &user, profile).await?;
    println!("created user profile");

    let profile = Profile {
        name: "Johnny Boy".to_string(),
        image: "example image".to_string(),
        bio: "eating sugar".to_string(),
    };
    update_profile(client.clone(), program_id, &user, profile).await?;
    println!("updated user profile");

    create_post(
        client.clone(),
        program_id,
        &user,
        CreatePostArgs {
            parent_post: None,
            is_repost: false,
            content: "Hello World!".to_string(),
        },
    )
    .await?;
    println!("created post");

    Ok(())
}

async fn create_post(
    mut client: BanksClient,
    program_id: Pubkey,
    user: &Keypair,
    args: CreatePostArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let (user_acc_addr, _) =
        Pubkey::find_program_address(&[UserAccount::seed(user.pubkey()).as_slice()], &program_id);
    let user_acc = client
        .get_account(user_acc_addr)
        .await?
        .ok_or("could not find user account")?;
    let user_acc = UserAccount::deserialize(&mut user_acc.data.as_slice())?;

    let (user_acc_post_addr, _) = Pubkey::find_program_address(
        &[AccountPost::seed(user.pubkey(), user_acc.post_count + 1).as_slice()],
        &program_id,
    );

    let (program_state_addr, _) =
        Pubkey::find_program_address(&[ProgramState::seed().as_slice()], &program_id);
    let program_state_acc = client
        .get_account(program_state_addr)
        .await?
        .ok_or("could not find program state account")?;
    let program_state = ProgramState::deserialize(&mut program_state_acc.data.as_slice())?;

    let (post_addr, _) = Pubkey::find_program_address(
        &[Post::seed(program_state.post_count + 1).as_slice()],
        &program_id,
    );

    let create_post_instr = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state_addr, false),
            AccountMeta::new(user_acc_addr, false),
            AccountMeta::new(post_addr, false),
            AccountMeta::new(user_acc_post_addr, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(user.pubkey(), true),
        ],
        data: BlawgdInstruction::CreatePost(args.clone()).try_to_vec()?,
    };

    create_and_send_tx(
        client.clone(),
        vec![create_post_instr],
        vec![user],
        Some(&user.pubkey()),
    )
    .await?;

    let post_acc = client
        .get_account(post_addr)
        .await?
        .ok_or("could not find program state account")?;

    let post = Post::deserialize(&mut post_acc.data.as_slice())?;
    assert_eq!(post.parent_post, args.parent_post);
    assert_eq!(post.creator, user.pubkey());
    assert_eq!(post.content, args.content);
    assert_eq!(post.is_repost, args.is_repost);
    assert_eq!(post.like_count, 0);
    assert_eq!(post.comment_count, 0);
    assert_eq!(post.repost_count, 0);

    Ok(())
}

async fn update_profile(
    mut client: BanksClient,
    program_id: Pubkey,
    user: &Keypair,
    profile: Profile,
) -> Result<(), Box<dyn std::error::Error>> {
    let (account_addr, _) =
        Pubkey::find_program_address(&[UserAccount::seed(user.pubkey()).as_slice()], &program_id);

    let update_profile_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(account_addr, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(user.pubkey(), true),
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
    let (program_state, _) =
        Pubkey::find_program_address(&[ProgramState::seed().as_slice()], &program_id);

    let instantiate_instruction = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(program_state, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(instantiater.pubkey(), true),
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
