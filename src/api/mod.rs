#![allow(clippy::too_many_arguments)]

mod action;
mod activity;
mod alert;
mod analytics;
mod api_key;
mod automagic_dashboards;
mod bookmark;
mod bug_reporting;
mod cache;
mod card;
mod cards;
mod channel;
mod cloud_migration;
mod collection;
mod dashboard;
mod database;
mod dataset;
mod eid_translation;
mod email;
mod embed;
mod field;
mod geojson;
mod glossary;
mod google;
mod health;
mod ldap;
mod logger;
mod login_history;
mod model_index;
mod native_query_snippet;
mod notification;
mod notification_unsubscribe;
mod notify;
mod permissions;
mod persist;
mod premium_features;
mod preview_embed;
mod product_feedback;
mod public;
mod pulse;
mod pulse_unsubscribe;
mod revision;
mod search;
mod segment;
mod session;
mod setting;
mod setup;
mod slack;
mod table;
mod task;
mod tiles;
mod timeline;
mod timeline_event;
mod upload;
mod user;
mod user_key_value;
mod util;

#[cfg(feature = "blocking")]
pub use health::BlockingHealthService;
#[cfg(feature = "async")]
pub use health::HealthService;

#[cfg(feature = "async")]
pub use action::ActionService;
#[cfg(feature = "blocking")]
pub use action::BlockingActionService;

#[cfg(feature = "async")]
pub use activity::ActivityService;
#[cfg(feature = "blocking")]
pub use activity::BlockingActivityService;

#[cfg(feature = "async")]
pub use alert::AlertService;
#[cfg(feature = "blocking")]
pub use alert::BlockingAlertService;

#[cfg(feature = "async")]
pub use analytics::AnalyticsService;
#[cfg(feature = "blocking")]
pub use analytics::BlockingAnalyticsService;

#[cfg(feature = "async")]
pub use api_key::ApiKeyService;
#[cfg(feature = "blocking")]
pub use api_key::BlockingApiKeyService;

#[cfg(feature = "async")]
pub use automagic_dashboards::AutomagicDashboardsService;
#[cfg(feature = "blocking")]
pub use automagic_dashboards::BlockingAutomagicDashboardsService;

#[cfg(feature = "blocking")]
pub use bookmark::BlockingBookmarkService;
#[cfg(feature = "async")]
pub use bookmark::BookmarkService;

#[cfg(feature = "blocking")]
pub use bug_reporting::BlockingBugReportingService;
#[cfg(feature = "async")]
pub use bug_reporting::BugReportingService;

#[cfg(feature = "blocking")]
pub use cache::BlockingCacheService;
#[cfg(feature = "async")]
pub use cache::CacheService;

#[cfg(feature = "blocking")]
pub use card::BlockingCardService;
#[cfg(feature = "async")]
pub use card::CardService;

#[cfg(feature = "blocking")]
pub use cards::BlockingCardsService;
#[cfg(feature = "async")]
pub use cards::CardsService;

#[cfg(feature = "blocking")]
pub use channel::BlockingChannelService;
#[cfg(feature = "async")]
pub use channel::ChannelService;

#[cfg(feature = "blocking")]
pub use cloud_migration::BlockingCloudMigrationService;
#[cfg(feature = "async")]
pub use cloud_migration::CloudMigrationService;

#[cfg(feature = "blocking")]
pub use collection::BlockingCollectionService;
#[cfg(feature = "async")]
pub use collection::CollectionService;

#[cfg(feature = "blocking")]
pub use dashboard::BlockingDashboardService;
#[cfg(feature = "async")]
pub use dashboard::DashboardService;

#[cfg(feature = "blocking")]
pub use database::BlockingDatabaseService;
#[cfg(feature = "async")]
pub use database::DatabaseService;

#[cfg(feature = "blocking")]
pub use dataset::BlockingDatasetService;
#[cfg(feature = "async")]
pub use dataset::DatasetService;

#[cfg(feature = "blocking")]
pub use eid_translation::BlockingEidTranslationService;
#[cfg(feature = "async")]
pub use eid_translation::EidTranslationService;

#[cfg(feature = "blocking")]
pub use email::BlockingEmailService;
#[cfg(feature = "async")]
pub use email::EmailService;

#[cfg(feature = "blocking")]
pub use embed::BlockingEmbedService;
#[cfg(feature = "async")]
pub use embed::EmbedService;

#[cfg(feature = "blocking")]
pub use field::BlockingFieldService;
#[cfg(feature = "async")]
pub use field::FieldService;

#[cfg(feature = "blocking")]
pub use geojson::BlockingGeojsonService;
#[cfg(feature = "async")]
pub use geojson::GeojsonService;

#[cfg(feature = "blocking")]
pub use glossary::BlockingGlossaryService;
#[cfg(feature = "async")]
pub use glossary::GlossaryService;

#[cfg(feature = "blocking")]
pub use google::BlockingGoogleService;
#[cfg(feature = "async")]
pub use google::GoogleService;

