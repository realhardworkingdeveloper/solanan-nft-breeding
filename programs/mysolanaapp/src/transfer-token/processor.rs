// use std::convert::TryInto;
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::Pubkey};
use spl_token::instruction::transfer_checked;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let from = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let to = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::InvalidSeeds);
    }

    // let amount = unpack_amount(instruction_data)?;

    let instruction = transfer_checked(
        &token_program.key, 
        &from.key, 
        &token_account.key, 
        &to.key, 
        &owner.key, 
        &[&owner.key], 
        1000000000, 
        9
    );

    msg!("Calling the token program to transfer tokens...");

    invoke(
        &instruction.unwrap(), 
        &[
            token_program.clone(),
            from.clone(),
            token_account.clone(),
            to.clone(),
            owner.clone(),
        ]
    )?;
    
    Ok(())
}
