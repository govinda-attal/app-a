use crate::prelude::*;
use crate::utils::validate;

use super::v1::{
    CancelAuctionRq, CloseAuctionRq, DraftAuctionRq, FetchAuctionRq, NewBidRq, QueryAuctionsRq,
    QueryBidsRq, StartAuctionRq,
};
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

impl SimpleValidation for StartAuctionRq {
    fn validate(&self) -> Result<&Self> {
        let auction_id = &self.auction_id;
        validate::not_empty!(auction_id, Error::BadRquest);
        validate::uuid!(auction_id, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for CloseAuctionRq {
    fn validate(&self) -> Result<&Self> {
        let auction_id = &self.auction_id;
        validate::not_empty!(auction_id, Error::BadRquest);
        validate::uuid!(auction_id, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for CancelAuctionRq {
    fn validate(&self) -> Result<&Self> {
        let auction_id = &self.auction_id;
        validate::not_empty!(auction_id, Error::BadRquest);
        validate::uuid!(auction_id, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for NewBidRq {
    fn validate(&self) -> Result<&Self> {
        let info = validate::ok_or_err!(self.info, Error::BadRquest, "info is required")?;
        validate::not_empty!(info.auction_id, Error::BadRquest);
        validate::uuid!(info.auction_id, Error::BadRquest);
        validate::not_empty!(info.bidder, Error::BadRquest);
        validate::positive!(info.bid_price, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for FetchAuctionRq {
    fn validate(&self) -> Result<&Self> {
        let auction_id = &self.auction_id;
        validate::not_empty!(auction_id, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for QueryAuctionsRq {
    fn validate(&self) -> Result<&Self> {
        let seller = &self.seller;
        validate::not_empty!(seller, Error::BadRquest);
        Ok(self)
    }
}

impl SimpleValidation for QueryBidsRq {
    fn validate(&self) -> Result<&Self> {
        let auction_id = &self.auction_id;
        validate::not_empty!(auction_id, Error::BadRquest);
        Ok(self)
    }
}
