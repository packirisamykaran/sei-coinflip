use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");

pub const PLAYER_WINNINGS: Map<&Addr, Uint128> = Map::new("player_winnings");
