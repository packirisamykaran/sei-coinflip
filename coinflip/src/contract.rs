use crate::error::ContractError;
use crate::msg::{ExecuteMsg, FlipResp, InstantiateMsg, OwnerResp, PlayerWinningsResp, QueryMsg};
use crate::state::{OWNER, PLAYER_WINNINGS};
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_storage_plus::Item;
use std::collections::HashMap;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    OWNER.save(deps.storage, &info.sender)?;
    PLAYER_WINNINGS.save(deps.storage, &HashMap::new())?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Currently, no execution functionality is implemented.
    Err(ContractError::Unauthorized {})
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            to_json_binary(&OwnerResp { owner })
        }
    }
}
