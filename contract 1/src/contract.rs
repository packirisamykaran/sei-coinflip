use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};

use crate::error::ContractError;
use crate::msg::GetOwnerResponse;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::OWNER;

// intialize the ownder
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
        ExecuteMsg::Withdraw { amount } => try_withdraw(deps, env, info, amount),
    }
}

fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let owner = OWNER.load(deps.storage)?;

    // Check if the sender is the owner
    if info.sender != owner {
        return Err(ContractError::Unauthorized {});
    }

    // Query the contract's balance
    let balance = deps.querier.query_balance(env.contract.address, "usei")?;

    // Check if the contract has enough balance to cover the withdrawal
    if balance.amount < amount {
        return Err(ContractError::InsufficientFunds {});
    }

    // Create a response to send the specified amount to the owner
    let res = Response::new()
        .add_message(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![coin(amount.u128(), "usei")],
        })
        .add_attribute("method", "withdraw")
        .add_attribute("owner", info.sender.to_string())
        .add_attribute("amount", amount.to_string());

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => query::get_owner(deps),
    }
}

pub mod query {
    use super::*;

    pub fn get_owner(deps: Deps) -> StdResult<Binary> {
        let owner = OWNER.load(deps.storage)?;
        to_json_binary(&GetOwnerResponse { owner })
    }
}

#[cfg(test)]
mod tests {}
