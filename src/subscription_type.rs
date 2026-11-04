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
    /// A user is notified if a message is caught by automod for review.
    ///
    /// [Automod Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold)
    AutomodMessageHold => ("automod.message.hold", "1"),
    /// A user is notified if a message is caught by automod for review.  Only public blocked terms trigger notifications, not private ones.
    ///
    /// [Automod Message Hold V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold-v2)
    AutomodMessageHoldV2 => ("automod.message.hold", "2"),
    /// A message in the automod queue had its status changed.
    ///
    /// [Automod Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate)
    AutomodMessageUpdate => ("automod.message.update", "1"),
    /// A message in the automod queue had its status changed. Only public blocked terms trigger notifications, not private ones.
    ///
    /// [Automod Message Update V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate-v2)
    AutomodMessageUpdateV2 => ("automod.message.update", "2"),
    /// A notification is sent when a broadcaster’s automod settings are updated.
    ///
    /// [Automod Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodsettingsupdate)
    AutomodSettingsUpdate => ("automod.settings.update", "1"),
    /// A notification is sent when a broadcaster’s automod terms are updated. Changes to private terms are not sent.
    ///
    /// [Automod Terms Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodtermsupdate)
    AutomodTermsUpdate => ("automod.terms.update", "1"),
    /// A midroll commercial break has started running.
    ///
    /// [Channel Ad Break Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin)
    ChannelAdBreakBegin => ("channel.ad_break.begin", "1"),
    /// A viewer is banned from the specified channel.
    ///
    /// [Channel Ban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelban)
    ChannelBan => ("channel.ban", "1"),
    /// A notification is sent whenever Bits are used on a channel.
    ///
    /// [Channel Bits Use](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelbitsuse)
    ChannelBitsUse => ("channel.bits.use", "1"),
    /// A viewer has redeemed an automatic channel points reward on the specified channel.
    ///
    /// [Channel Points Automatic Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)
    ChannelPointsAutomaticRewardRedemptionAdd => ("channel.channel_points_automatic_reward_redemption.add", "1"),
    /// A viewer has redeemed an automatic channel points reward on the specified channel.
    ///
    /// [Channel Points Automatic Reward Redemption Add V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd-v2)
    ChannelPointsAutomaticRewardRedemptionAddV2 => ("channel.channel_points_automatic_reward_redemption.add", "2"),
    /// A custom channel points reward has been created for the specified channel.
    ///
    /// [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)
    ChannelPointsCustomRewardAdd => ("channel.channel_points_custom_reward.add", "1"),
    /// A custom channel points reward has been removed from the specified channel.
    ///
    /// [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)
    ChannelPointsCustomRewardRemove => ("channel.channel_points_custom_reward.remove", "1"),
    /// A custom channel points reward has been updated for the specified channel.
    ///
    /// [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)
    ChannelPointsCustomRewardUpdate => ("channel.channel_points_custom_reward.update", "1"),
    /// A viewer has redeemed a custom channel points reward on the specified channel.
    ///
    /// [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)
    ChannelPointsCustomRewardRedemptionAdd => ("channel.channel_points_custom_reward_redemption.add", "1"),
    /// A redemption of a channel points custom reward has been updated for the specified channel.
    ///
    /// [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)
    ChannelPointsCustomRewardRedemptionUpdate => ("channel.channel_points_custom_reward_redemption.update", "1"),
    /// Sends an event notification when a user donates to the broadcaster’s charity campaign.
    ///
    /// [Charity Donation](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaigndonate)
    ChannelCharityCampaignDonate => ("channel.charity_campaign.donate", "1"),
    /// Sends an event notification when progress is made towards the campaign’s goal or when the broadcaster changes the fundraising goal.
    ///
    /// [Charity Campaign Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignprogress)
    ChannelCharityCampaignProgress => ("channel.charity_campaign.progress", "1"),
    /// Sends an event notification when the broadcaster starts a charity campaign.
    ///
    /// [Charity Campaign Start](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstart)
    ChannelCharityCampaignStart => ("channel.charity_campaign.start", "1"),
    /// Sends an event notification when the broadcaster stops a charity campaign.
    ///
    /// [Charity Campaign Stop](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstop)
    ChannelCharityCampaignStop => ("channel.charity_campaign.stop", "1"),
    /// A moderator or bot has cleared all messages from the chat room.
    ///
    /// [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)
    ChannelChatClear => ("channel.chat.clear", "1"),
    /// A moderator or bot has cleared all messages from a specific user.
    ///
    /// [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)
    ChannelChatClearUserMessages => ("channel.chat.clear_user_messages", "1"),
    /// Any user sends a message to a specific chat room.
    ///
    /// [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)
    ChannelChatMessage => ("channel.chat.message", "1"),
    /// A moderator has removed a specific message.
    ///
    /// [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)
    ChannelChatMessageDelete => ("channel.chat.message_delete", "1"),
    /// A notification for when an event that appears in chat has occurred.
    ///
    /// [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)
    ChannelChatNotification => ("channel.chat.notification", "1"),
    /// A user is notified if their message is caught by automod.
    ///
    /// [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_hold)
    ChannelChatUserMessageHold => ("channel.chat.user_message_hold", "1"),
    /// A user is notified if their message’s automod status is updated.
    ///
    /// [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)
    ChannelChatUserMessageUpdate => ("channel.chat.user_message_update", "1"),
    /// A notification for when a broadcaster’s chat settings are updated.
    ///
    /// [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)
    ChannelChatSettingsUpdate => ("channel.chat_settings.update", "1"),
    /// A user cheers on the specified channel.
    ///
    /// [Channel Cheer](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer)
    ChannelCheer => ("channel.cheer", "1"),
    /// A viewer has redeemed a custom Power-up on the specified channel.
    ///
    /// [Channel Custom Power-ups Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcustom_power_up_redemptionadd)
    ChannelCustomPowerUpRedemptionAdd => ("channel.custom_power_up_redemption.add", "1"),
    /// A specified channel receives a follow.
    ///
    /// [Channel Follow](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow)
    ChannelFollowV2 => ("channel.follow", "2"),
    /// Get notified when a broadcaster begins a goal.
    ///
    /// [Goal Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalbegin)
    ChannelGoalBegin => ("channel.goal.begin", "1"),
    /// Get notified when a broadcaster ends a goal.
    ///
    /// [Goal End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalend)
    ChannelGoalEnd => ("channel.goal.end", "1"),
    /// Get notified when progress (either positive or negative) is made towards a broadcaster’s goal.
    ///
    /// [Goal Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalprogress)
    ChannelGoalProgress => ("channel.goal.progress", "1"),
    /// A guest or a slot is updated in an active Guest Star session.
    ///
    /// [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate) (beta)
    ChannelGuestStarGuestUpdate => ("channel.guest_star_guest.update", "beta"),
    /// The host began a new Guest Star session.
    ///
    /// [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin) (beta)
    ChannelGuestStarSessionBegin => ("channel.guest_star_session.begin", "beta"),
    /// A running Guest Star session has ended.
    ///
    /// [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend) (beta)
    ChannelGuestStarSessionEnd => ("channel.guest_star_session.end", "beta"),
    /// The host preferences for Guest Star have been updated.
    ///
    /// [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate) (beta)
    ChannelGuestStarSettingsUpdate => ("channel.guest_star_settings.update", "beta"),
    /// A Hype Train begins on the specified channel.
    ///
    /// [Hype Train Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainbegin)
    ChannelHypeTrainBeginV2 => ("channel.hype_train.begin", "2"),
    /// A Hype Train ends on the specified channel.
    ///
    /// [Hype Train End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainend)
    ChannelHypeTrainEndV2 => ("channel.hype_train.end", "2"),
    /// A Hype Train makes progress on the specified channel.
    ///
    /// [Hype Train Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainprogress)
    ChannelHypeTrainProgressV2 => ("channel.hype_train.progress", "2"),
    /// A moderator performs a moderation action in a channel.
    ///
    /// [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)
    ChannelModerate => ("channel.moderate", "1"),
    /// A moderator performs a moderation action in a channel. Includes warnings.
    ///
    /// [Channel Moderate V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)
    ChannelModerateV2 => ("channel.moderate", "2"),
    /// Moderator privileges were added to a user on a specified channel.
    ///
    /// [Channel Moderator Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatoradd)
    ChannelModeratorAdd => ("channel.moderator.add", "1"),
    /// Moderator privileges were removed from a user on a specified channel.
    ///
    /// [Channel Moderator Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatorremove)
    ChannelModeratorRemove => ("channel.moderator.remove", "1"),
    /// A poll started on a specified channel.
    ///
    /// [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)
    ChannelPollBegin => ("channel.poll.begin", "1"),
    /// A poll ended on a specified channel.
    ///
    /// [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)
    ChannelPollEnd => ("channel.poll.end", "1"),
    /// Users respond to a poll on a specified channel.
    ///
    /// [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)
    ChannelPollProgress => ("channel.poll.progress", "1"),
    /// A Prediction started on a specified channel.
    ///
    /// [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)
    ChannelPredictionBegin => ("channel.prediction.begin", "1"),
    /// A Prediction ended on a specified channel.
    ///
    /// [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)
    ChannelPredictionEnd => ("channel.prediction.end", "1"),
    /// A Prediction was locked on a specified channel.
    ///
    /// [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)
    ChannelPredictionLock => ("channel.prediction.lock", "1"),
    /// Users participated in a Prediction on a specified channel.
    ///
    /// [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)
    ChannelPredictionProgress => ("channel.prediction.progress", "1"),
    /// A broadcaster raids another broadcaster’s channel.
    ///
    /// [Channel Raid](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelraid)
    ChannelRaid => ("channel.raid", "1"),
    /// A notification when a channel becomes active in an active shared chat session.
    ///
    /// [Channel Shared Chat Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatbegin)
    ChannelSharedChatBegin => ("channel.shared_chat.begin", "1"),
    /// A notification when a channel leaves a shared chat session or the session ends.
    ///
    /// [Channel Shared Chat Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatend)
    ChannelSharedChatEnd => ("channel.shared_chat.end", "1"),
    /// A notification when the active shared chat session the channel is in changes.
    ///
    /// [Channel Shared Chat Session Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatupdate)
    ChannelSharedChatUpdate => ("channel.shared_chat.update", "1"),
    /// Sends a notification when the broadcaster activates Shield Mode.
    ///
    /// [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)
    ChannelShieldModeBegin => ("channel.shield_mode.begin", "1"),
    /// Sends a notification when the broadcaster deactivates Shield Mode.
    ///
    /// [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)
    ChannelShieldModeEnd => ("channel.shield_mode.end", "1"),
    /// Sends a notification when the specified broadcaster sends a Shoutout.
    ///
    /// [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)
    ChannelShoutoutCreate => ("channel.shoutout.create", "1"),
    /// Sends a notification when the specified broadcaster receives a Shoutout.
    ///
    /// [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)
    ChannelShoutoutReceive => ("channel.shoutout.receive", "1"),
    /// A notification is sent when a specified channel receives a subscriber. This does not include resubscribes.
    ///
    /// [Channel Subscribe](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscribe)
    ChannelSubscribe => ("channel.subscribe", "1"),
    /// A notification when a subscription to the specified channel ends.
    ///
    /// [Channel Subscription End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionend)
    ChannelSubscriptionEnd => ("channel.subscription.end", "1"),
    /// A notification when a viewer gives a gift subscription to one or more users in the specified channel.
    ///
    /// [Channel Subscription Gift](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptiongift)
    ChannelSubscriptionGift => ("channel.subscription.gift", "1"),
    /// A notification when a user sends a resubscription chat message in a specific channel.
    ///
    /// [Channel Subscription Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionmessage)
    ChannelSubscriptionMessage => ("channel.subscription.message", "1"),
    /// A chat message has been sent by a suspicious user.
    ///
    /// [Channel Suspicious User Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_usermessage)
    ChannelSuspiciousUserMessage => ("channel.suspicious_user.message", "1"),
    /// A suspicious user has been updated.
    ///
    /// [Channel Suspicious User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_userupdate)
    ChannelSuspiciousUserUpdate => ("channel.suspicious_user.update", "1"),
    /// A viewer is unbanned from the specified channel.
    ///
    /// [Channel Unban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban)
    ChannelUnban => ("channel.unban", "1"),
    /// A user creates an unban request.
    ///
    /// [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)
    ChannelUnbanRequestCreate => ("channel.unban_request.create", "1"),
    /// An unban request has been resolved.
    ///
    /// [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)
    ChannelUnbanRequestResolve => ("channel.unban_request.resolve", "1"),
    /// A broadcaster updates their channel properties e.g., category, title, content classification labels, broadcast, or language.
    ///
    /// [Channel Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelupdate)
    ChannelUpdateV2 => ("channel.update", "2"),
    /// A VIP is added to the channel.
    ///
    /// [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)
    ChannelVipAdd => ("channel.vip.add", "1"),
    /// A VIP is removed from the channel.
    ///
    /// [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)
    ChannelVipRemove => ("channel.vip.remove", "1"),
    /// A user acknowledges a warning. Broadcasters and moderators can see the warning’s details.
    ///
    /// [Channel Warning Acknowledgement](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)
    ChannelWarningAcknowledge => ("channel.warning.acknowledge", "1"),
    /// A user is sent a warning. Broadcasters and moderators can see the warning’s details.
    ///
    /// [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend)
    ChannelWarningSend => ("channel.warning.send", "1"),
    /// Sends a notification when EventSub disables a shard due to the status of the underlying transport changing.
    ///
    /// [Conduit Shard Disabled](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#conduitsharddisabled)
    ConduitShardDisabled => ("conduit.shard.disabled", "1"),
    /// An entitlement for a Drop is granted to a user.
    ///
    /// [Drop Entitlement Grant](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#dropentitlementgrant)
    DropEntitlementGrant => ("drop.entitlement.grant", "1"),
    /// A Bits transaction occurred for a specified Twitch Extension.
    ///
    /// [Extension Bits Transaction Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#extensionbits_transactioncreate)
    ExtensionBitsTransactionCreate => ("extension.bits_transaction.create", "1"),
    /// The specified broadcaster stops a stream.
    ///
    /// [Stream Offline](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#streamoffline)
    StreamOffline => ("stream.offline", "1"),
    /// The specified broadcaster starts a stream.
    ///
    /// [Stream Online](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#streamonline)
    StreamOnline => ("stream.online", "1"),
    /// A user’s authorization has been granted to your client id.
    ///
    /// [User Authorization Grant](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userauthorizationgrant)
    UserAuthorizationGrant => ("user.authorization.grant", "1"),
    /// A user’s authorization has been revoked for your client id.
    ///
    /// [User Authorization Revoke](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userauthorizationrevoke)
    UserAuthorizationRevoke => ("user.authorization.revoke", "1"),
    /// A user has updated their account.
    ///
    /// [User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userupdate)
    UserUpdate => ("user.update", "1"),
    /// A user receives a whisper.
    ///
    /// [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage)
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
