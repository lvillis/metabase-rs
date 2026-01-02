#[cfg(feature = "blocking")]
impl crate::client::BlockingClient {
    pub fn health(&self) -> crate::api::BlockingHealthService {
        crate::api::BlockingHealthService::new(self.clone())
    }

    pub fn action(&self) -> crate::api::BlockingActionService {
        crate::api::BlockingActionService::new(self.clone())
    }

    pub fn activity(&self) -> crate::api::BlockingActivityService {
        crate::api::BlockingActivityService::new(self.clone())
    }

    pub fn alert(&self) -> crate::api::BlockingAlertService {
        crate::api::BlockingAlertService::new(self.clone())
    }

    pub fn analytics(&self) -> crate::api::BlockingAnalyticsService {
        crate::api::BlockingAnalyticsService::new(self.clone())
    }

    pub fn api_key(&self) -> crate::api::BlockingApiKeyService {
        crate::api::BlockingApiKeyService::new(self.clone())
    }

    pub fn automagic_dashboards(&self) -> crate::api::BlockingAutomagicDashboardsService {
        crate::api::BlockingAutomagicDashboardsService::new(self.clone())
    }

    pub fn bookmark(&self) -> crate::api::BlockingBookmarkService {
        crate::api::BlockingBookmarkService::new(self.clone())
    }

    pub fn bug_reporting(&self) -> crate::api::BlockingBugReportingService {
        crate::api::BlockingBugReportingService::new(self.clone())
    }

    pub fn cache(&self) -> crate::api::BlockingCacheService {
        crate::api::BlockingCacheService::new(self.clone())
    }

    pub fn card(&self) -> crate::api::BlockingCardService {
        crate::api::BlockingCardService::new(self.clone())
    }

    pub fn cards(&self) -> crate::api::BlockingCardsService {
        crate::api::BlockingCardsService::new(self.clone())
    }

    pub fn channel(&self) -> crate::api::BlockingChannelService {
        crate::api::BlockingChannelService::new(self.clone())
    }

    pub fn cloud_migration(&self) -> crate::api::BlockingCloudMigrationService {
        crate::api::BlockingCloudMigrationService::new(self.clone())
    }

    pub fn collection(&self) -> crate::api::BlockingCollectionService {
        crate::api::BlockingCollectionService::new(self.clone())
    }

    pub fn dashboard(&self) -> crate::api::BlockingDashboardService {
        crate::api::BlockingDashboardService::new(self.clone())
    }

    pub fn database(&self) -> crate::api::BlockingDatabaseService {
        crate::api::BlockingDatabaseService::new(self.clone())
    }

    pub fn dataset(&self) -> crate::api::BlockingDatasetService {
        crate::api::BlockingDatasetService::new(self.clone())
    }

    pub fn eid_translation(&self) -> crate::api::BlockingEidTranslationService {
        crate::api::BlockingEidTranslationService::new(self.clone())
    }

    pub fn email(&self) -> crate::api::BlockingEmailService {
        crate::api::BlockingEmailService::new(self.clone())
    }

    pub fn embed(&self) -> crate::api::BlockingEmbedService {
        crate::api::BlockingEmbedService::new(self.clone())
    }

    pub fn field(&self) -> crate::api::BlockingFieldService {
        crate::api::BlockingFieldService::new(self.clone())
    }

    pub fn geojson(&self) -> crate::api::BlockingGeojsonService {
        crate::api::BlockingGeojsonService::new(self.clone())
    }

    pub fn glossary(&self) -> crate::api::BlockingGlossaryService {
        crate::api::BlockingGlossaryService::new(self.clone())
    }

    pub fn google(&self) -> crate::api::BlockingGoogleService {
        crate::api::BlockingGoogleService::new(self.clone())
    }

    pub fn ldap(&self) -> crate::api::BlockingLdapService {
        crate::api::BlockingLdapService::new(self.clone())
    }

    pub fn logger(&self) -> crate::api::BlockingLoggerService {
        crate::api::BlockingLoggerService::new(self.clone())
    }

    pub fn login_history(&self) -> crate::api::BlockingLoginHistoryService {
        crate::api::BlockingLoginHistoryService::new(self.clone())
    }

