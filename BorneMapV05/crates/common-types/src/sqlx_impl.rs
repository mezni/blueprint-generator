use crate::*;
use sqlx::postgres::{PgTypeInfo, PgValueRef};
use sqlx::{Decode, Postgres, Type};

macro_rules! impl_sqlx_enum {
    ($ty:ty) => {
        impl Type<Postgres> for $ty {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("text")
            }
        }

        impl<'r> Decode<'r, Postgres> for $ty {
            fn decode(value: PgValueRef<'r>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                let s = <&str as Decode<'r, Postgres>>::decode(value)?;
                <$ty>::from_str(s).ok_or_else(|| format!("invalid variant for {}: {}", stringify!($ty), s).into())
            }
        }
    };
}

impl_sqlx_enum!(PartnerStatus);
impl_sqlx_enum!(StationStatus);
impl_sqlx_enum!(StationAvailabilityStatus);
impl_sqlx_enum!(ChargerStatus);
impl_sqlx_enum!(ChargerType);
impl_sqlx_enum!(AvailabilitySource);
impl_sqlx_enum!(ReviewStatus);
impl_sqlx_enum!(GisQueueStatus);
