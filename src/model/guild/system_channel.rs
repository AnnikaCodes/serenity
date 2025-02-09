use bitflags::__impl_bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::internal::prelude::StdResult;
use crate::model::utils::U64Visitor;

/// Describes a system channel flags.
#[derive(Copy, PartialEq, Eq, Clone, PartialOrd, Ord, Hash, Default)]
pub struct SystemChannelFlags {
    pub bits: u64,
}

__impl_bitflags! {
    SystemChannelFlags: u64 {
        /// Suppress member join notifications.
        SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
        /// Suppress server boost notifications.
        SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
        /// Suppress server setup tips.
        SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
        /// Hide member join sticker reply buttons.
        SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;
    }
}

impl<'de> Deserialize<'de> for SystemChannelFlags {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(SystemChannelFlags::from_bits_truncate(deserializer.deserialize_u64(U64Visitor)?))
    }
}

impl Serialize for SystemChannelFlags {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
