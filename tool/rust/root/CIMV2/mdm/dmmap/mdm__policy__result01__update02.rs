// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_Update02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_Update02 {

/// 
    #[serde(rename = "ActiveHoursEnd")]
    pub active_hours_end: Option<i32>,

/// 
    #[serde(rename = "ActiveHoursMaxRange")]
    pub active_hours_max_range: Option<i32>,

/// 
    #[serde(rename = "ActiveHoursStart")]
    pub active_hours_start: Option<i32>,

/// 
    #[serde(rename = "AllowAutoUpdate")]
    pub allow_auto_update: Option<i32>,

/// 
    #[serde(rename = "AllowAutoWindowsUpdateDownloadOverMeteredNetwork")]
    pub allow_auto_windows_update_download_over_metered_network: Option<i32>,

/// 
    #[serde(rename = "AllowMUUpdateService")]
    pub allow_muupdate_service: Option<i32>,

/// 
    #[serde(rename = "AllowNonMicrosoftSignedUpdate")]
    pub allow_non_microsoft_signed_update: Option<i32>,

/// 
    #[serde(rename = "AllowOptionalContent")]
    pub allow_optional_content: Option<i32>,

/// 
    #[serde(rename = "AllowTemporaryEnterpriseFeatureControl")]
    pub allow_temporary_enterprise_feature_control: Option<i32>,

/// 
    #[serde(rename = "AllowUpdateService")]
    pub allow_update_service: Option<i32>,

/// 
    #[serde(rename = "AlwaysAutoRebootAtScheduledTimeMinutes")]
    pub always_auto_reboot_at_scheduled_time_minutes: Option<i32>,

/// 
    #[serde(rename = "AutomaticMaintenanceWakeUp")]
    pub automatic_maintenance_wake_up: Option<i32>,

/// 
    #[serde(rename = "AutoRestartDeadlinePeriodInDays")]
    pub auto_restart_deadline_period_in_days: Option<i32>,

/// 
    #[serde(rename = "AutoRestartDeadlinePeriodInDaysForFeatureUpdates")]
    pub auto_restart_deadline_period_in_days_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "AutoRestartNotificationSchedule")]
    pub auto_restart_notification_schedule: Option<i32>,

/// 
    #[serde(rename = "AutoRestartRequiredNotificationDismissal")]
    pub auto_restart_required_notification_dismissal: Option<i32>,

/// 
    #[serde(rename = "BranchReadinessLevel")]
    pub branch_readiness_level: Option<i32>,

/// 
    #[serde(rename = "ConfigureDeadlineForFeatureUpdates")]
    pub configure_deadline_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "ConfigureDeadlineForQualityUpdates")]
    pub configure_deadline_for_quality_updates: Option<i32>,

/// 
    #[serde(rename = "ConfigureDeadlineGracePeriod")]
    pub configure_deadline_grace_period: Option<i32>,

/// 
    #[serde(rename = "ConfigureDeadlineGracePeriodForFeatureUpdates")]
    pub configure_deadline_grace_period_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "ConfigureDeadlineNoAutoReboot")]
    pub configure_deadline_no_auto_reboot: Option<i32>,

/// 
    #[serde(rename = "ConfigureFeatureUpdateUninstallPeriod")]
    pub configure_feature_update_uninstall_period: Option<i32>,

/// 
    #[serde(rename = "DeferFeatureUpdatesPeriodInDays")]
    pub defer_feature_updates_period_in_days: Option<i32>,

/// 
    #[serde(rename = "DeferQualityUpdatesPeriodInDays")]
    pub defer_quality_updates_period_in_days: Option<i32>,

/// 
    #[serde(rename = "DeferUpdatePeriod")]
    pub defer_update_period: Option<i32>,

/// 
    #[serde(rename = "DeferUpgradePeriod")]
    pub defer_upgrade_period: Option<i32>,

/// 
    #[serde(rename = "DetectionFrequency")]
    pub detection_frequency: Option<i32>,

/// 
    #[serde(rename = "DisableDualScan")]
    pub disable_dual_scan: Option<i32>,

/// 
    #[serde(rename = "DisableWUfBSafeguards")]
    pub disable_wuf_bsafeguards: Option<i32>,

/// 
    #[serde(rename = "DoNotEnforceEnterpriseTLSCertPinningForUpdateDetection")]
    pub do_not_enforce_enterprise_tlscert_pinning_for_update_detection: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartDeadline")]
    pub engaged_restart_deadline: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartDeadlineForFeatureUpdates")]
    pub engaged_restart_deadline_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartSnoozeSchedule")]
    pub engaged_restart_snooze_schedule: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartSnoozeScheduleForFeatureUpdates")]
    pub engaged_restart_snooze_schedule_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartTransitionSchedule")]
    pub engaged_restart_transition_schedule: Option<i32>,

/// 
    #[serde(rename = "EngagedRestartTransitionScheduleForFeatureUpdates")]
    pub engaged_restart_transition_schedule_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "ExcludeWUDriversInQualityUpdate")]
    pub exclude_wudrivers_in_quality_update: Option<i32>,

/// 
    #[serde(rename = "FillEmptyContentUrls")]
    pub fill_empty_content_urls: Option<i32>,

/// 
    #[serde(rename = "IgnoreMOAppDownloadLimit")]
    pub ignore_moapp_download_limit: Option<i32>,

