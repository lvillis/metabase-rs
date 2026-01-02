#[cfg(feature = "async")]
impl crate::client::Client {
    pub fn health(&self) -> crate::api::HealthService {
        crate::api::HealthService::new(self.clone())
    }

    pub fn action(&self) -> crate::api::ActionService {
        crate::api::ActionService::new(self.clone())
    }

    pub fn activity(&self) -> crate::api::ActivityService {
        crate::api::ActivityService::new(self.clone())
    }

    pub fn alert(&self) -> crate::api::AlertService {
        crate::api::AlertService::new(self.clone())
    }

    pub fn analytics(&self) -> crate::api::AnalyticsService {
        crate::api::AnalyticsService::new(self.clone())
    }

    pub fn api_key(&self) -> crate::api::ApiKeyService {
        crate::api::ApiKeyService::new(self.clone())
    }

    pub fn automagic_dashboards(&self) -> crate::api::AutomagicDashboardsService {
        crate::api::AutomagicDashboardsService::new(self.clone())
    }

    pub fn bookmark(&self) -> crate::api::BookmarkService {
        crate::api::BookmarkService::new(self.clone())
    }

    pub fn bug_reporting(&self) -> crate::api::BugReportingService {
        crate::api::BugReportingService::new(self.clone())
    }

    pub fn cache(&self) -> crate::api::CacheService {
        crate::api::CacheService::new(self.clone())
    }

    pub fn card(&self) -> crate::api::CardService {
        crate::api::CardService::new(self.clone())
    }

    pub fn cards(&self) -> crate::api::CardsService {
        crate::api::CardsService::new(self.clone())
    }

    pub fn channel(&self) -> crate::api::ChannelService {
        crate::api::ChannelService::new(self.clone())
    }

    pub fn cloud_migration(&self) -> crate::api::CloudMigrationService {
        crate::api::CloudMigrationService::new(self.clone())
    }

    pub fn collection(&self) -> crate::api::CollectionService {
        crate::api::CollectionService::new(self.clone())
    }

    pub fn dashboard(&self) -> crate::api::DashboardService {
        crate::api::DashboardService::new(self.clone())
    }

    pub fn database(&self) -> crate::api::DatabaseService {
        crate::api::DatabaseService::new(self.clone())
    }

    pub fn dataset(&self) -> crate::api::DatasetService {
        crate::api::DatasetService::new(self.clone())
    }

    pub fn eid_translation(&self) -> crate::api::EidTranslationService {
        crate::api::EidTranslationService::new(self.clone())
    }

    pub fn email(&self) -> crate::api::EmailService {
        crate::api::EmailService::new(self.clone())
    }

    pub fn embed(&self) -> crate::api::EmbedService {
        crate::api::EmbedService::new(self.clone())
    }

    pub fn field(&self) -> crate::api::FieldService {
        crate::api::FieldService::new(self.clone())
    }

    pub fn geojson(&self) -> crate::api::GeojsonService {
        crate::api::GeojsonService::new(self.clone())
    }

    pub fn glossary(&self) -> crate::api::GlossaryService {
        crate::api::GlossaryService::new(self.clone())
    }

    pub fn google(&self) -> crate::api::GoogleService {
        crate::api::GoogleService::new(self.clone())
    }

    pub fn ldap(&self) -> crate::api::LdapService {
        crate::api::LdapService::new(self.clone())
    }

    pub fn logger(&self) -> crate::api::LoggerService {
        crate::api::LoggerService::new(self.clone())
    }

    pub fn login_history(&self) -> crate::api::LoginHistoryService {
        crate::api::LoginHistoryService::new(self.clone())
    }

    pub fn model_index(&self) -> crate::api::ModelIndexService {
        crate::api::ModelIndexService::new(self.clone())
    }

    pub fn native_query_snippet(&self) -> crate::api::NativeQuerySnippetService {
        crate::api::NativeQuerySnippetService::new(self.clone())
    }

    pub fn notification(&self) -> crate::api::NotificationService {
        crate::api::NotificationService::new(self.clone())
    }

    pub fn notification_unsubscribe(&self) -> crate::api::NotificationUnsubscribeService {
        crate::api::NotificationUnsubscribeService::new(self.clone())
    }

    pub fn notify(&self) -> crate::api::NotifyService {
        crate::api::NotifyService::new(self.clone())
    }

    pub fn permissions(&self) -> crate::api::PermissionsService {
        crate::api::PermissionsService::new(self.clone())
    }

    pub fn persist(&self) -> crate::api::PersistService {
        crate::api::PersistService::new(self.clone())
    }

    pub fn premium_features(&self) -> crate::api::PremiumFeaturesService {
        crate::api::PremiumFeaturesService::new(self.clone())
    }

    pub fn preview_embed(&self) -> crate::api::PreviewEmbedService {
        crate::api::PreviewEmbedService::new(self.clone())
    }

    pub fn product_feedback(&self) -> crate::api::ProductFeedbackService {
        crate::api::ProductFeedbackService::new(self.clone())
    }

    pub fn public(&self) -> crate::api::PublicService {
        crate::api::PublicService::new(self.clone())
    }

    pub fn pulse(&self) -> crate::api::PulseService {
        crate::api::PulseService::new(self.clone())
    }

    pub fn pulse_unsubscribe(&self) -> crate::api::PulseUnsubscribeService {
        crate::api::PulseUnsubscribeService::new(self.clone())
    }

    pub fn revision(&self) -> crate::api::RevisionService {
        crate::api::RevisionService::new(self.clone())
    }

    pub fn search(&self) -> crate::api::SearchService {
        crate::api::SearchService::new(self.clone())
    }

    pub fn segment(&self) -> crate::api::SegmentService {
        crate::api::SegmentService::new(self.clone())
    }

    pub fn session(&self) -> crate::api::SessionService {
        crate::api::SessionService::new(self.clone())
    }

    pub fn setting(&self) -> crate::api::SettingService {
        crate::api::SettingService::new(self.clone())
    }

    pub fn setup(&self) -> crate::api::SetupService {
        crate::api::SetupService::new(self.clone())
    }

    pub fn slack(&self) -> crate::api::SlackService {
        crate::api::SlackService::new(self.clone())
    }

    pub fn table(&self) -> crate::api::TableService {
        crate::api::TableService::new(self.clone())
    }

    pub fn task(&self) -> crate::api::TaskService {
        crate::api::TaskService::new(self.clone())
    }

    pub fn tiles(&self) -> crate::api::TilesService {
        crate::api::TilesService::new(self.clone())
    }

    pub fn timeline(&self) -> crate::api::TimelineService {
        crate::api::TimelineService::new(self.clone())
    }

    pub fn timeline_event(&self) -> crate::api::TimelineEventService {
        crate::api::TimelineEventService::new(self.clone())
    }

    pub fn upload(&self) -> crate::api::UploadService {
        crate::api::UploadService::new(self.clone())
    }

    pub fn user(&self) -> crate::api::UserService {
        crate::api::UserService::new(self.clone())
    }

    pub fn user_key_value(&self) -> crate::api::UserKeyValueService {
        crate::api::UserKeyValueService::new(self.clone())
    }

    pub fn util(&self) -> crate::api::UtilService {
        crate::api::UtilService::new(self.clone())
    }
}
