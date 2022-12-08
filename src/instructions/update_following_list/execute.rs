use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint::ProgramResult;

use crate::{
    state::account::{UserAccount, UserFollowingList},
    util::create_pda,
};

use super::UpdateFollowingList;

impl<'a, 'b> UpdateFollowingList<'a, 'b> {
    pub fn execute_instruction(&self) -> ProgramResult {
        let mut following_list = if self.accounts.following_list.data.borrow().len() > 0 {
            UserFollowingList::deserialize(&mut &**self.accounts.following_list.data.borrow())?
        } else {
            create_pda(
                &self.program_id,
                UserFollowingList::space()?,
                self.accounts.signer,
                self.accounts.following_list,
                self.accounts.system_program,
                UserFollowingList::seed(*self.accounts.signer.key).as_slice(),
            )?;
            UserFollowingList::default()
        };
        let mut signer_user_acc =
            UserAccount::deserialize(&mut &**self.accounts.signer_user_account.data.borrow())?;
        let mut to_follow_user_acc =
            UserAccount::deserialize(&mut &**self.accounts.to_follow_user_account.data.borrow())?;

        if self.args.add_operation {
            if following_list.list.insert(self.args.user) {
                signer_user_acc.following_count += 1;
                to_follow_user_acc.follower_count += 1;
            }
        } else if following_list.list.remove(&self.args.user) {
            signer_user_acc.following_count -= 1;
            to_follow_user_acc.follower_count -= 1;
        }

        following_list.serialize(&mut &mut self.accounts.following_list.data.borrow_mut()[..])?;
        signer_user_acc
            .serialize(&mut &mut self.accounts.signer_user_account.data.borrow_mut()[..])?;
        to_follow_user_acc
            .serialize(&mut &mut self.accounts.to_follow_user_account.data.borrow_mut()[..])?;

        Ok(())
    }
}
