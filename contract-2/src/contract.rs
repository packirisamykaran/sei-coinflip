use crate::error::ContractError;
use crate::msg::GetOwnerResponse;
use crate::msg::GetPlayerWinningsResponse;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::OWNER;
use crate::state::PLAYER_WINNINGS;

use cosmwasm_std::entry_point;
use cosmwasm_std::Addr;
use cosmwasm_std::Uint128;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use rand::Rng;

const REQUIRED_BALANCE: Uint128 = Uint128::new(50);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::FlipCoin {} => flip_coin(info, env, deps),
    }
}

fn flip_coin(info: MessageInfo, env: Env, deps: DepsMut) -> Result<Response, ContractError> {
    // Convert nanoseconds to seconds
    let seconds = env.block.time.nanos() / 1_000_000_000;
    // Get the last whole number digit of the seconds
    let last_whole_digit = seconds % 10;

    // Convert nanoseconds to tenths of a second (deciseconds)
    let deciseconds = env.block.time.nanos() / 100_000_000;
    // Get the first decimal digit
    let first_decimal_digit = (deciseconds % 10) as u64;

    // Combine the two parts
    let random_num = last_whole_digit * 10 + first_decimal_digit;

    let contract_balance = deps.querier.query_balance(env.contract.address, "usei")?;

    // Determine the win chance based on the contract's balance
    let win_chance = if contract_balance.amount < REQUIRED_BALANCE {
        10
    } else {
        50
    };

    // Determine the coin flip result
    let result = if random_num < win_chance { true } else { false };
    let mut tokens_received = Uint128::new(0);

    if result {
        // Tokens sent ot the contract

        for coin in info.funds.iter() {
            if coin.denom == "usei" {
                // Found the 'usei' denom
                tokens_received = coin.amount;

                break; // Exit the loop as we've found the desired denom
            }
        }
        if (tokens_received > Uint128::new(0)) {
            // Calculate the winnings
            let winnings = tokens_received * Uint128::new(2);

            // Set the player's winnings
            set_player_winnings(deps, info.sender.clone(), winnings)?;
        }
    }

    Ok(Response::new()
        .add_attribute("flip_result", result.to_string())
        .add_attribute("account_balance", contract_balance.amount)
        .add_attribute("required_balance", REQUIRED_BALANCE)
        .add_attribute("win_chance", win_chance.to_string())
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("tokens_received", tokens_received.to_string()))
}

fn set_player_winnings(deps: DepsMut, player: Addr, winnings: Uint128) -> StdResult<()> {
    PLAYER_WINNINGS.save(deps.storage, &player, &winnings)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => query::get_owner(deps),
        QueryMsg::GetPlayerWinnings { address } => query::get_player_winnings(deps, address),
    }
}

pub mod query {
    use super::*;

    pub fn get_owner(deps: Deps) -> StdResult<Binary> {
        let owner = OWNER.load(deps.storage)?;
        to_json_binary(&GetOwnerResponse { owner })
    }

    pub fn get_player_winnings(deps: Deps, player: Addr) -> StdResult<Binary> {
        let winnings = PLAYER_WINNINGS.load(deps.storage, &player)?;
        to_json_binary(&GetPlayerWinningsResponse { winnings })
    }
}
