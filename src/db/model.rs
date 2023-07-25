use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use sqlx::{encode::Encode, postgres::types::PgMoney};
use uuid::Uuid;

#[derive(
    strum_macros::Display, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(type_name = "AUCTION_STATUS", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuctionStatus {
    #[default]
    Draft,
    Open,
    Closed,
    Cancelled,
}

#[derive(PartialEq, Debug, sqlx::Decode, sqlx::Encode)]
pub struct AuctionStatuses(pub Vec<AuctionStatus>);

#[derive(
    strum_macros::Display, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(type_name = "BID_STATUS", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BidStatus {
    #[default]
    Empty,
    Accepted,
    Rejected,
    OverTurned,
}

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Auction {
    pub id: Option<Uuid>,
    pub item: String,
    pub description: String,
    pub seller: String,
    pub start_price: i32,
    pub current_price: Option<i32>,
    pub status: AuctionStatus,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub last_updated_at: Option<DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct Bid {
    pub id: Option<Uuid>,
    pub auction_id: Uuid,
    pub bidder: String,
    pub bid_price: i32,
    pub status: BidStatus,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub last_updated_at: Option<DateTime<chrono::Utc>>,
}
