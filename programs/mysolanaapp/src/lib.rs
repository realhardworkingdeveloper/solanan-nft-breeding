pub mod utils;

use {
    crate::utils::{
        assert_initialized, assert_is_ata, assert_keys_equal, assert_owned_by,
        assert_valid_go_live, spl_token_burn, spl_token_transfer, TokenBurnParams,
        TokenTransferParams,
    },
    anchor_lang::{
        prelude::*,
        solana_program::{
            log::sol_log_compute_units,
            program::{invoke, invoke_signed},
            serialize_utils::{read_pubkey, read_u16},
            system_instruction, sysvar,
        },
        AnchorDeserialize, AnchorSerialize, Discriminator, Key,
    },
    anchor_spl::token::Token,
    arrayref::array_ref,
    mpl_token_metadata::{
        instruction::{create_master_edition, create_metadata_accounts, update_metadata_accounts},
        state::{
            MAX_CREATOR_LEN, MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
        },
    },
    spl_token::state::Mint,
    std::{cell::RefMut, ops::Deref, str::FromStr},
};
anchor_lang::declare_id!("2b9Q4a8DSNfypePwm1FvE9AgLWJDbHZbVSuV6DqGooaS");

const EXPIRE_OFFSET: i64 = 10 * 60;
const PREFIX: &str = "breeding_machine";
#[program]
pub mod nft_breeding_machine_v2 {

    use super::*;