/// 
    #[serde(rename = "IgnoreMOUpdateDownloadLimit")]
    pub ignore_moupdate_download_limit: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ManagePreviewBuilds")]
    pub manage_preview_builds: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PauseDeferrals")]
    pub pause_deferrals: Option<i32>,

/// 
    #[serde(rename = "PauseFeatureUpdates")]
    pub pause_feature_updates: Option<i32>,

/// 
    #[serde(rename = "PauseFeatureUpdatesStartTime")]
    pub pause_feature_updates_start_time: Option<String>,

/// 
    #[serde(rename = "PauseQualityUpdates")]
    pub pause_quality_updates: Option<i32>,

/// 
    #[serde(rename = "PauseQualityUpdatesStartTime")]
    pub pause_quality_updates_start_time: Option<String>,

/// 
    #[serde(rename = "PhoneUpdateRestrictions")]
    pub phone_update_restrictions: Option<i32>,

/// 
    #[serde(rename = "ProductVersion")]
    pub product_version: Option<String>,

/// 
    #[serde(rename = "RequireDeferUpgrade")]
    pub require_defer_upgrade: Option<i32>,

/// 
    #[serde(rename = "RequireUpdateApproval")]
    pub require_update_approval: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallDay")]
    pub scheduled_install_day: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallEveryWeek")]
    pub scheduled_install_every_week: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallFirstWeek")]
    pub scheduled_install_first_week: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallFourthWeek")]
    pub scheduled_install_fourth_week: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallSecondWeek")]
    pub scheduled_install_second_week: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallThirdWeek")]
    pub scheduled_install_third_week: Option<i32>,

/// 
    #[serde(rename = "ScheduledInstallTime")]
    pub scheduled_install_time: Option<i32>,

/// 
    #[serde(rename = "ScheduleImminentRestartWarning")]
    pub schedule_imminent_restart_warning: Option<i32>,

/// 
    #[serde(rename = "ScheduleRestartWarning")]
    pub schedule_restart_warning: Option<i32>,

/// 
    #[serde(rename = "SetAutoRestartNotificationDisable")]
    pub set_auto_restart_notification_disable: Option<i32>,

/// 
    #[serde(rename = "SetDisablePauseUXAccess")]
    pub set_disable_pause_uxaccess: Option<i32>,

/// 
    #[serde(rename = "SetDisableUXWUAccess")]
    pub set_disable_uxwuaccess: Option<i32>,

/// 
    #[serde(rename = "SetEDURestart")]
    pub set_edurestart: Option<i32>,

/// 
    #[serde(rename = "SetPolicyDrivenUpdateSourceForDriverUpdates")]
    pub set_policy_driven_update_source_for_driver_updates: Option<i32>,

/// 
    #[serde(rename = "SetPolicyDrivenUpdateSourceForFeatureUpdates")]
    pub set_policy_driven_update_source_for_feature_updates: Option<i32>,

/// 
    #[serde(rename = "SetPolicyDrivenUpdateSourceForOtherUpdates")]
    pub set_policy_driven_update_source_for_other_updates: Option<i32>,

/// 
    #[serde(rename = "SetPolicyDrivenUpdateSourceForQualityUpdates")]
    pub set_policy_driven_update_source_for_quality_updates: Option<i32>,

/// 
    #[serde(rename = "SetProxyBehaviorForUpdateDetection")]
    pub set_proxy_behavior_for_update_detection: Option<i32>,

/// 
    #[serde(rename = "TargetReleaseVersion")]
    pub target_release_version: Option<String>,

/// 
    #[serde(rename = "UpdateNotificationLevel")]
    pub update_notification_level: Option<i32>,

/// 
    #[serde(rename = "UpdateServiceUrl")]
    pub update_service_url: Option<String>,

/// 
    #[serde(rename = "UpdateServiceUrlAlternate")]
    pub update_service_url_alternate: Option<String>,
}

