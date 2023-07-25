use std::str::FromStr;

use crate::api::v1;
use crate::db::model::*;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl TryFrom<&v1::AuctionInfo> for Auction {
    type Error = Error;

    fn try_from(val: &v1::AuctionInfo) -> Result<Self> {
        let cur_price = if val.current_price != 0 {
            Some(val.current_price as i32)
        } else {
            None
        };

        let rs = Auction {
            item: val.item.clone(),
            description: val.description.clone(),
            seller: val.seller.clone(),
            current_price: cur_price,
            status: AuctionStatus::Draft,
            start_price: val.start_price as i32,
            ..Default::default()
        };

        Ok(rs)
    }
}

impl TryFrom<&v1::AuctionStatus> for AuctionStatus {
    type Error = Error;

    fn try_from(val: &v1::AuctionStatus) -> Result<Self> {
        use v1::AuctionStatus::*;
        let rs = match val {
            Unspecified => Err(Error::BadRquest("invalid auction status".to_string()))?,
            Draft => Self::Draft,
            Open => Self::Open,
            Closed => Self::Closed,
            Cancelled => Self::Cancelled,
        };
        Ok(rs)
    }
}

impl TryFrom<&v1::BidInfo> for Bid {
    type Error = Error;

    fn try_from(val: &v1::BidInfo) -> Result<Self> {
        let rs = Bid {
            auction_id: Uuid::from_str(&val.auction_id)?,
            bidder: val.bidder.clone(),
            bid_price: val.bid_price as i32,
            status: BidStatus::Empty,
            ..Default::default()
        };
        Ok(rs)
    }
}
