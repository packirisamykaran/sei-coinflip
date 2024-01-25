use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    FlipCoin {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOwnerResponse)]
    GetOwner {},
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: Addr,
}
