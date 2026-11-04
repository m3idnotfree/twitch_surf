---
checked: 2026-11-05
source: https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/
---

## Subscription Types

### `automod.message.hold` v1 [Automod Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold)

A user is notified if a message is caught by automod for review.

### `automod.message.hold` v2 [Automod Message Hold V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessagehold-v2)

A user is notified if a message is caught by automod for review.  Only public blocked terms trigger notifications, not private ones.

### `automod.message.update` v1 [Automod Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate)

A message in the automod queue had its status changed.

### `automod.message.update` v2 [Automod Message Update V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodmessageupdate-v2)

A message in the automod queue had its status changed. Only public blocked terms trigger notifications, not private ones.

### `automod.settings.update` v1 [Automod Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodsettingsupdate)

A notification is sent when a broadcaster’s automod settings are updated.

### `automod.terms.update` v1 [Automod Terms Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#automodtermsupdate)

A notification is sent when a broadcaster’s automod terms are updated. Changes to private terms are not sent.

### `channel.bits.use` v1 [Channel Bits Use](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelbitsuse) (new)

A notification is sent whenever Bits are used on a channel.

### `channel.update` v2 [Channel Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelupdate)

A broadcaster updates their channel properties e.g., category, title, content classification labels, broadcast, or language.

### `channel.follow` v2 [Channel Follow](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelfollow)

A specified channel receives a follow.

### `channel.ad_break.begin` v1 [Channel Ad Break Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelad_breakbegin)

A midroll commercial break has started running.

### `channel.chat.clear` v1 [Channel Chat Clear](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear)

A moderator or bot has cleared all messages from the chat room.

### `channel.chat.clear_user_messages` v1 [Channel Chat Clear User Messages](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatclear_user_messages)

A moderator or bot has cleared all messages from a specific user.

### `channel.chat.message` v1 [Channel Chat Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage)

Any user sends a message to a specific chat room.

### `channel.chat.message_delete` v1 [Channel Chat Message Delete](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatmessage_delete)

A moderator has removed a specific message.

### `channel.chat.notification` v1 [Channel Chat Notification](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatnotification)

A notification for when an event that appears in chat has occurred.

### `channel.chat_settings.update` v1 [Channel Chat Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchat_settingsupdate)

A notification for when a broadcaster’s chat settings are updated.

### `channel.chat.user_message_hold` v1 [Channel Chat User Message Hold](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_hold)

A user is notified if their message is caught by automod.

### `channel.chat.user_message_update` v1 [Channel Chat User Message Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchatuser_message_update)

A user is notified if their message’s automod status is updated.

### `channel.shared_chat.begin` v1 [Channel Shared Chat Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatbegin)

A notification when a channel becomes active in an active shared chat session.

### `channel.shared_chat.update` v1 [Channel Shared Chat Session Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatupdate)

A notification when the active shared chat session the channel is in changes.

### `channel.shared_chat.end` v1 [Channel Shared Chat Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshared_chatend)

A notification when a channel leaves a shared chat session or the session ends.

### `channel.subscribe` v1 [Channel Subscribe](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscribe)

A notification is sent when a specified channel receives a subscriber. This does not include resubscribes.

### `channel.subscription.end` v1 [Channel Subscription End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionend)

A notification when a subscription to the specified channel ends.

### `channel.subscription.gift` v1 [Channel Subscription Gift](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptiongift)

A notification when a viewer gives a gift subscription to one or more users in the specified channel.

### `channel.subscription.message` v1 [Channel Subscription Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsubscriptionmessage)

A notification when a user sends a resubscription chat message in a specific channel.

### `channel.cheer` v1 [Channel Cheer](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcheer)

A user cheers on the specified channel.

### `channel.raid` v1 [Channel Raid](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelraid)

A broadcaster raids another broadcaster’s channel.

### `channel.ban` v1 [Channel Ban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelban)

A viewer is banned from the specified channel.