#[cfg(feature = "blocking")]
pub use ldap::BlockingLdapService;
#[cfg(feature = "async")]
pub use ldap::LdapService;

#[cfg(feature = "blocking")]
pub use logger::BlockingLoggerService;
#[cfg(feature = "async")]
pub use logger::LoggerService;

#[cfg(feature = "blocking")]
pub use login_history::BlockingLoginHistoryService;
#[cfg(feature = "async")]
pub use login_history::LoginHistoryService;

#[cfg(feature = "blocking")]
pub use model_index::BlockingModelIndexService;
#[cfg(feature = "async")]
pub use model_index::ModelIndexService;

#[cfg(feature = "blocking")]
pub use native_query_snippet::BlockingNativeQuerySnippetService;
#[cfg(feature = "async")]
pub use native_query_snippet::NativeQuerySnippetService;

#[cfg(feature = "blocking")]
pub use notification::BlockingNotificationService;
#[cfg(feature = "async")]
pub use notification::NotificationService;

#[cfg(feature = "blocking")]
pub use notification_unsubscribe::BlockingNotificationUnsubscribeService;
#[cfg(feature = "async")]
pub use notification_unsubscribe::NotificationUnsubscribeService;

#[cfg(feature = "blocking")]
pub use notify::BlockingNotifyService;
#[cfg(feature = "async")]
pub use notify::NotifyService;

#[cfg(feature = "blocking")]
pub use permissions::BlockingPermissionsService;
#[cfg(feature = "async")]
pub use permissions::PermissionsService;

#[cfg(feature = "blocking")]
pub use persist::BlockingPersistService;
#[cfg(feature = "async")]
pub use persist::PersistService;

#[cfg(feature = "blocking")]
pub use premium_features::BlockingPremiumFeaturesService;
#[cfg(feature = "async")]
pub use premium_features::PremiumFeaturesService;

#[cfg(feature = "blocking")]
pub use preview_embed::BlockingPreviewEmbedService;
#[cfg(feature = "async")]
pub use preview_embed::PreviewEmbedService;

#[cfg(feature = "blocking")]
pub use product_feedback::BlockingProductFeedbackService;
#[cfg(feature = "async")]
pub use product_feedback::ProductFeedbackService;

#[cfg(feature = "blocking")]
pub use public::BlockingPublicService;
#[cfg(feature = "async")]
pub use public::PublicService;

#[cfg(feature = "blocking")]
pub use pulse::BlockingPulseService;
#[cfg(feature = "async")]
pub use pulse::PulseService;

#[cfg(feature = "blocking")]
pub use pulse_unsubscribe::BlockingPulseUnsubscribeService;
#[cfg(feature = "async")]
pub use pulse_unsubscribe::PulseUnsubscribeService;

#[cfg(feature = "blocking")]
pub use revision::BlockingRevisionService;
#[cfg(feature = "async")]
pub use revision::RevisionService;

#[cfg(feature = "blocking")]
pub use search::BlockingSearchService;
#[cfg(feature = "async")]
pub use search::SearchService;

#[cfg(feature = "blocking")]
pub use segment::BlockingSegmentService;
#[cfg(feature = "async")]
pub use segment::SegmentService;

#[cfg(feature = "blocking")]
pub use session::BlockingSessionService;
#[cfg(feature = "async")]
pub use session::SessionService;

#[cfg(feature = "blocking")]
pub use setting::BlockingSettingService;
#[cfg(feature = "async")]
pub use setting::SettingService;

#[cfg(feature = "blocking")]
pub use setup::BlockingSetupService;
#[cfg(feature = "async")]
pub use setup::SetupService;

#[cfg(feature = "blocking")]
pub use slack::BlockingSlackService;
#[cfg(feature = "async")]
pub use slack::SlackService;

#[cfg(feature = "blocking")]
pub use table::BlockingTableService;
#[cfg(feature = "async")]
pub use table::TableService;

#[cfg(feature = "blocking")]
pub use task::BlockingTaskService;
#[cfg(feature = "async")]
pub use task::TaskService;

#[cfg(feature = "blocking")]
pub use tiles::BlockingTilesService;
#[cfg(feature = "async")]
pub use tiles::TilesService;

#[cfg(feature = "blocking")]
pub use timeline::BlockingTimelineService;
#[cfg(feature = "async")]
pub use timeline::TimelineService;

#[cfg(feature = "blocking")]
pub use timeline_event::BlockingTimelineEventService;
#[cfg(feature = "async")]
pub use timeline_event::TimelineEventService;

#[cfg(feature = "blocking")]
pub use upload::BlockingUploadService;
#[cfg(feature = "async")]
pub use upload::UploadService;

#[cfg(feature = "blocking")]
pub use user::BlockingUserService;
#[cfg(feature = "async")]
pub use user::UserService;

#[cfg(feature = "blocking")]
pub use user_key_value::BlockingUserKeyValueService;
#[cfg(feature = "async")]
pub use user_key_value::UserKeyValueService;

#[cfg(feature = "blocking")]
pub use util::BlockingUtilService;
#[cfg(feature = "async")]
pub use util::UtilService;