    pub fn mint_nft<'info>(
        ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
        creator_bump: u8,
    ) -> ProgramResult {
        let breeding_machine = &mut ctx.accounts.breeding_machine;
        let breeding_machine_creator = &ctx.accounts.breeding_machine_creator;
        let clock = &ctx.accounts.clock;
        // Note this is the wallet of the Breeding machine
        let wallet = &ctx.accounts.wallet;
        let payer = &ctx.accounts.payer;
        let token_program = &ctx.accounts.token_program;
        let recent_blockhashes = &ctx.accounts.recent_blockhashes;
        let instruction_sysvar_account = &ctx.accounts.instruction_sysvar_account;
        let mut price = breeding_machine.data.price;
        if let Some(es) = &breeding_machine.data.end_settings {
            match es.end_setting_type {
                EndSettingType::Date => {
                    if clock.unix_timestamp > es.number as i64 {
                        if *ctx.accounts.payer.key != breeding_machine.authority {
                            return Err(ErrorCode::BreedingMachineNotLive.into());
                        }
                    }
                }
                EndSettingType::Amount => {
                    if breeding_machine.items_redeemed >= es.number {
                        return Err(ErrorCode::BreedingMachineNotLive.into());
                    }
                }
            }
        }

        let mut remaining_accounts_counter: usize = 0;
        if let Some(gatekeeper) = &breeding_machine.data.gatekeeper {
            if ctx.remaining_accounts.len() <= remaining_accounts_counter {
                return Err(ErrorCode::GatewayTokenMissing.into());
            }
            let gateway_token_info = &ctx.remaining_accounts[remaining_accounts_counter];
            let gateway_token = ::solana_gateway::borsh::try_from_slice_incomplete::<
                ::solana_gateway::state::GatewayToken,
            >(*gateway_token_info.data.borrow())?;
            // stores the expire_time before the verification, since the verification
            // will update the expire_time of the token and we won't be able to
            // calculate the creation time
            let expire_time = gateway_token
                .expire_time
                .ok_or(ErrorCode::GatewayTokenExpireTimeInvalid)?
                as i64;
            remaining_accounts_counter += 1;
            if gatekeeper.expire_on_use {
                if ctx.remaining_accounts.len() <= remaining_accounts_counter {
                    return Err(ErrorCode::GatewayAppMissing.into());
                }
                let gateway_app = &ctx.remaining_accounts[remaining_accounts_counter];
                remaining_accounts_counter += 1;
                if ctx.remaining_accounts.len() <= remaining_accounts_counter {
                    return Err(ErrorCode::NetworkExpireFeatureMissing.into());
                }
                let network_expire_feature = &ctx.remaining_accounts[remaining_accounts_counter];
                remaining_accounts_counter += 1;
                ::solana_gateway::Gateway::verify_and_expire_token(
                    gateway_app.clone(),
                    gateway_token_info.clone(),
                    payer.deref().clone(),
                    &gatekeeper.gatekeeper_network,
                    network_expire_feature.clone(),
                )?;
            } else {
                ::solana_gateway::Gateway::verify_gateway_token_account_info(
                    gateway_token_info,
                    &payer.key(),
                    &gatekeeper.gatekeeper_network,
                    None,
                )?;
            }
            // verifies that the gatway token was not created before the breeding
            // machine go_live_date (avoids pre-solving the captcha)
            match breeding_machine.data.go_live_date {
                Some(val) => {
                    msg!(
                        "Comparing token expire time {} and go_live_date {}",
                        expire_time,
                        val
                    );
                    if (expire_time - EXPIRE_OFFSET) < val {
                        if let Some(ws) = &breeding_machine.data.whitelist_mint_settings {
                            // when dealing with whitelist, the expire_time can be
                            // before the go_live_date only if presale enabled
                            if !ws.presale {
                                msg!(
                                    "Invalid gateway token: calculated creation time {} and go_live_date {}",
                                    expire_time - EXPIRE_OFFSET,
                                    val);
                                return Err(ErrorCode::GatewayTokenExpireTimeInvalid.into());
                            }
                        } else {
                            msg!(
                                "Invalid gateway token: calculated creation time {} and go_live_date {}",
                                expire_time - EXPIRE_OFFSET,
                                val);
                            return Err(ErrorCode::GatewayTokenExpireTimeInvalid.into());
                        }
                    }
                }
                None => {}
            }
        }

        if let Some(ws) = &breeding_machine.data.whitelist_mint_settings {
            let whitelist_token_account = &ctx.remaining_accounts[remaining_accounts_counter];
            remaining_accounts_counter += 1;
            // If the user has not actually made this account,
            // this explodes and we just check normal dates.
            // If they have, we check amount, if it's > 0 we let them use the logic
            // if 0, check normal dates.
            match assert_is_ata(whitelist_token_account, &payer.key(), &ws.mint) {
                Ok(wta) => {
                    if wta.amount > 0 {
                        if ws.mode == WhitelistMintMode::BurnEveryTime {
                            let whitelist_token_mint =
                                &ctx.remaining_accounts[remaining_accounts_counter];
                            remaining_accounts_counter += 1;

                            let whitelist_burn_authority =
                                &ctx.remaining_accounts[remaining_accounts_counter];
                            remaining_accounts_counter += 1;

                            assert_keys_equal(*whitelist_token_mint.key, ws.mint)?;

                            spl_token_burn(TokenBurnParams {
                                mint: whitelist_token_mint.clone(),
                                source: whitelist_token_account.clone(),
                                amount: 1,
                                authority: whitelist_burn_authority.clone(),
                                authority_signer_seeds: None,
                                token_program: token_program.to_account_info(),
                            })?;
                        }

                        match breeding_machine.data.go_live_date {
                            None => {
                                if *ctx.accounts.payer.key != breeding_machine.authority && !ws.presale
                                {
                                    return Err(ErrorCode::BreedingMachineNotLive.into());
                                }
                            }
                            Some(val) => {
                                if clock.unix_timestamp < val
                                    && *ctx.accounts.payer.key != breeding_machine.authority
                                    && !ws.presale
                                {
                                    return Err(ErrorCode::BreedingMachineNotLive.into());
                                }
                            }
                        }

                        if let Some(dp) = ws.discount_price {
                            price = dp;
                        }
                    } else {
                        if wta.amount == 0 && ws.discount_price.is_none() && !ws.presale {
                            // A non-presale whitelist with no discount price is a forced whitelist
                            // If a pre-sale has no discount, its no issue, because the "discount"
                            // is minting first - a presale whitelist always has an open post sale.
                            return Err(ErrorCode::NoWhitelistToken.into());
                        }
                        assert_valid_go_live(payer, clock, breeding_machine)?;
                    }
                }
                Err(_) => {
                    if ws.discount_price.is_none() && !ws.presale {
                        // A non-presale whitelist with no discount price is a forced whitelist
                        // If a pre-sale has no discount, its no issue, because the "discount"
                        // is minting first - a presale whitelist always has an open post sale.
                        return Err(ErrorCode::NoWhitelistToken.into());
                    }
                    assert_valid_go_live(payer, clock, breeding_machine)?
                }
            }
        } else {
            // no whitelist means normal datecheck
            assert_valid_go_live(payer, clock, breeding_machine)?;
        }

        if breeding_machine.items_redeemed >= breeding_machine.data.items_available {
            return Err(ErrorCode::BreedingMachineEmpty.into());
        }

        if let Some(mint) = breeding_machine.token_mint {
            let token_account_info = &ctx.remaining_accounts[remaining_accounts_counter];
            remaining_accounts_counter += 1;
            let transfer_authority_info = &ctx.remaining_accounts[remaining_accounts_counter];
            let token_account = assert_is_ata(token_account_info, &payer.key(), &mint)?;

            if token_account.amount < price {
                return Err(ErrorCode::NotEnoughTokens.into());
            }

            spl_token_transfer(TokenTransferParams {
                source: token_account_info.clone(),
                destination: wallet.to_account_info(),
                authority: transfer_authority_info.clone(),
                authority_signer_seeds: &[],
                token_program: token_program.to_account_info(),
                amount: price,
            })?;
        } else {
            if ctx.accounts.payer.lamports() < price {
                return Err(ErrorCode::NotEnoughSOL.into());
            }

            invoke(
                &system_instruction::transfer(&ctx.accounts.payer.key, wallet.key, price),
                &[
                    ctx.accounts.payer.to_account_info(),
                    wallet.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }
        let data = recent_blockhashes.data.borrow();
        let most_recent = array_ref![data, 8, 8];

        let index = u64::from_le_bytes(*most_recent);
        let modded: usize = index
            .checked_rem(breeding_machine.data.items_available)
            .ok_or(ErrorCode::NumericalOverflowError)? as usize;

        let config_line = get_config_line(&breeding_machine, modded, breeding_machine.items_redeemed)?;

        breeding_machine.items_redeemed = breeding_machine
            .items_redeemed
            .checked_add(1)
            .ok_or(ErrorCode::NumericalOverflowError)?;

        let cm_key = breeding_machine.key();
        let authority_seeds = [PREFIX.as_bytes(), cm_key.as_ref(), &[creator_bump]];

        let mut creators: Vec<mpl_token_metadata::state::Creator> =
            vec![mpl_token_metadata::state::Creator {
                address: breeding_machine_creator.key(),
                verified: true,
                share: 0,
            }];

        for c in &breeding_machine.data.creators {
            creators.push(mpl_token_metadata::state::Creator {
                address: c.address,
                verified: false,
                share: c.share,
            });
        }

        let metadata_infos = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            breeding_machine_creator.to_account_info(),
        ];

        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            breeding_machine_creator.to_account_info(),
        ];
        msg!("Before metadata");
        sol_log_compute_units();

        invoke_signed(
            &create_metadata_accounts(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.metadata.key,
                *ctx.accounts.mint.key,
                *ctx.accounts.mint_authority.key,
                *ctx.accounts.payer.key,
                breeding_machine_creator.key(),
                config_line.name,
                breeding_machine.data.symbol.clone(),
                config_line.uri,
                Some(creators),
                breeding_machine.data.seller_fee_basis_points,
                true,
                breeding_machine.data.is_mutable,
            ),
            metadata_infos.as_slice(),
            &[&authority_seeds],
        )?;

        msg!("Before master");
        sol_log_compute_units();
        invoke_signed(
            &create_master_edition(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.master_edition.key,
                *ctx.accounts.mint.key,
                breeding_machine_creator.key(),
                *ctx.accounts.mint_authority.key,
                *ctx.accounts.metadata.key,
                *ctx.accounts.payer.key,
                Some(breeding_machine.data.max_supply),
            ),
            master_edition_infos.as_slice(),
            &[&authority_seeds],
        )?;

        let mut new_update_authority = Some(breeding_machine.authority);

        if !breeding_machine.data.retain_authority {
            new_update_authority = Some(ctx.accounts.update_authority.key());
        }

        msg!("Before update");
        sol_log_compute_units();
        invoke_signed(
            &update_metadata_accounts(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.metadata.key,
                breeding_machine_creator.key(),
                new_update_authority,
                None,
                Some(true),
            ),
            &[
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                breeding_machine_creator.to_account_info(),
            ],
            &[&authority_seeds],
        )?;

        msg!("Before instr check");
        sol_log_compute_units();

        let instruction_sysvar_account_info = instruction_sysvar_account.to_account_info();

        let instruction_sysvar = instruction_sysvar_account_info.data.borrow();

        let mut idx = 0;
        let num_instructions = read_u16(&mut idx, &instruction_sysvar)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let associated_token =
            Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();

        for index in 0..num_instructions {
            let mut current = 2 + (index * 2) as usize;
            let start = read_u16(&mut current, &instruction_sysvar).unwrap();

            current = start as usize;
            let num_accounts = read_u16(&mut current, &instruction_sysvar).unwrap();
            current += (num_accounts as usize) * (1 + 32);
            let program_id = read_pubkey(&mut current, &instruction_sysvar).unwrap();

            if program_id != nft_breeding_machine_v2::id()
                && program_id != spl_token::id()
                && program_id != anchor_lang::solana_program::system_program::ID
                && program_id != associated_token
            {
                msg!("Transaction had ix with program id {}", program_id);
                return Err(ErrorCode::SuspiciousTransaction.into());
            }
        }

        msg!("At the end");
        sol_log_compute_units();
        Ok(())
    }
}