    pub fn model_index(&self) -> crate::api::BlockingModelIndexService {
        crate::api::BlockingModelIndexService::new(self.clone())
    }

    pub fn native_query_snippet(&self) -> crate::api::BlockingNativeQuerySnippetService {
        crate::api::BlockingNativeQuerySnippetService::new(self.clone())
    }

    pub fn notification(&self) -> crate::api::BlockingNotificationService {
        crate::api::BlockingNotificationService::new(self.clone())
    }

    pub fn notification_unsubscribe(&self) -> crate::api::BlockingNotificationUnsubscribeService {
        crate::api::BlockingNotificationUnsubscribeService::new(self.clone())
    }

    pub fn notify(&self) -> crate::api::BlockingNotifyService {
        crate::api::BlockingNotifyService::new(self.clone())
    }

    pub fn permissions(&self) -> crate::api::BlockingPermissionsService {
        crate::api::BlockingPermissionsService::new(self.clone())
    }

    pub fn persist(&self) -> crate::api::BlockingPersistService {
        crate::api::BlockingPersistService::new(self.clone())
    }

    pub fn premium_features(&self) -> crate::api::BlockingPremiumFeaturesService {
        crate::api::BlockingPremiumFeaturesService::new(self.clone())
    }

    pub fn preview_embed(&self) -> crate::api::BlockingPreviewEmbedService {
        crate::api::BlockingPreviewEmbedService::new(self.clone())
    }

    pub fn product_feedback(&self) -> crate::api::BlockingProductFeedbackService {
        crate::api::BlockingProductFeedbackService::new(self.clone())
    }

    pub fn public(&self) -> crate::api::BlockingPublicService {
        crate::api::BlockingPublicService::new(self.clone())
    }

    pub fn pulse(&self) -> crate::api::BlockingPulseService {
        crate::api::BlockingPulseService::new(self.clone())
    }

    pub fn pulse_unsubscribe(&self) -> crate::api::BlockingPulseUnsubscribeService {
        crate::api::BlockingPulseUnsubscribeService::new(self.clone())
    }

    pub fn revision(&self) -> crate::api::BlockingRevisionService {
        crate::api::BlockingRevisionService::new(self.clone())
    }

    pub fn search(&self) -> crate::api::BlockingSearchService {
        crate::api::BlockingSearchService::new(self.clone())
    }

    pub fn segment(&self) -> crate::api::BlockingSegmentService {
        crate::api::BlockingSegmentService::new(self.clone())
    }

    pub fn session(&self) -> crate::api::BlockingSessionService {
        crate::api::BlockingSessionService::new(self.clone())
    }

    pub fn setting(&self) -> crate::api::BlockingSettingService {
        crate::api::BlockingSettingService::new(self.clone())
    }

    pub fn setup(&self) -> crate::api::BlockingSetupService {
        crate::api::BlockingSetupService::new(self.clone())
    }

    pub fn slack(&self) -> crate::api::BlockingSlackService {
        crate::api::BlockingSlackService::new(self.clone())
    }

    pub fn table(&self) -> crate::api::BlockingTableService {
        crate::api::BlockingTableService::new(self.clone())
    }

    pub fn task(&self) -> crate::api::BlockingTaskService {
        crate::api::BlockingTaskService::new(self.clone())
    }

    pub fn tiles(&self) -> crate::api::BlockingTilesService {
        crate::api::BlockingTilesService::new(self.clone())
    }

    pub fn timeline(&self) -> crate::api::BlockingTimelineService {
        crate::api::BlockingTimelineService::new(self.clone())
    }

    pub fn timeline_event(&self) -> crate::api::BlockingTimelineEventService {
        crate::api::BlockingTimelineEventService::new(self.clone())
    }

    pub fn upload(&self) -> crate::api::BlockingUploadService {
        crate::api::BlockingUploadService::new(self.clone())
    }

    pub fn user(&self) -> crate::api::BlockingUserService {
        crate::api::BlockingUserService::new(self.clone())
    }

    pub fn user_key_value(&self) -> crate::api::BlockingUserKeyValueService {
        crate::api::BlockingUserKeyValueService::new(self.clone())
    }

    pub fn util(&self) -> crate::api::BlockingUtilService {
        crate::api::BlockingUtilService::new(self.clone())
    }
}
