pub mod helper;

use blawgd_solana::{
    instructions::{create_post::CreatePostArgs, update_following_list::UpdateFollowingListArgs},
    state::{account::Profile, post::Post},
};
use helper::*;
use solana_program::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use solana_program_test::tokio;
use solana_sdk::{signature::Keypair, signer::Signer};

#[tokio::test]
async fn basic_functionality() -> Result<(), Box<dyn std::error::Error>> {
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

    let (post_addr, _) = Pubkey::find_program_address(&[Post::seed(1).as_slice()], &program_id);
    create_post(
        client.clone(),
        program_id,
        &user,
        CreatePostArgs {
            parent_post: Some(post_addr),
            is_repost: false,
            content: "commenting on 'Hello World!'".to_string(),
        },
    )
    .await?;
    println!("commented on post");

    create_post(
        client.clone(),
        program_id,
        &user,
        CreatePostArgs {
            parent_post: Some(post_addr),
            is_repost: true,
            content: "reposting 'Hello World!'".to_string(),
        },
    )
    .await?;
    println!("reposted post");

    // let (repost_addr, _) = Pubkey::find_program_address(&[Post::seed(3).as_slice()], &program_id);
    // // TODO fix this bug
    // create_post(
    //     client.clone(),
    //     program_id,
    //     &user,
    //     CreatePostArgs {
    //         parent_post: Some(repost_addr),
    //         is_repost: false,
    //         content: "commenting on respost".to_string(),
    //     },
    // )
    // .await?;
    // println!("commented on repost - this should not be allowed - need to fix this");

    like_post(client.clone(), program_id, &user, post_addr).await?;
    println!("liked post");

    // like_post(client.clone(), program_id, &user, post_addr).await?;
    // println!("liked post again with same user - this should not be allowed - need to fix this");

    let user2 = Keypair::new();
    request_airdrop(client.clone(), &mint, &user2, LAMPORTS_PER_SOL * 10).await?;

    let profile = Profile {
        name: "Bob".to_string(),
        image: "example image".to_string(),
        bio: "The builder".to_string(),
    };
    update_profile(client.clone(), program_id, &user2, profile).await?;
    println!("created second users profile");

    update_following_list(
        client.clone(),
        program_id,
        &user,
        UpdateFollowingListArgs {
            user: user2.pubkey(),
            add_operation: true,
        },
    )
    .await?;
    println!("first user followed second user");

    update_following_list(
        client.clone(),
        program_id,
        &user,
        UpdateFollowingListArgs {
            user: user2.pubkey(),
            add_operation: false,
        },
    )
    .await?;
    println!("first user unfollowed second user");

    // update_following_list(
    //     client.clone(),
    //     program_id,
    //     &user,
    //     UpdateFollowingListArgs {
    //         user: user.pubkey(),
    //         add_operation: true,
    //     },
    // )
    // .await?;
    // println!("this should fail should not be able to follow or unfollow yourself");

    Ok(())
}