impl MDM_Policy_Result01_Update02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active_hours_end: None,
            active_hours_max_range: None,
            active_hours_start: None,
            allow_auto_update: None,
            allow_auto_windows_update_download_over_metered_network: None,
            allow_muupdate_service: None,
            allow_non_microsoft_signed_update: None,
            allow_optional_content: None,
            allow_temporary_enterprise_feature_control: None,
            allow_update_service: None,
            always_auto_reboot_at_scheduled_time_minutes: None,
            automatic_maintenance_wake_up: None,
            auto_restart_deadline_period_in_days: None,
            auto_restart_deadline_period_in_days_for_feature_updates: None,
            auto_restart_notification_schedule: None,
            auto_restart_required_notification_dismissal: None,
            branch_readiness_level: None,
            configure_deadline_for_feature_updates: None,
            configure_deadline_for_quality_updates: None,
            configure_deadline_grace_period: None,
            configure_deadline_grace_period_for_feature_updates: None,
            configure_deadline_no_auto_reboot: None,
            configure_feature_update_uninstall_period: None,
            defer_feature_updates_period_in_days: None,
            defer_quality_updates_period_in_days: None,
            defer_update_period: None,
            defer_upgrade_period: None,
            detection_frequency: None,
            disable_dual_scan: None,
            disable_wuf_bsafeguards: None,
            do_not_enforce_enterprise_tlscert_pinning_for_update_detection: None,
            engaged_restart_deadline: None,
            engaged_restart_deadline_for_feature_updates: None,
            engaged_restart_snooze_schedule: None,
            engaged_restart_snooze_schedule_for_feature_updates: None,
            engaged_restart_transition_schedule: None,
            engaged_restart_transition_schedule_for_feature_updates: None,
            exclude_wudrivers_in_quality_update: None,
            fill_empty_content_urls: None,
            ignore_moapp_download_limit: None,
            ignore_moupdate_download_limit: None,
            instance_id: None,
            manage_preview_builds: None,
            parent_id: None,
            pause_deferrals: None,
            pause_feature_updates: None,
            pause_feature_updates_start_time: None,
            pause_quality_updates: None,
            pause_quality_updates_start_time: None,
            phone_update_restrictions: None,
            product_version: None,
            require_defer_upgrade: None,
            require_update_approval: None,
            scheduled_install_day: None,
            scheduled_install_every_week: None,
            scheduled_install_first_week: None,
            scheduled_install_fourth_week: None,
            scheduled_install_second_week: None,
            scheduled_install_third_week: None,
            scheduled_install_time: None,
            schedule_imminent_restart_warning: None,
            schedule_restart_warning: None,
            set_auto_restart_notification_disable: None,
            set_disable_pause_uxaccess: None,
            set_disable_uxwuaccess: None,
            set_edurestart: None,
            set_policy_driven_update_source_for_driver_updates: None,
            set_policy_driven_update_source_for_feature_updates: None,
            set_policy_driven_update_source_for_other_updates: None,
            set_policy_driven_update_source_for_quality_updates: None,
            set_proxy_behavior_for_update_detection: None,
            target_release_version: None,
            update_notification_level: None,
            update_service_url: None,
            update_service_url_alternate: None,
        }
    }


    /// Sets the value of ActiveHoursEnd
    pub fn set_active_hours_end(&mut self, value: i32) {
        self.active_hours_end = Some(value);
    }

    /// Gets the value of ActiveHoursEnd
    pub fn get_active_hours_end(&self) -> Option<&i32> {
        self.active_hours_end.as_ref()
    }

    /// Sets the value of ActiveHoursMaxRange
    pub fn set_active_hours_max_range(&mut self, value: i32) {
        self.active_hours_max_range = Some(value);
    }

    /// Gets the value of ActiveHoursMaxRange
    pub fn get_active_hours_max_range(&self) -> Option<&i32> {
        self.active_hours_max_range.as_ref()
    }

    /// Sets the value of ActiveHoursStart
    pub fn set_active_hours_start(&mut self, value: i32) {
        self.active_hours_start = Some(value);
    }

    /// Gets the value of ActiveHoursStart
    pub fn get_active_hours_start(&self) -> Option<&i32> {
        self.active_hours_start.as_ref()
    }

    /// Sets the value of AllowAutoUpdate
    pub fn set_allow_auto_update(&mut self, value: i32) {
        self.allow_auto_update = Some(value);
    }

    /// Gets the value of AllowAutoUpdate
    pub fn get_allow_auto_update(&self) -> Option<&i32> {
        self.allow_auto_update.as_ref()
    }

    /// Sets the value of AllowAutoWindowsUpdateDownloadOverMeteredNetwork
    pub fn set_allow_auto_windows_update_download_over_metered_network(&mut self, value: i32) {
        self.allow_auto_windows_update_download_over_metered_network = Some(value);
    }

    /// Gets the value of AllowAutoWindowsUpdateDownloadOverMeteredNetwork
    pub fn get_allow_auto_windows_update_download_over_metered_network(&self) -> Option<&i32> {
        self.allow_auto_windows_update_download_over_metered_network.as_ref()
    }

    /// Sets the value of AllowMUUpdateService
    pub fn set_allow_muupdate_service(&mut self, value: i32) {
        self.allow_muupdate_service = Some(value);
    }

    /// Gets the value of AllowMUUpdateService
    pub fn get_allow_muupdate_service(&self) -> Option<&i32> {
        self.allow_muupdate_service.as_ref()
    }

    /// Sets the value of AllowNonMicrosoftSignedUpdate
    pub fn set_allow_non_microsoft_signed_update(&mut self, value: i32) {
        self.allow_non_microsoft_signed_update = Some(value);
    }

    /// Gets the value of AllowNonMicrosoftSignedUpdate
    pub fn get_allow_non_microsoft_signed_update(&self) -> Option<&i32> {
        self.allow_non_microsoft_signed_update.as_ref()
    }

    /// Sets the value of AllowOptionalContent
    pub fn set_allow_optional_content(&mut self, value: i32) {
        self.allow_optional_content = Some(value);
    }

    /// Gets the value of AllowOptionalContent
    pub fn get_allow_optional_content(&self) -> Option<&i32> {
        self.allow_optional_content.as_ref()
    }

    /// Sets the value of AllowTemporaryEnterpriseFeatureControl
    pub fn set_allow_temporary_enterprise_feature_control(&mut self, value: i32) {
        self.allow_temporary_enterprise_feature_control = Some(value);
    }

    /// Gets the value of AllowTemporaryEnterpriseFeatureControl
    pub fn get_allow_temporary_enterprise_feature_control(&self) -> Option<&i32> {
        self.allow_temporary_enterprise_feature_control.as_ref()
    }

    /// Sets the value of AllowUpdateService
    pub fn set_allow_update_service(&mut self, value: i32) {
        self.allow_update_service = Some(value);
    }

    /// Gets the value of AllowUpdateService
    pub fn get_allow_update_service(&self) -> Option<&i32> {
        self.allow_update_service.as_ref()
    }

    /// Sets the value of AlwaysAutoRebootAtScheduledTimeMinutes
    pub fn set_always_auto_reboot_at_scheduled_time_minutes(&mut self, value: i32) {
        self.always_auto_reboot_at_scheduled_time_minutes = Some(value);
    }

    /// Gets the value of AlwaysAutoRebootAtScheduledTimeMinutes
    pub fn get_always_auto_reboot_at_scheduled_time_minutes(&self) -> Option<&i32> {
        self.always_auto_reboot_at_scheduled_time_minutes.as_ref()
    }

    /// Sets the value of AutomaticMaintenanceWakeUp
    pub fn set_automatic_maintenance_wake_up(&mut self, value: i32) {
        self.automatic_maintenance_wake_up = Some(value);
    }

    /// Gets the value of AutomaticMaintenanceWakeUp
    pub fn get_automatic_maintenance_wake_up(&self) -> Option<&i32> {
        self.automatic_maintenance_wake_up.as_ref()
    }

    /// Sets the value of AutoRestartDeadlinePeriodInDays
    pub fn set_auto_restart_deadline_period_in_days(&mut self, value: i32) {
        self.auto_restart_deadline_period_in_days = Some(value);
    }

    /// Gets the value of AutoRestartDeadlinePeriodInDays
    pub fn get_auto_restart_deadline_period_in_days(&self) -> Option<&i32> {
        self.auto_restart_deadline_period_in_days.as_ref()
    }

    /// Sets the value of AutoRestartDeadlinePeriodInDaysForFeatureUpdates
    pub fn set_auto_restart_deadline_period_in_days_for_feature_updates(&mut self, value: i32) {
        self.auto_restart_deadline_period_in_days_for_feature_updates = Some(value);
    }

    /// Gets the value of AutoRestartDeadlinePeriodInDaysForFeatureUpdates
    pub fn get_auto_restart_deadline_period_in_days_for_feature_updates(&self) -> Option<&i32> {
        self.auto_restart_deadline_period_in_days_for_feature_updates.as_ref()
    }

    /// Sets the value of AutoRestartNotificationSchedule
    pub fn set_auto_restart_notification_schedule(&mut self, value: i32) {
        self.auto_restart_notification_schedule = Some(value);
    }

    /// Gets the value of AutoRestartNotificationSchedule
    pub fn get_auto_restart_notification_schedule(&self) -> Option<&i32> {
        self.auto_restart_notification_schedule.as_ref()
    }

    /// Sets the value of AutoRestartRequiredNotificationDismissal
    pub fn set_auto_restart_required_notification_dismissal(&mut self, value: i32) {
        self.auto_restart_required_notification_dismissal = Some(value);
    }

    /// Gets the value of AutoRestartRequiredNotificationDismissal
    pub fn get_auto_restart_required_notification_dismissal(&self) -> Option<&i32> {
        self.auto_restart_required_notification_dismissal.as_ref()
    }

    /// Sets the value of BranchReadinessLevel
    pub fn set_branch_readiness_level(&mut self, value: i32) {
        self.branch_readiness_level = Some(value);
    }

    /// Gets the value of BranchReadinessLevel
    pub fn get_branch_readiness_level(&self) -> Option<&i32> {
        self.branch_readiness_level.as_ref()
    }

    /// Sets the value of ConfigureDeadlineForFeatureUpdates
    pub fn set_configure_deadline_for_feature_updates(&mut self, value: i32) {
        self.configure_deadline_for_feature_updates = Some(value);
    }

    /// Gets the value of ConfigureDeadlineForFeatureUpdates
    pub fn get_configure_deadline_for_feature_updates(&self) -> Option<&i32> {
        self.configure_deadline_for_feature_updates.as_ref()
    }

    /// Sets the value of ConfigureDeadlineForQualityUpdates
    pub fn set_configure_deadline_for_quality_updates(&mut self, value: i32) {
        self.configure_deadline_for_quality_updates = Some(value);
    }

    /// Gets the value of ConfigureDeadlineForQualityUpdates
    pub fn get_configure_deadline_for_quality_updates(&self) -> Option<&i32> {
        self.configure_deadline_for_quality_updates.as_ref()
    }

    /// Sets the value of ConfigureDeadlineGracePeriod
    pub fn set_configure_deadline_grace_period(&mut self, value: i32) {
        self.configure_deadline_grace_period = Some(value);
    }

    /// Gets the value of ConfigureDeadlineGracePeriod
    pub fn get_configure_deadline_grace_period(&self) -> Option<&i32> {
        self.configure_deadline_grace_period.as_ref()
    }

    /// Sets the value of ConfigureDeadlineGracePeriodForFeatureUpdates
    pub fn set_configure_deadline_grace_period_for_feature_updates(&mut self, value: i32) {
        self.configure_deadline_grace_period_for_feature_updates = Some(value);
    }

    /// Gets the value of ConfigureDeadlineGracePeriodForFeatureUpdates
    pub fn get_configure_deadline_grace_period_for_feature_updates(&self) -> Option<&i32> {
        self.configure_deadline_grace_period_for_feature_updates.as_ref()
    }

    /// Sets the value of ConfigureDeadlineNoAutoReboot
    pub fn set_configure_deadline_no_auto_reboot(&mut self, value: i32) {
        self.configure_deadline_no_auto_reboot = Some(value);
    }

    /// Gets the value of ConfigureDeadlineNoAutoReboot
    pub fn get_configure_deadline_no_auto_reboot(&self) -> Option<&i32> {
        self.configure_deadline_no_auto_reboot.as_ref()
    }

    /// Sets the value of ConfigureFeatureUpdateUninstallPeriod
    pub fn set_configure_feature_update_uninstall_period(&mut self, value: i32) {
        self.configure_feature_update_uninstall_period = Some(value);
    }

    /// Gets the value of ConfigureFeatureUpdateUninstallPeriod
    pub fn get_configure_feature_update_uninstall_period(&self) -> Option<&i32> {
        self.configure_feature_update_uninstall_period.as_ref()
    }

    /// Sets the value of DeferFeatureUpdatesPeriodInDays
    pub fn set_defer_feature_updates_period_in_days(&mut self, value: i32) {
        self.defer_feature_updates_period_in_days = Some(value);
    }

    /// Gets the value of DeferFeatureUpdatesPeriodInDays
    pub fn get_defer_feature_updates_period_in_days(&self) -> Option<&i32> {
        self.defer_feature_updates_period_in_days.as_ref()
    }

    /// Sets the value of DeferQualityUpdatesPeriodInDays
    pub fn set_defer_quality_updates_period_in_days(&mut self, value: i32) {
        self.defer_quality_updates_period_in_days = Some(value);
    }

    /// Gets the value of DeferQualityUpdatesPeriodInDays
    pub fn get_defer_quality_updates_period_in_days(&self) -> Option<&i32> {
        self.defer_quality_updates_period_in_days.as_ref()
    }

    /// Sets the value of DeferUpdatePeriod
    pub fn set_defer_update_period(&mut self, value: i32) {
        self.defer_update_period = Some(value);
    }

    /// Gets the value of DeferUpdatePeriod
    pub fn get_defer_update_period(&self) -> Option<&i32> {
        self.defer_update_period.as_ref()
    }

    /// Sets the value of DeferUpgradePeriod
    pub fn set_defer_upgrade_period(&mut self, value: i32) {
        self.defer_upgrade_period = Some(value);
    }

    /// Gets the value of DeferUpgradePeriod
    pub fn get_defer_upgrade_period(&self) -> Option<&i32> {
        self.defer_upgrade_period.as_ref()
    }

    /// Sets the value of DetectionFrequency
    pub fn set_detection_frequency(&mut self, value: i32) {
        self.detection_frequency = Some(value);
    }

    /// Gets the value of DetectionFrequency
    pub fn get_detection_frequency(&self) -> Option<&i32> {
        self.detection_frequency.as_ref()
    }

    /// Sets the value of DisableDualScan
    pub fn set_disable_dual_scan(&mut self, value: i32) {
        self.disable_dual_scan = Some(value);
    }

    /// Gets the value of DisableDualScan
    pub fn get_disable_dual_scan(&self) -> Option<&i32> {
        self.disable_dual_scan.as_ref()
    }

    /// Sets the value of DisableWUfBSafeguards
    pub fn set_disable_wuf_bsafeguards(&mut self, value: i32) {
        self.disable_wuf_bsafeguards = Some(value);
    }

    /// Gets the value of DisableWUfBSafeguards
    pub fn get_disable_wuf_bsafeguards(&self) -> Option<&i32> {
        self.disable_wuf_bsafeguards.as_ref()
    }

    /// Sets the value of DoNotEnforceEnterpriseTLSCertPinningForUpdateDetection
    pub fn set_do_not_enforce_enterprise_tlscert_pinning_for_update_detection(&mut self, value: i32) {
        self.do_not_enforce_enterprise_tlscert_pinning_for_update_detection = Some(value);
    }

    /// Gets the value of DoNotEnforceEnterpriseTLSCertPinningForUpdateDetection
    pub fn get_do_not_enforce_enterprise_tlscert_pinning_for_update_detection(&self) -> Option<&i32> {
        self.do_not_enforce_enterprise_tlscert_pinning_for_update_detection.as_ref()
    }

    /// Sets the value of EngagedRestartDeadline
    pub fn set_engaged_restart_deadline(&mut self, value: i32) {
        self.engaged_restart_deadline = Some(value);
    }

    /// Gets the value of EngagedRestartDeadline
    pub fn get_engaged_restart_deadline(&self) -> Option<&i32> {
        self.engaged_restart_deadline.as_ref()
    }

    /// Sets the value of EngagedRestartDeadlineForFeatureUpdates
    pub fn set_engaged_restart_deadline_for_feature_updates(&mut self, value: i32) {
        self.engaged_restart_deadline_for_feature_updates = Some(value);
    }

    /// Gets the value of EngagedRestartDeadlineForFeatureUpdates
    pub fn get_engaged_restart_deadline_for_feature_updates(&self) -> Option<&i32> {
        self.engaged_restart_deadline_for_feature_updates.as_ref()
    }

    /// Sets the value of EngagedRestartSnoozeSchedule
    pub fn set_engaged_restart_snooze_schedule(&mut self, value: i32) {
        self.engaged_restart_snooze_schedule = Some(value);
    }

    /// Gets the value of EngagedRestartSnoozeSchedule
    pub fn get_engaged_restart_snooze_schedule(&self) -> Option<&i32> {
        self.engaged_restart_snooze_schedule.as_ref()
    }

    /// Sets the value of EngagedRestartSnoozeScheduleForFeatureUpdates
    pub fn set_engaged_restart_snooze_schedule_for_feature_updates(&mut self, value: i32) {
        self.engaged_restart_snooze_schedule_for_feature_updates = Some(value);
    }

    /// Gets the value of EngagedRestartSnoozeScheduleForFeatureUpdates
    pub fn get_engaged_restart_snooze_schedule_for_feature_updates(&self) -> Option<&i32> {
        self.engaged_restart_snooze_schedule_for_feature_updates.as_ref()
    }

    /// Sets the value of EngagedRestartTransitionSchedule
    pub fn set_engaged_restart_transition_schedule(&mut self, value: i32) {
        self.engaged_restart_transition_schedule = Some(value);
    }

    /// Gets the value of EngagedRestartTransitionSchedule
    pub fn get_engaged_restart_transition_schedule(&self) -> Option<&i32> {
        self.engaged_restart_transition_schedule.as_ref()
    }

    /// Sets the value of EngagedRestartTransitionScheduleForFeatureUpdates
    pub fn set_engaged_restart_transition_schedule_for_feature_updates(&mut self, value: i32) {
        self.engaged_restart_transition_schedule_for_feature_updates = Some(value);
    }

    /// Gets the value of EngagedRestartTransitionScheduleForFeatureUpdates
    pub fn get_engaged_restart_transition_schedule_for_feature_updates(&self) -> Option<&i32> {
        self.engaged_restart_transition_schedule_for_feature_updates.as_ref()
    }

    /// Sets the value of ExcludeWUDriversInQualityUpdate
    pub fn set_exclude_wudrivers_in_quality_update(&mut self, value: i32) {
        self.exclude_wudrivers_in_quality_update = Some(value);
    }

    /// Gets the value of ExcludeWUDriversInQualityUpdate
    pub fn get_exclude_wudrivers_in_quality_update(&self) -> Option<&i32> {
        self.exclude_wudrivers_in_quality_update.as_ref()
    }

    /// Sets the value of FillEmptyContentUrls
    pub fn set_fill_empty_content_urls(&mut self, value: i32) {
        self.fill_empty_content_urls = Some(value);
    }

    /// Gets the value of FillEmptyContentUrls
    pub fn get_fill_empty_content_urls(&self) -> Option<&i32> {
        self.fill_empty_content_urls.as_ref()
    }

    /// Sets the value of IgnoreMOAppDownloadLimit
    pub fn set_ignore_moapp_download_limit(&mut self, value: i32) {
        self.ignore_moapp_download_limit = Some(value);
    }

    /// Gets the value of IgnoreMOAppDownloadLimit
    pub fn get_ignore_moapp_download_limit(&self) -> Option<&i32> {
        self.ignore_moapp_download_limit.as_ref()
    }

    /// Sets the value of IgnoreMOUpdateDownloadLimit
    pub fn set_ignore_moupdate_download_limit(&mut self, value: i32) {
        self.ignore_moupdate_download_limit = Some(value);
    }

    /// Gets the value of IgnoreMOUpdateDownloadLimit
    pub fn get_ignore_moupdate_download_limit(&self) -> Option<&i32> {
        self.ignore_moupdate_download_limit.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ManagePreviewBuilds
    pub fn set_manage_preview_builds(&mut self, value: i32) {
        self.manage_preview_builds = Some(value);
    }

    /// Gets the value of ManagePreviewBuilds
    pub fn get_manage_preview_builds(&self) -> Option<&i32> {
        self.manage_preview_builds.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PauseDeferrals
    pub fn set_pause_deferrals(&mut self, value: i32) {
        self.pause_deferrals = Some(value);
    }

    /// Gets the value of PauseDeferrals
    pub fn get_pause_deferrals(&self) -> Option<&i32> {
        self.pause_deferrals.as_ref()
    }

    /// Sets the value of PauseFeatureUpdates
    pub fn set_pause_feature_updates(&mut self, value: i32) {
        self.pause_feature_updates = Some(value);
    }

    /// Gets the value of PauseFeatureUpdates
    pub fn get_pause_feature_updates(&self) -> Option<&i32> {
        self.pause_feature_updates.as_ref()
    }

    /// Sets the value of PauseFeatureUpdatesStartTime
    pub fn set_pause_feature_updates_start_time(&mut self, value: String) {
        self.pause_feature_updates_start_time = Some(value);
    }

    /// Gets the value of PauseFeatureUpdatesStartTime
    pub fn get_pause_feature_updates_start_time(&self) -> Option<&String> {
        self.pause_feature_updates_start_time.as_ref()
    }

    /// Sets the value of PauseQualityUpdates
    pub fn set_pause_quality_updates(&mut self, value: i32) {
        self.pause_quality_updates = Some(value);
    }

    /// Gets the value of PauseQualityUpdates
    pub fn get_pause_quality_updates(&self) -> Option<&i32> {
        self.pause_quality_updates.as_ref()
    }

    /// Sets the value of PauseQualityUpdatesStartTime
    pub fn set_pause_quality_updates_start_time(&mut self, value: String) {
        self.pause_quality_updates_start_time = Some(value);
    }

    /// Gets the value of PauseQualityUpdatesStartTime
    pub fn get_pause_quality_updates_start_time(&self) -> Option<&String> {
        self.pause_quality_updates_start_time.as_ref()
    }

    /// Sets the value of PhoneUpdateRestrictions
    pub fn set_phone_update_restrictions(&mut self, value: i32) {
        self.phone_update_restrictions = Some(value);
    }

    /// Gets the value of PhoneUpdateRestrictions
    pub fn get_phone_update_restrictions(&self) -> Option<&i32> {
        self.phone_update_restrictions.as_ref()
    }

    /// Sets the value of ProductVersion
    pub fn set_product_version(&mut self, value: String) {
        self.product_version = Some(value);
    }

    /// Gets the value of ProductVersion
    pub fn get_product_version(&self) -> Option<&String> {
        self.product_version.as_ref()
    }

    /// Sets the value of RequireDeferUpgrade
    pub fn set_require_defer_upgrade(&mut self, value: i32) {
        self.require_defer_upgrade = Some(value);
    }

    /// Gets the value of RequireDeferUpgrade
    pub fn get_require_defer_upgrade(&self) -> Option<&i32> {
        self.require_defer_upgrade.as_ref()
    }

    /// Sets the value of RequireUpdateApproval
    pub fn set_require_update_approval(&mut self, value: i32) {
        self.require_update_approval = Some(value);
    }

    /// Gets the value of RequireUpdateApproval
    pub fn get_require_update_approval(&self) -> Option<&i32> {
        self.require_update_approval.as_ref()
    }

    /// Sets the value of ScheduledInstallDay
    pub fn set_scheduled_install_day(&mut self, value: i32) {
        self.scheduled_install_day = Some(value);
    }

    /// Gets the value of ScheduledInstallDay
    pub fn get_scheduled_install_day(&self) -> Option<&i32> {
        self.scheduled_install_day.as_ref()
    }

    /// Sets the value of ScheduledInstallEveryWeek
    pub fn set_scheduled_install_every_week(&mut self, value: i32) {
        self.scheduled_install_every_week = Some(value);
    }

    /// Gets the value of ScheduledInstallEveryWeek
    pub fn get_scheduled_install_every_week(&self) -> Option<&i32> {
        self.scheduled_install_every_week.as_ref()
    }

    /// Sets the value of ScheduledInstallFirstWeek
    pub fn set_scheduled_install_first_week(&mut self, value: i32) {
        self.scheduled_install_first_week = Some(value);
    }

    /// Gets the value of ScheduledInstallFirstWeek
    pub fn get_scheduled_install_first_week(&self) -> Option<&i32> {
        self.scheduled_install_first_week.as_ref()
    }

    /// Sets the value of ScheduledInstallFourthWeek
    pub fn set_scheduled_install_fourth_week(&mut self, value: i32) {
        self.scheduled_install_fourth_week = Some(value);
    }

    /// Gets the value of ScheduledInstallFourthWeek
    pub fn get_scheduled_install_fourth_week(&self) -> Option<&i32> {
        self.scheduled_install_fourth_week.as_ref()
    }

    /// Sets the value of ScheduledInstallSecondWeek
    pub fn set_scheduled_install_second_week(&mut self, value: i32) {
        self.scheduled_install_second_week = Some(value);
    }

    /// Gets the value of ScheduledInstallSecondWeek
    pub fn get_scheduled_install_second_week(&self) -> Option<&i32> {
        self.scheduled_install_second_week.as_ref()
    }

    /// Sets the value of ScheduledInstallThirdWeek
    pub fn set_scheduled_install_third_week(&mut self, value: i32) {
        self.scheduled_install_third_week = Some(value);
    }

    /// Gets the value of ScheduledInstallThirdWeek
    pub fn get_scheduled_install_third_week(&self) -> Option<&i32> {
        self.scheduled_install_third_week.as_ref()
    }

    /// Sets the value of ScheduledInstallTime
    pub fn set_scheduled_install_time(&mut self, value: i32) {
        self.scheduled_install_time = Some(value);
    }

    /// Gets the value of ScheduledInstallTime
    pub fn get_scheduled_install_time(&self) -> Option<&i32> {
        self.scheduled_install_time.as_ref()
    }

    /// Sets the value of ScheduleImminentRestartWarning
    pub fn set_schedule_imminent_restart_warning(&mut self, value: i32) {
        self.schedule_imminent_restart_warning = Some(value);
    }

    /// Gets the value of ScheduleImminentRestartWarning
    pub fn get_schedule_imminent_restart_warning(&self) -> Option<&i32> {
        self.schedule_imminent_restart_warning.as_ref()
    }

    /// Sets the value of ScheduleRestartWarning
    pub fn set_schedule_restart_warning(&mut self, value: i32) {
        self.schedule_restart_warning = Some(value);
    }

    /// Gets the value of ScheduleRestartWarning
    pub fn get_schedule_restart_warning(&self) -> Option<&i32> {
        self.schedule_restart_warning.as_ref()
    }

    /// Sets the value of SetAutoRestartNotificationDisable
    pub fn set_set_auto_restart_notification_disable(&mut self, value: i32) {
        self.set_auto_restart_notification_disable = Some(value);
    }

    /// Gets the value of SetAutoRestartNotificationDisable
    pub fn get_set_auto_restart_notification_disable(&self) -> Option<&i32> {
        self.set_auto_restart_notification_disable.as_ref()
    }

    /// Sets the value of SetDisablePauseUXAccess
    pub fn set_set_disable_pause_uxaccess(&mut self, value: i32) {
        self.set_disable_pause_uxaccess = Some(value);
    }

    /// Gets the value of SetDisablePauseUXAccess
    pub fn get_set_disable_pause_uxaccess(&self) -> Option<&i32> {
        self.set_disable_pause_uxaccess.as_ref()
    }

    /// Sets the value of SetDisableUXWUAccess
    pub fn set_set_disable_uxwuaccess(&mut self, value: i32) {
        self.set_disable_uxwuaccess = Some(value);
    }

    /// Gets the value of SetDisableUXWUAccess
    pub fn get_set_disable_uxwuaccess(&self) -> Option<&i32> {
        self.set_disable_uxwuaccess.as_ref()
    }

    /// Sets the value of SetEDURestart
    pub fn set_set_edurestart(&mut self, value: i32) {
        self.set_edurestart = Some(value);
    }

    /// Gets the value of SetEDURestart
    pub fn get_set_edurestart(&self) -> Option<&i32> {
        self.set_edurestart.as_ref()
    }

    /// Sets the value of SetPolicyDrivenUpdateSourceForDriverUpdates
    pub fn set_set_policy_driven_update_source_for_driver_updates(&mut self, value: i32) {
        self.set_policy_driven_update_source_for_driver_updates = Some(value);
    }

    /// Gets the value of SetPolicyDrivenUpdateSourceForDriverUpdates
    pub fn get_set_policy_driven_update_source_for_driver_updates(&self) -> Option<&i32> {
        self.set_policy_driven_update_source_for_driver_updates.as_ref()
    }

    /// Sets the value of SetPolicyDrivenUpdateSourceForFeatureUpdates
    pub fn set_set_policy_driven_update_source_for_feature_updates(&mut self, value: i32) {
        self.set_policy_driven_update_source_for_feature_updates = Some(value);
    }

    /// Gets the value of SetPolicyDrivenUpdateSourceForFeatureUpdates
    pub fn get_set_policy_driven_update_source_for_feature_updates(&self) -> Option<&i32> {
        self.set_policy_driven_update_source_for_feature_updates.as_ref()
    }

    /// Sets the value of SetPolicyDrivenUpdateSourceForOtherUpdates
    pub fn set_set_policy_driven_update_source_for_other_updates(&mut self, value: i32) {
        self.set_policy_driven_update_source_for_other_updates = Some(value);
    }

    /// Gets the value of SetPolicyDrivenUpdateSourceForOtherUpdates
    pub fn get_set_policy_driven_update_source_for_other_updates(&self) -> Option<&i32> {
        self.set_policy_driven_update_source_for_other_updates.as_ref()
    }

    /// Sets the value of SetPolicyDrivenUpdateSourceForQualityUpdates
    pub fn set_set_policy_driven_update_source_for_quality_updates(&mut self, value: i32) {
        self.set_policy_driven_update_source_for_quality_updates = Some(value);
    }

    /// Gets the value of SetPolicyDrivenUpdateSourceForQualityUpdates
    pub fn get_set_policy_driven_update_source_for_quality_updates(&self) -> Option<&i32> {
        self.set_policy_driven_update_source_for_quality_updates.as_ref()
    }

    /// Sets the value of SetProxyBehaviorForUpdateDetection
    pub fn set_set_proxy_behavior_for_update_detection(&mut self, value: i32) {
        self.set_proxy_behavior_for_update_detection = Some(value);
    }

    /// Gets the value of SetProxyBehaviorForUpdateDetection
    pub fn get_set_proxy_behavior_for_update_detection(&self) -> Option<&i32> {
        self.set_proxy_behavior_for_update_detection.as_ref()
    }

    /// Sets the value of TargetReleaseVersion
    pub fn set_target_release_version(&mut self, value: String) {
        self.target_release_version = Some(value);
    }

    /// Gets the value of TargetReleaseVersion
    pub fn get_target_release_version(&self) -> Option<&String> {
        self.target_release_version.as_ref()
    }

    /// Sets the value of UpdateNotificationLevel
    pub fn set_update_notification_level(&mut self, value: i32) {
        self.update_notification_level = Some(value);
    }

    /// Gets the value of UpdateNotificationLevel
    pub fn get_update_notification_level(&self) -> Option<&i32> {
        self.update_notification_level.as_ref()
    }

    /// Sets the value of UpdateServiceUrl
    pub fn set_update_service_url(&mut self, value: String) {
        self.update_service_url = Some(value);
    }

    /// Gets the value of UpdateServiceUrl
    pub fn get_update_service_url(&self) -> Option<&String> {
        self.update_service_url.as_ref()
    }

    /// Sets the value of UpdateServiceUrlAlternate
    pub fn set_update_service_url_alternate(&mut self, value: String) {
        self.update_service_url_alternate = Some(value);
    }

    /// Gets the value of UpdateServiceUrlAlternate
    pub fn get_update_service_url_alternate(&self) -> Option<&String> {
        self.update_service_url_alternate.as_ref()
    }
}

