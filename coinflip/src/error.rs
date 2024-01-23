use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // You can add more custom error types as needed
    // For example, if you have specific errors for your contract logic, you can define them here
    // #[error("Some specific error")]
    // SpecificError {},
}
