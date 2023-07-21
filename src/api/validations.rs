use crate::prelude::*;
use crate::utils::validate;

use super::v1::DraftAuctionRq;
pub trait SimpleValidation {
    fn validate(&self) -> Result<&Self>
    where
        Self: Sized;
}

impl SimpleValidation for DraftAuctionRq {
    fn validate(&self) -> Result<&Self> {
        let info = validate::ok_or_err!(self.info, Error::BadRquest, "info is required")?;

        validate::not_empty!(info.item, Error::BadRquest);
        validate::not_empty!(info.description, Error::BadRquest);
        validate::not_empty!(info.seller, Error::BadRquest);
        validate::positive!(info.start_price, Error::BadRquest);

        Ok(self)
    }
}
