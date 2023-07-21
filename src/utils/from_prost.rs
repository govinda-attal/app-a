use crate::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use prost_types::Timestamp;

// impl TryFrom<W<&Timestamp>> for DateTime<chrono::Utc> {
//     type Error = Error;
//     fn try_from(val: W<&Timestamp>) -> Result<Self> {
//         let val = val.0;
//         let rs = NaiveDateTime::from_timestamp_opt(val.seconds, val.nanos as u32);
//         let rs = rs.ok_or(Error::BadRquest(f!("invalid timestamp value {:?}", val)))?;
//         Ok(DateTime::<Utc>::from_utc(rs, Utc))
//     }
// }

// impl TryFrom<W<Option<&Timestamp>>> for Option<DateTime<chrono::Utc>> {
//     type Error = Error;
//     fn try_from(val: W<Option<&Timestamp>>) -> Result<Self> {
//         let val = val.0;
//         let Some(val) = val else {
//             return Ok(None);
//         };
//         let rs = NaiveDateTime::from_timestamp_opt(val.seconds, val.nanos as u32);
//         let rs = rs.ok_or(Error::BadRquest(f!("invalid timestamp value {:?}", val)))?;
//         Ok(Some(DateTime::<Utc>::from_utc(rs, Utc)))
//     }
// }
