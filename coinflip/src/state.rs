use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use std::collections::HashMap;

// pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");

// Constant for the owner's address
pub const OWNER: Item<Addr> = Item::new("owner");
// Constant for player winnings
pub const PLAYER_WINNINGS: Item<HashMap<Addr, u128>> = Item::new("player_winnings");
