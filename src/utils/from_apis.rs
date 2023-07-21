use crate::api::v1::AuctionInfo;
use crate::db::model::*;
use crate::prelude::*;
use chrono::{DateTime, Utc};

impl TryFrom<&AuctionInfo> for Auction {
    type Error = Error;

    fn try_from(val: &AuctionInfo) -> Result<Self> {
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