### `channel.unban` v1 [Channel Unban](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban)

A viewer is unbanned from the specified channel.

### `channel.unban_request.create` v1 [Channel Unban Request Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestcreate)

A user creates an unban request.

### `channel.unban_request.resolve` v1 [Channel Unban Request Resolve](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelunban_requestresolve)

An unban request has been resolved.

### `channel.moderate` v1 [Channel Moderate](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate)

A moderator performs a moderation action in a channel.

### `channel.moderate` v2 [Channel Moderate V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderate-v2)

A moderator performs a moderation action in a channel. Includes warnings.

### `channel.moderator.add` v1 [Channel Moderator Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatoradd)

Moderator privileges were added to a user on a specified channel.

### `channel.moderator.remove` v1 [Channel Moderator Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelmoderatorremove)

Moderator privileges were removed from a user on a specified channel.

### `channel.guest_star_session.begin` beta [Channel Guest Star Session Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionbegin) (beta)

The host began a new Guest Star session.

### `channel.guest_star_session.end` beta [Channel Guest Star Session End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_sessionend) (beta)

A running Guest Star session has ended.

### `channel.guest_star_guest.update` beta [Channel Guest Star Guest Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_guestupdate) (beta)

A guest or a slot is updated in an active Guest Star session.

### `channel.guest_star_settings.update` beta [Channel Guest Star Settings Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelguest_star_settingsupdate) (beta)

The host preferences for Guest Star have been updated.

### `channel.channel_points_automatic_reward_redemption.add` v1 [Channel Points Automatic Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd)

A viewer has redeemed an automatic channel points reward on the specified channel.

### `channel.channel_points_automatic_reward_redemption.add` v2 [Channel Points Automatic Reward Redemption Add V2](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_automatic_reward_redemptionadd-v2)

A viewer has redeemed an automatic channel points reward on the specified channel.

### `channel.channel_points_custom_reward.add` v1 [Channel Points Custom Reward Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardadd)

A custom channel points reward has been created for the specified channel.

### `channel.channel_points_custom_reward.update` v1 [Channel Points Custom Reward Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardupdate)

A custom channel points reward has been updated for the specified channel.

### `channel.channel_points_custom_reward.remove` v1 [Channel Points Custom Reward Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_rewardremove)

A custom channel points reward has been removed from the specified channel.

### `channel.channel_points_custom_reward_redemption.add` v1 [Channel Points Custom Reward Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionadd)

A viewer has redeemed a custom channel points reward on the specified channel.

### `channel.channel_points_custom_reward_redemption.update` v1 [Channel Points Custom Reward Redemption Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelchannel_points_custom_reward_redemptionupdate)

A redemption of a channel points custom reward has been updated for the specified channel.

### `channel.custom_power_up_redemption.add` v1 [Channel Custom Power-ups Redemption Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcustom_power_up_redemptionadd) (new)

A viewer has redeemed a custom Power-up on the specified channel.

### `channel.poll.begin` v1 [Channel Poll Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollbegin)

A poll started on a specified channel.

### `channel.poll.progress` v1 [Channel Poll Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollprogress)

Users respond to a poll on a specified channel.

### `channel.poll.end` v1 [Channel Poll End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpollend)

A poll ended on a specified channel.

### `channel.prediction.begin` v1 [Channel Prediction Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionbegin)

A Prediction started on a specified channel.

### `channel.prediction.progress` v1 [Channel Prediction Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionprogress)

Users participated in a Prediction on a specified channel.

### `channel.prediction.lock` v1 [Channel Prediction Lock](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionlock)

A Prediction was locked on a specified channel.

### `channel.prediction.end` v1 [Channel Prediction End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelpredictionend)

A Prediction ended on a specified channel.

### `channel.suspicious_user.message` v1 [Channel Suspicious User Message](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_usermessage) (new)

A chat message has been sent by a suspicious user.

