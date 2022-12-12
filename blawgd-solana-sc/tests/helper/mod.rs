use blawgd_solana_sc::{
    instructions::{
        create_post::CreatePostArgs, instantiate::InstantiateArgs,
        update_following_list::UpdateFollowingListArgs, update_profile::UpdateProfileArgs,
        BlawgdInstruction,
    },
    state::{
        account::Profile,
        account::{AccountPost, UserAccount, UserFollowingList},
        post::{Post, PostUserInteractionStatus},
        program_state::ProgramState,
    },
};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    system_instruction, system_program,
};
use solana_program_test::BanksClient;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub async fn update_following_list(
    mut client: BanksClient,
    program_id: Pubkey,
    user: &Keypair,
    args: UpdateFollowingListArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let (signer_user_acc_addr, _) =
        Pubkey::find_program_address(&[UserAccount::seed(user.pubkey()).as_slice()], &program_id);
    let (to_follow_user_acc_addr, _) =
        Pubkey::find_program_address(&[UserAccount::seed(args.user).as_slice()], &program_id);
    let (following_list_addr, _) = Pubkey::find_program_address(
        &[UserFollowingList::seed(user.pubkey()).as_slice()],
        &program_id,
    );

    let update_following_list_instr = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(following_list_addr, false),
            AccountMeta::new(signer_user_acc_addr, false),
            AccountMeta::new(to_follow_user_acc_addr, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(user.pubkey(), true),
        ],
        data: BlawgdInstruction::UpdateFollowingList(args.clone()).try_to_vec()?,
    };

    if let Some(original_following_list_acc) = client.get_account(following_list_addr).await? {
        let original_following_list =
            UserFollowingList::deserialize(&mut original_following_list_acc.data.as_slice())?;
        // println!("original-following-list - {:?}", original_following_list);
        assert_eq!(
            original_following_list.list.contains(&args.user),
            !args.add_operation
        );
    }

    create_and_send_tx(
        client.clone(),
        vec![update_following_list_instr],
        vec![user],
        Some(&user.pubkey()),
    )
    .await?;

    let modified_following_list_acc = client
        .get_account(following_list_addr)
        .await?
        .ok_or("could not find following list state account")?;

    let modified_following_list =
        UserFollowingList::deserialize(&mut modified_following_list_acc.data.as_slice())?;
    // println!("modified-following-list - {:?}", modified_following_list);
    assert_eq!(
        modified_following_list.list.contains(&args.user),
        args.add_operation
    );

    Ok(())
}

// TODO check if post and account post count got updated
pub async fn like_post(
    mut client: BanksClient,
    program_id: Pubkey,
    user: &Keypair,
    post_addr: Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let post_acc = client
        .get_account(post_addr)
        .await?
        .ok_or("could not find post state account")?;

    let post = Post::deserialize(&mut post_acc.data.as_slice())?;
    let original_like_count = post.like_count;

    let post_user_interaction_status_addr = Pubkey::find_program_address(
        &[PostUserInteractionStatus::seed(post_addr, user.pubkey()).as_slice()],
        &program_id,
    )
    .0;

    let like_post_instr = Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(post_addr, false),
            AccountMeta::new(post_user_interaction_status_addr, false),
            AccountMeta::new(system_program::id(), false),
            AccountMeta::new(user.pubkey(), true),
        ],
        data: BlawgdInstruction::LikePost(
            blawgd_solana_sc::instructions::like_post::LikePostArgs {},
        )
        .try_to_vec()?,
    };

    create_and_send_tx(
        client.clone(),
        vec![like_post_instr],
        vec![user],
        Some(&user.pubkey()),
    )
    .await?;

    let post_acc = client
        .get_account(post_addr)
        .await?
        .ok_or("could not find program state account")?;

    let post = Post::deserialize(&mut post_acc.data.as_slice())?;
    assert_eq!(post.like_count, original_like_count + 1);

    let post_user_interaction_status_acc = client
        .get_account(post_user_interaction_status_addr)
        .await?
        .ok_or("could not find post user interaction status state account")?;

    let post_user_interaction_status = PostUserInteractionStatus::deserialize(
        &mut post_user_interaction_status_acc.data.as_slice(),
    )?;
    assert_eq!(post_user_interaction_status.liked, true);

    Ok(())
}

// TODO check if post and account post count got updated
pub async fn create_post(
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

    let mut create_post_instr = Instruction {
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

    if let Some(parent_post_addr) = args.parent_post {
        create_post_instr
            .accounts
            .push(AccountMeta::new(parent_post_addr, false));
        let parent_post_user_interaction_status_acc = Pubkey::find_program_address(
            &[PostUserInteractionStatus::seed(parent_post_addr, user.pubkey()).as_slice()],
            &program_id,
        )
        .0;
        create_post_instr.accounts.push(AccountMeta::new(
            parent_post_user_interaction_status_acc,
            false,
        ));
    }

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

pub async fn update_profile(
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

pub async fn instantiate_program(
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

pub async fn request_airdrop(
    client: BanksClient,
    mint: &Keypair,
    user: &Keypair,
    lamports: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = system_instruction::transfer(&mint.pubkey(), &user.pubkey(), lamports);
    create_and_send_tx(client, vec![ix], vec![mint], Some(&mint.pubkey())).await?;
    Ok(())
}

pub async fn create_and_send_tx(
    mut client: BanksClient,
    instructions: Vec<Instruction>,
    signers: Vec<&dyn Signer>,
    payer: Option<&Pubkey>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::new(instructions.as_slice(), payer);
    let tx = Transaction::new(&signers, msg, client.get_latest_blockhash().await?);
    Ok(client.process_transaction(tx).await?)
}
