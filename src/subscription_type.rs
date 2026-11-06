use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

macro_rules! types {
    ($(
        $(#[$meta:meta])*
        $name:ident => ($kind:literal, $version:literal),
    )+) => {
        /// <https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/>
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[non_exhaustive]
        pub enum SubscriptionType {
            $(
                $(#[$meta])*
                $name,
            )+
        }

        impl SubscriptionType {
            pub const ALL: &[SubscriptionType] = &[$(Self::$name),+];

            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$name => $kind,)+
                }
            }

            pub const fn version(&self) -> &'static str {
                match self {
                    $(Self::$name => $version,)+
                }
            }

            pub fn parse(kind: &str, version: &str) -> Result<Self, ParseSubscriptionTypeError> {
                match (kind, version) {
                    $(($kind, $version) => Ok(Self::$name),)+
                    _ => Err(ParseSubscriptionTypeError::new(kind, version)),
                }
            }
        }
    };
}

types! {
    AutomodMessageHold => ("automod.message.hold", "1"),
    AutomodMessageHoldV2 => ("automod.message.hold", "2"),
    AutomodMessageUpdate => ("automod.message.update", "1"),
    AutomodMessageUpdateV2 => ("automod.message.update", "2"),
    AutomodSettingsUpdate => ("automod.settings.update", "1"),
    AutomodTermsUpdate => ("automod.terms.update", "1"),
    ChannelAdBreakBegin => ("channel.ad_break.begin", "1"),
    ChannelBan => ("channel.ban", "1"),
    ChannelBitsUse => ("channel.bits.use", "1"),
    ChannelPointsAutomaticRewardRedemptionAdd => ("channel.channel_points_automatic_reward_redemption.add", "1"),
    ChannelPointsAutomaticRewardRedemptionAddV2 => ("channel.channel_points_automatic_reward_redemption.add", "2"),
    ChannelPointsCustomRewardAdd => ("channel.channel_points_custom_reward.add", "1"),
    ChannelPointsCustomRewardRemove => ("channel.channel_points_custom_reward.remove", "1"),
    ChannelPointsCustomRewardUpdate => ("channel.channel_points_custom_reward.update", "1"),
    ChannelPointsCustomRewardRedemptionAdd => ("channel.channel_points_custom_reward_redemption.add", "1"),
    ChannelPointsCustomRewardRedemptionUpdate => ("channel.channel_points_custom_reward_redemption.update", "1"),
    ChannelCharityCampaignDonate => ("channel.charity_campaign.donate", "1"),
    ChannelCharityCampaignProgress => ("channel.charity_campaign.progress", "1"),
    ChannelCharityCampaignStart => ("channel.charity_campaign.start", "1"),
    ChannelCharityCampaignStop => ("channel.charity_campaign.stop", "1"),
    ChannelChatClear => ("channel.chat.clear", "1"),
    ChannelChatClearUserMessages => ("channel.chat.clear_user_messages", "1"),
    ChannelChatMessage => ("channel.chat.message", "1"),
    ChannelChatMessageDelete => ("channel.chat.message_delete", "1"),
    ChannelChatNotification => ("channel.chat.notification", "1"),
    ChannelChatUserMessageHold => ("channel.chat.user_message_hold", "1"),
    ChannelChatUserMessageUpdate => ("channel.chat.user_message_update", "1"),
    ChannelChatSettingsUpdate => ("channel.chat_settings.update", "1"),
    ChannelCheer => ("channel.cheer", "1"),
    ChannelCustomPowerUpRedemptionAdd => ("channel.custom_power_up_redemption.add", "1"),
    ChannelFollowV2 => ("channel.follow", "2"),
    ChannelGoalBegin => ("channel.goal.begin", "1"),
    ChannelGoalEnd => ("channel.goal.end", "1"),
    ChannelGoalProgress => ("channel.goal.progress", "1"),
    ChannelGuestStarGuestUpdate => ("channel.guest_star_guest.update", "beta"),
    ChannelGuestStarSessionBegin => ("channel.guest_star_session.begin", "beta"),
    ChannelGuestStarSessionEnd => ("channel.guest_star_session.end", "beta"),
    ChannelGuestStarSettingsUpdate => ("channel.guest_star_settings.update", "beta"),
    ChannelHypeTrainBeginV2 => ("channel.hype_train.begin", "2"),
    ChannelHypeTrainEndV2 => ("channel.hype_train.end", "2"),
    ChannelHypeTrainProgressV2 => ("channel.hype_train.progress", "2"),
    ChannelModerate => ("channel.moderate", "1"),
    ChannelModerateV2 => ("channel.moderate", "2"),
    ChannelModeratorAdd => ("channel.moderator.add", "1"),
    ChannelModeratorRemove => ("channel.moderator.remove", "1"),
    ChannelPollBegin => ("channel.poll.begin", "1"),
    ChannelPollEnd => ("channel.poll.end", "1"),
    ChannelPollProgress => ("channel.poll.progress", "1"),
    ChannelPredictionBegin => ("channel.prediction.begin", "1"),
    ChannelPredictionEnd => ("channel.prediction.end", "1"),
    ChannelPredictionLock => ("channel.prediction.lock", "1"),
    ChannelPredictionProgress => ("channel.prediction.progress", "1"),
    ChannelRaid => ("channel.raid", "1"),
    ChannelSharedChatBegin => ("channel.shared_chat.begin", "1"),
    ChannelSharedChatEnd => ("channel.shared_chat.end", "1"),
    ChannelSharedChatUpdate => ("channel.shared_chat.update", "1"),
    ChannelShieldModeBegin => ("channel.shield_mode.begin", "1"),
    ChannelShieldModeEnd => ("channel.shield_mode.end", "1"),
    ChannelShoutoutCreate => ("channel.shoutout.create", "1"),
    ChannelShoutoutReceive => ("channel.shoutout.receive", "1"),
    ChannelSubscribe => ("channel.subscribe", "1"),
    ChannelSubscriptionEnd => ("channel.subscription.end", "1"),
    ChannelSubscriptionGift => ("channel.subscription.gift", "1"),
    ChannelSubscriptionMessage => ("channel.subscription.message", "1"),
    ChannelSuspiciousUserMessage => ("channel.suspicious_user.message", "1"),
    ChannelSuspiciousUserUpdate => ("channel.suspicious_user.update", "1"),
    ChannelUnban => ("channel.unban", "1"),
    ChannelUnbanRequestCreate => ("channel.unban_request.create", "1"),
    ChannelUnbanRequestResolve => ("channel.unban_request.resolve", "1"),
    ChannelUpdateV2 => ("channel.update", "2"),
    ChannelVipAdd => ("channel.vip.add", "1"),
    ChannelVipRemove => ("channel.vip.remove", "1"),
    ChannelWarningAcknowledge => ("channel.warning.acknowledge", "1"),
    ChannelWarningSend => ("channel.warning.send", "1"),
    ConduitShardDisabled => ("conduit.shard.disabled", "1"),
    DropEntitlementGrant => ("drop.entitlement.grant", "1"),
    ExtensionBitsTransactionCreate => ("extension.bits_transaction.create", "1"),
    StreamOffline => ("stream.offline", "1"),
    StreamOnline => ("stream.online", "1"),
    UserAuthorizationGrant => ("user.authorization.grant", "1"),
    UserAuthorizationRevoke => ("user.authorization.revoke", "1"),
    UserUpdate => ("user.update", "1"),
    UserWhisperMessage => ("user.whisper.message", "1"),
}

impl Display for SubscriptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for SubscriptionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseSubscriptionTypeError {
    kind: String,
    version: String,
}

impl ParseSubscriptionTypeError {
    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    fn new(kind: &str, version: &str) -> Self {
        Self {
            kind: kind.to_owned(),
            version: version.to_owned(),
        }
    }
}

impl Display for ParseSubscriptionTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "unknown subscription type: {:?} (version {:?})",
            self.kind, self.version
        )
    }
}

impl StdError for ParseSubscriptionTypeError {}

#[cfg(feature = "serde")]
impl serde::Serialize for SubscriptionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriptionType;

    #[test]
    fn round_trip() {
        for &s in SubscriptionType::ALL {
            assert_eq!(SubscriptionType::parse(s.as_str(), s.version()), Ok(s));
        }
    }

    #[test]
    fn version_select() {
        assert_eq!(
            SubscriptionType::parse("automod.message.hold", "1"),
            Ok(SubscriptionType::AutomodMessageHold)
        );
        assert_eq!(
            SubscriptionType::parse("automod.message.hold", "2"),
            Ok(SubscriptionType::AutomodMessageHoldV2)
        );
    }
}
