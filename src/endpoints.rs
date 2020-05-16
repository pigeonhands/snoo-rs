use reqwest::Url;
use std::borrow::Cow;

use std::io;

macro_rules! endpoints {
    ($($name:ident => $ep:tt),*) => {
        $(pub const $name : Endpoint = Endpoint(Cow::Borrowed($ep));)*
    };
}

macro_rules! uri_segments {
    ($($name:ident),*) => {
        $(
        pub fn $name(&self, $name: &str) -> Endpoint {
            self.replace(concat!("#", stringify!($name)), $name)
        }
        )*
    };
}

#[derive(Copy, Clone)]
pub enum SearchSort {
    Relevance,
    Hot,
    Top,
    New,
    Comments,
}

impl SearchSort {
    pub fn to_str(&self) -> &'static str {
        match self {
            SearchSort::Relevance => "relevance",
            SearchSort::Hot => "hot",
            SearchSort::Top => "top",
            SearchSort::New => "new",
            SearchSort::Comments => "comments",
        }
    }
}

pub struct Endpoint(Cow<'static, str>);

impl Endpoint {
    const REDDIT_URL: &'static str = "https://reddit.com";

    pub fn new(ep: &str) -> Endpoint {
        Endpoint(Cow::Owned(ep.to_owned()))
    }

    uri_segments! {
        subreddit,
        id,
        page,
        user
    }

    fn replace(&self, needle: &str, haystack: &str) -> Endpoint {
        Endpoint(self.0.as_ref().replace(needle, haystack).into())
    }

    pub fn as_url(&self) -> io::Result<Url> {
        let url = Url::parse(Endpoint::REDDIT_URL)
            .unwrap()
            .join(&self.0)
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid for url: {}", &self.0),
                )
            })?;

        Ok(url)
    }

    pub fn as_api_endpoint(&self) -> io::Result<Url> {
        let api_endpoint = self.as_url()?.join(".json").unwrap();
        Ok(api_endpoint)
    }

    pub fn as_filter_url(
        &self,
        q: Option<&str>,
        sort: SearchSort,
        before: Option<&str>,
        after: Option<&str>,
    ) -> io::Result<Url> {
        let mut url = self.as_api_endpoint()?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("restrict_sr", "on");
            if let Some(search_string) = q {
                query.append_pair("q", search_string);
            }
            query.append_pair("sort", sort.to_str());

            if let Some(afer_thing) = after {
                query.append_pair("after", afer_thing.as_ref());
            }

            if let Some(before_thing) = before {
                query.append_pair("before", &before_thing.as_ref());
            }
        }
        Ok(url)
    }
}

