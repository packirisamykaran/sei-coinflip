pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
use cosmwasm_std::{entry_point, Binary, DepsMut, Env, MessageInfo, Response, StdResult};
use msg::InstantiateMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}
