use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

// instantia coinflip contract with ownder address
#[cw_serde]
pub struct InstantiateMsg {
    // No fields are needed for now
}

#[cw_serde]
pub enum ExecuteMsg {
    Flip { bet: u128 },
    Withdraw { amount: u128 },
}

#[cw_serde]
pub struct FlipResp {
    pub result: bool, // true for win, false for loss
    pub winnings: u128,
}

#[cw_serde]
pub struct PlayerWinningsResp {
    pub winnings: u128,
}

#[cw_serde]
pub struct OwnerResp {
    pub owner: Addr,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // #[returns(PlayerWinningsResp)]
    // PlayerWinnings { address: String },
    #[returns(OwnerResp)]
    Owner {},
}