impl AsRef<str> for Endpoint {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

endpoints! {
    ABOUT_EDITED =>            "r/#subreddit/about/edited/",
    ABOUT_LOG =>               "r/#subreddit/about/log/",
    ABOUT_MODQUEUE =>          "r/#subreddit/about/modqueue/",
    ABOUT_REPORTS =>           "r/#subreddit/about/reports/",
    ABOUT_SPAM =>              "r/#subreddit/about/spam/",
    ABOUT_STICKY =>            "r/#subreddit/about/sticky/",
    ABOUT_STYLESHEET =>        "r/#subreddit/about/stylesheet/",
    ABOUT_TRAFFIC =>           "r/#subreddit/about/traffic/",
    ABOUT_UNMODERATED =>       "r/#subreddit/about/unmoderated/",
    ACCEPT_MOD_INVITE =>       "r/#subreddit/api/accept_moderator_invite/",
    ADD_SUBREDDIT_RULE =>      "api/add_subreddit_rule/",
    APPROVE =>                 "api/approve/",
    BLOCK =>                   "api/block/",
    BLOCK_USER =>              "/api/block_user/",
    BLOCKED =>                 "prefs/blocked/",
    COLLAPSE =>                "api/collapse_message/",
    COLLECTION =>              "api/v1/collections/collection/",
    COLLECTION_ADD_POST =>     "api/v1/collections/add_post_to_collection/",
    COLLECTION_CREATE =>       "api/v1/collections/create_collection/",
    COLLECTION_DELETE =>       "api/v1/collections/delete_collection/",
    COLLECTION_DESC =>         "api/v1/collections/update_collection_description/",
    COLLECTION_FOLLOW =>       "api/v1/collections/follow_collection/",
    COLLECTION_REMOVE_POST =>  "api/v1/collections/remove_post_in_collection/",
    COLLECTION_REORDER =>      "api/v1/collections/reorder_collection/",
    COLLECTION_SUBREDDIT =>    "api/v1/collections/subreddit_collections/",
    COLLECTION_TITLE =>        "api/v1/collections/update_collection_title/",
    COMMENT =>                 "api/comment/",
    COMMENT_REPLIES =>         "message/comments/",
    COMPOSE =>                 "api/compose/",
    CONTEST_MODE =>            "api/set_contest_mode/",
    DEL =>                     "api/del/",
    DELETE_MESSAGE =>          "api/del_msg/",
    DELETE_SR_BANNER =>        "r/#subreddit/api/delete_sr_banner/",
    DELETE_SR_HEADER =>        "r/#subreddit/api/delete_sr_header/",
    DELETE_SR_ICON =>          "r/#subreddit/api/delete_sr_icon/",
    DELETE_SR_IMAGE =>         "r/#subreddit/api/delete_sr_img/",
    DELETEFLAIR =>             "r/#subreddit/api/deleteflair/",
    DISTINGUISH =>             "api/distinguish/",
    DOMAIN =>                  "domain/#domain/",
    DUPLICATES =>              "duplicates/{submission_id}/",
    EDIT =>                    "api/editusertext/",
    EMOJI_DELETE =>            "api/v1/#subreddit/emoji/{emoji_name}",
    EMOJI_LEASE =>             "api/v1/#subreddit/emoji_asset_upload_s3.json/",
    EMOJI_LIST =>              "api/v1/#subreddit/emojis/all/",
    EMOJI_UPDATE =>            "api/v1/#subreddit/emoji_permissions/",
    EMOJI_UPLOAD =>            "api/v1/#subreddit/emoji.json/",
    FLAIR =>                   "r/#subreddit/api/flair/",
    FLAIRCONFIG =>             "r/#subreddit/api/flairconfig/",
    FLAIRCSV =>                "r/#subreddit/api/flaircsv/",
    FLAIRLIST =>               "r/#subreddit/api/flairlist/",
    FLAIRSELECTOR =>           "r/#subreddit/api/flairselector/",
    FLAIRTEMPLATE_V2 =>        "r/#subreddit/api/flairtemplate_v2",
    FLAIRTEMPLATECLEAR =>      "r/#subreddit/api/clearflairtemplates/",
    FLAIRTEMPLATEDELETE =>     "r/#subreddit/api/deleteflairtemplate/",
    FRIEND =>                  "r/#subreddit/api/friend/",
    FRIEND_V1 =>               "api/v1/me/friends/#user/",
    FRIENDS =>                 "api/v1/me/friends/",
    GILD_THING =>              "api/v1/gold/gild/#fullname/",
    GILD_USER =>               "api/v1/gold/give/#username/",
    HIDE =>                    "api/hide/",
    IGNORE_REPORTS =>          "api/ignore_reports/",
    INBOX =>                   "message/inbox/",
    INFO =>                    "api/info/",
    KARMA =>                   "api/v1/me/karma/",
    LEAVECONTRIBUTOR =>        "api/leavecontributor/",
    LINK_FLAIR =>              "r/#subreddit/api/link_flair_v2",
    LIST_BANNED =>             "r/#subreddit/about/banned/",
    LIST_CONTRIBUTOR =>        "r/#subreddit/about/contributors/",
    LIST_MODERATOR =>          "r/#subreddit/about/moderators/",
    LIST_MUTED =>              "r/#subreddit/about/muted/",
    LIST_WIKIBANNED =>         "r/#subreddit/about/wikibanned/",
    LIST_WIKICONTRIBUTOR =>    "r/#subreddit/about/wikicontributors/",
    LIVE_ACCEPT_INVITE =>      "api/live/#id/accept_contributor_invite/",
    LIVE_ADD_UPDATE =>         "api/live/#id/update/",
    LIVE_CLOSE =>              "api/live/#id/close_thread/",
    LIVE_CONTRIBUTORS =>       "live/#id/contributors/",
    LIVE_DISCUSSIONS =>        "live/#id/discussions/",
    LIVE_FOCUS =>              "live/{thread_id}/updates/{update_id}",
    LIVE_INFO =>               "api/live/by_id/#ids/",
    LIVE_INVITE =>             "api/live/#id/invite_contributor/",
    LIVE_LEAVE =>              "api/live/#id/leave_contributor/",
    LIVE_NOW =>                "api/live/happening_now/",
    LIVE_REMOVE_CONTRIB =>     "api/live/#id/rm_contributor/",
    LIVE_REMOVE_INVITE =>      "api/live/#id/rm_contributor_invite/",
    LIVE_REMOVE_UPDATE =>      "api/live/#id/delete_update/",
    LIVE_REPORT =>             "api/live/#id/report/",
    LIVE_STRIKE =>             "api/live/#id/strike_update/",
    LIVE_UPDATE_PERMS =>       "api/live/#id/set_contributor_permissions/",
    LIVE_UPDATE_THREAD =>      "api/live/#id/edit/",
    LIVE_UPDATES =>            "live/#id/",
    LIVEABOUT =>               "api/live/#id/about/",
    LIVECREATE =>              "api/live/create/",
    LOCK =>                    "api/lock/",
    MARKNSFW =>                "api/marknsfw/",
    ME =>                      "api/v1/me/",
    MEDIA_ASSET =>             "api/media/asset.json/",
    MENTIONS =>                "message/mentions/",
    MESSAGE =>                 "message/messages/#id/",
    MESSAGES =>                "message/messages/",
    MODERATED =>               "user/#user/moderated_subreddits/",
    MODERATOR_MESSAGES =>      "r/#subreddit/message/moderator/",
    MODERATOR_UNREAD =>        "r/#subreddit/message/moderator/unread/",
    MODMAIL_ARCHIVE =>         "api/mod/conversations/#id/archive/",
    MODMAIL_BULK_READ =>       "api/mod/conversations/bulk/read/",
    MODMAIL_CONVERSATION =>    "api/mod/conversations/#id/",
    MODMAIL_CONVERSATIONS =>   "api/mod/conversations/",
    MODMAIL_HIGHLIGHT =>       "api/mod/conversations/#id/highlight/",
    MODMAIL_MUTE =>            "api/mod/conversations/#id/mute/",
    MODMAIL_READ =>            "api/mod/conversations/read/",
    MODMAIL_SUBREDDITS =>      "api/mod/conversations/subreddits/",
    MODMAIL_UNARCHIVE =>       "api/mod/conversations/#id/unarchive/",
    MODMAIL_UNMUTE =>          "api/mod/conversations/#id/unmute/",
    MODMAIL_UNREAD =>          "api/mod/conversations/unread/",
    MODMAIL_UNREAD_COUNT =>    "api/mod/conversations/unread/count/",
    MORECHILDREN =>            "api/morechildren/",
    MULTIREDDIT =>             "user/#user/m/#multi/",
    MULTIREDDIT_API =>         "api/multi/user/#user/m/#multi/",
    MULTIREDDIT_BASE =>        "api/multi/",
    MULTIREDDIT_COPY =>        "api/multi/copy/",
    MULTIREDDIT_RENAME =>      "api/multi/rename/",
    MULTIREDDIT_UPDATE =>      "api/multi/user/#user/m/#multi/r/#subreddit/",
    MULTIREDDIT_USER =>        "api/multi/user/#user/",
    MUTE_SENDER =>             "api/mute_message_author/",
    MY_CONTRIBUTOR =>          "subreddits/mine/contributor/",
    MY_MODERATOR =>            "subreddits/mine/moderator/",
    MY_MULTIREDDITS =>         "api/multi/mine/",
    MY_SUBREDDITS =>           "subreddits/mine/subscriber/",
    POST_REQUIREMENTS =>       "api/v1/#subreddit/post_requirements/",
    PREFERENCES =>             "api/v1/me/prefs/",
    QUARANTINE_OPT_IN =>       "api/quarantine_optin/",
    QUARANTINE_OPT_OUT =>      "api/quarantine_optout/",
    READ_MESSAGE =>            "api/read_message/",
    REMOVAL_COMMENT_MESSAGE => "api/v1/modactions/removal_comment_message/",
    REMOVAL_LINK_MESSAGE =>    "api/v1/modactions/removal_link_message/",
    REMOVAL_REASONS =>         "api/v1/modactions/removal_reasons/",
    REMOVAL_REASON =>          "api/v1/#subreddit/removal_reasons/#id/",
    REMOVAL_REASONS_LIST =>    "api/v1/#subreddit/removal_reasons/",
    REMOVE_SUBREDDIT_RULE =>   "api/remove_subreddit_rule/",
    REMOVE =>                  "api/remove/",
    REORDER_SUBREDDIT_RULES => "api/reorder_subreddit_rules/",
    REPORT =>                  "api/report/",
    RULES =>                   "r/#subreddit/about/rules/",
    SAVE =>                    "api/save/",
    SEARCH =>                  "search/",
    SELECT_FLAIR =>            "r/#subreddit/api/selectflair/",
    SENDREPLIES =>             "api/sendreplies/",
    SENT =>                    "message/sent/",
    SET_ORIGINAL_CONTENT =>    "api/set_original_content/",
    SETPERMISSIONS =>          "r/#subreddit/api/setpermissions/",
    SHOW_COMMENT =>            "api/show_comment/",
    SITE_ADMIN =>              "api/site_admin/",
    SPOILER =>                 "api/spoiler/",
    STICKY_SUBMISSION =>       "api/set_subreddit_sticky/",
    STORE_VISITS =>            "api/store_visits/",
    STRUCTURED_STYLES =>       "api/v1/structured_styles/#subreddit/",
    STYLE_ASSET_LEASE =>       "api/v1/style_asset_upload_s3/#subreddit/",
    SUB_RECOMMENDED =>         "api/recommend/sr/#subreddits/",
    SUBMISSION =>              "comments/#id/",
    SUBMISSION_REPLIES =>      "message/selfreply/",
    SUBMIT =>                  "api/submit/",
    SUBMIT_POLL_POST =>        "api/submit_poll_post/",
    SUBREDDIT =>               "r/#subreddit/",
    SUBREDDIT_TOP =>           "r/#subreddit/top/",
    SUBREDDIT_RISING =>        "r/#subreddit/rising/",
    SUBREDDIT_NEW =>           "r/#subreddit/new/",
    SUBREDDIT_HOT =>           "r/#subreddit/hot/",
    SUBREDDIT_ABOUT =>         "r/#subreddit/about/",
    SUBREDDIT_FILTER =>        "api/filter/user/#user/f/#special/r/#subreddit/",
    SUBREDDIT_FILTER_LIST =>   "api/filter/user/#user/f/#special/",
    SUBREDDIT_RANDOM =>        "r/#subreddit/random/",
    SUBREDDIT_SETTINGS =>      "r/#subreddit/about/edit/",
    SUBREDDIT_STYLESHEET =>    "r/#subreddit/api/subreddit_stylesheet/",
    SUBREDDITS_BY_TOPIC =>     "api/subreddits_by_topic/",
    SUBREDDITS_DEFAULT =>      "subreddits/default/",
    SUBREDDITS_GOLD =>         "subreddits/gold/",
    SUBREDDITS_NAME_SEARCH =>  "api/search_reddit_names/",
    SUBREDDITS_NEW =>          "subreddits/new/",
    SUBREDDITS_POPULAR =>      "subreddits/popular/",
    SUBREDDIT_SEARCH =>         "r/#subreddit/search/",
    SUBREDDITS_SEARCH =>       "subreddits/search/",
    SUBSCRIBE =>               "api/subscribe/",
    SUGGESTED_SORT =>          "api/set_suggested_sort/",
    TROPHIES =>                "api/v1/user/#user/trophies/",
    UNCOLLAPSE =>              "api/uncollapse_message/",
    UNFRIEND =>                "r/#subreddit/api/unfriend/",
    UNHIDE =>                  "api/unhide/",
    UNIGNORE_REPORTS =>        "api/unignore_reports/",
    UNLOCK =>                  "api/unlock/",
    UNMARKNSFW =>              "api/unmarknsfw/",
    UNMUTE_SENDER =>           "api/unmute_message_author/",
    UNREAD =>                  "message/unread/",
    UNREAD_MESSAGE =>          "api/unread_message/",
    UNSAVE =>                  "api/unsave/",
    UNSPOILER =>               "api/unspoiler/",
    UPDATE_SETTINGS =>         "api/v1/subreddit/update_settings/",
    UPDATE_SUBREDDIT_RULE =>   "api/update_subreddit_rule/",
    UPLOAD_IMAGE =>            "r/#subreddit/api/upload_sr_img/",
    USER =>                    "user/#user/",
    USER_ABOUT =>              "user/#user/about/",
    USER_SUBMITTED =>          "user/#user/submitted/",
    USER_COMMENTS =>           "user/#user/comments/",
    USER_BY_FULLNAME =>        "/api/user_data_by_account_ids/",
    USER_FLAIR =>              "r/#subreddit/api/user_flair_v2",
    USERS_NEW =>               "users/new/",
    USERS_POPULAR =>           "users/popular/",
    USERS_SEARCH =>            "users/search/",
    VOTE =>                    "api/vote/",
    WIDGET_CREATE =>           "r/#subreddit/api/widget/",
    WIDGET_LEASE =>            "r/#subreddit/api/widget_image_upload_s3",
    WIDGET_MODIFY =>           "r/#subreddit/api/widget/{widget_id}",
    WIDGET_ORDER =>            "r/#subreddit/api/widget_order/#section/",
    WIDGETS =>                 "r/#subreddit/api/widgets/",
    WIKI_EDIT =>               "r/#subreddit/api/wiki/edit/",
    WIKI_PAGE =>               "r/#subreddit/wiki/#page/",
    WIKI_PAGE_EDITOR =>        "r/#subreddit/api/wiki/alloweditor/#method/",
    WIKI_PAGE_REVISIONS =>     "r/#subreddit/wiki/revisions/#page/",
    WIKI_PAGE_SETTINGS =>      "r/#subreddit/wiki/settings/#page/",
    WIKI_PAGES =>              "r/#subreddit/wiki/pages/",
    WIKI_REVISIONS =>          "r/#subreddit/wiki/revisions/"
}
