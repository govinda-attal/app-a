use crate::api::v1;
use crate::db::model;
use crate::prelude::*;
use chrono::{DateTime, Timelike, Utc};

impl From<W<&DateTime<Utc>>> for prost_types::Timestamp {
    fn from(val: W<&DateTime<Utc>>) -> Self {
        let val = val.0;
        prost_types::Timestamp {
            seconds: val.timestamp(),
            nanos: val.nanosecond() as i32,
        }
    }
}

impl From<W<Option<&DateTime<Utc>>>> for Option<prost_types::Timestamp> {
    fn from(val: W<Option<&DateTime<Utc>>>) -> Self {
        let val = val.0;
        let Some(val) = val else {
            return None;
        };
        Some(W(val).into())
    }
}

impl From<&model::Auction> for v1::AuctionRec {
    fn from(val: &model::Auction) -> Self {
        v1::AuctionRec {
            id: val.id.unwrap().to_string(),
            info: Some(v1::AuctionInfo::from(val)),
            status: v1::AuctionStatus::from(&val.status) as i32,
            created_at: W(val.created_at.as_ref()).into(),
            updated_at: W(val.last_updated_at.as_ref()).into(),
        }
    }
}

impl From<&model::Auction> for v1::AuctionInfo {
    fn from(val: &model::Auction) -> Self {
        v1::AuctionInfo {
            item: val.item.clone(),
            description: val.description.clone(),
            seller: val.seller.clone(),
            start_price: val.start_price as u32,
            current_price: val.current_price.unwrap_or_default() as u32,
        }
    }
}

impl From<&model::AuctionStatus> for v1::AuctionStatus {
    fn from(s: &model::AuctionStatus) -> Self {
        use model::AuctionStatus;
        match s {
            AuctionStatus::Draft => Self::Draft,
            AuctionStatus::Open => Self::Open,
            AuctionStatus::Closed => Self::Closed,
            AuctionStatus::Cancelled => Self::Cancelled,
        }
    }
}
