use anchor_lang::prelude::*;

#[error_code]
pub enum DErrorCode {
    #[msg("Unauthorized access")]
    UnauthorizedAccess,

    #[msg("Invalid asset type")]
    InvalidAssetType,

    #[msg("Invalid position size")]
    InvalidPositionSize,

    #[msg("Invalid leverage")]
    InvalidLeverage,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Position already exists")]
    PositionAlreadyExists,

    #[msg("No position exists to close")]
    NoPositionExists,

    #[msg("Invalid oracle account")]
    InvalidOracleAccount,

    #[msg("Invalid oracle price")]
    InvalidOraclePrice,

    #[msg("Math overflow")]
    MathOverflow,
}