### `channel.suspicious_user.update` v1 [Channel Suspicious User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelsuspicious_userupdate) (new)

A suspicious user has been updated.

### `channel.vip.add` v1 [Channel VIP Add](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipadd)

A VIP is added to the channel.

### `channel.vip.remove` v1 [Channel VIP Remove](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove)

A VIP is removed from the channel.

### `channel.warning.acknowledge` v1 [Channel Warning Acknowledgement](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningacknowledge)

A user acknowledges a warning. Broadcasters and moderators can see the warning’s details.

### `channel.warning.send` v1 [Channel Warning Send](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelwarningsend) (new)

A user is sent a warning. Broadcasters and moderators can see the warning’s details.

### `channel.charity_campaign.donate` v1 [Charity Donation](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaigndonate)

Sends an event notification when a user donates to the broadcaster’s charity campaign.

### `channel.charity_campaign.start` v1 [Charity Campaign Start](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstart)

Sends an event notification when the broadcaster starts a charity campaign.

### `channel.charity_campaign.progress` v1 [Charity Campaign Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignprogress)

Sends an event notification when progress is made towards the campaign’s goal or when the broadcaster changes the fundraising goal.

### `channel.charity_campaign.stop` v1 [Charity Campaign Stop](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelcharity_campaignstop)

Sends an event notification when the broadcaster stops a charity campaign.

### `conduit.shard.disabled` v1 [Conduit Shard Disabled](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#conduitsharddisabled) (new)

Sends a notification when EventSub disables a shard due to the status of the underlying transport changing.

### `drop.entitlement.grant` v1 [Drop Entitlement Grant](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#dropentitlementgrant)

An entitlement for a Drop is granted to a user.

### `extension.bits_transaction.create` v1 [Extension Bits Transaction Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#extensionbits_transactioncreate)

A Bits transaction occurred for a specified Twitch Extension.

### `channel.goal.begin` v1 [Goal Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalbegin)

Get notified when a broadcaster begins a goal.

### `channel.goal.progress` v1 [Goal Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalprogress)

Get notified when progress (either positive or negative) is made towards a broadcaster’s goal.

### `channel.goal.end` v1 [Goal End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelgoalend)

Get notified when a broadcaster ends a goal.

### `channel.hype_train.begin` v2 [Hype Train Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainbegin)

A Hype Train begins on the specified channel.

### `channel.hype_train.progress` v2 [Hype Train Progress](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainprogress)

A Hype Train makes progress on the specified channel.

### `channel.hype_train.end` v2 [Hype Train End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelhype_trainend)

A Hype Train ends on the specified channel.

### `channel.shield_mode.begin` v1 [Shield Mode Begin](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modebegin)

Sends a notification when the broadcaster activates Shield Mode.

### `channel.shield_mode.end` v1 [Shield Mode End](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshield_modeend)

Sends a notification when the broadcaster deactivates Shield Mode.

### `channel.shoutout.create` v1 [Shoutout Create](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutcreate)

Sends a notification when the specified broadcaster sends a Shoutout.

### `channel.shoutout.receive` v1 [Shoutout Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelshoutoutreceive)

Sends a notification when the specified broadcaster receives a Shoutout.

### `stream.online` v1 [Stream Online](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#streamonline)

The specified broadcaster starts a stream.

### `stream.offline` v1 [Stream Offline](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#streamoffline)

The specified broadcaster stops a stream.

### `user.authorization.grant` v1 [User Authorization Grant](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userauthorizationgrant)

A user’s authorization has been granted to your client id.

### `user.authorization.revoke` v1 [User Authorization Revoke](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userauthorizationrevoke)

A user’s authorization has been revoked for your client id.

### `user.update` v1 [User Update](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userupdate)

A user has updated their account.

### `user.whisper.message` v1 [Whisper Received](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#userwhispermessage) (new)

A user receives a whisper.
