// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Config01_Privacy02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Config01_Privacy02 {

/// 
    #[serde(rename = "AllowAutoAcceptPairingAndPrivacyConsentPrompts")]
    pub allow_auto_accept_pairing_and_privacy_consent_prompts: Option<i32>,

/// 
    #[serde(rename = "AllowCrossDeviceClipboard")]
    pub allow_cross_device_clipboard: Option<i32>,

/// 
    #[serde(rename = "AllowInputPersonalization")]
    pub allow_input_personalization: Option<i32>,

/// 
    #[serde(rename = "DisableAdvertisingId")]
    pub disable_advertising_id: Option<i32>,

/// 
    #[serde(rename = "DisablePrivacyExperience")]
    pub disable_privacy_experience: Option<i32>,

/// 
    #[serde(rename = "EnableActivityFeed")]
    pub enable_activity_feed: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessAccountInfo")]
    pub let_apps_access_account_info: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessAccountInfo_ForceAllowTheseApps")]
    pub let_apps_access_account_info__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessAccountInfo_ForceDenyTheseApps")]
    pub let_apps_access_account_info__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessAccountInfo_UserInControlOfTheseApps")]
    pub let_apps_access_account_info__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessBackgroundSpatialPerception")]
    pub let_apps_access_background_spatial_perception: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessBackgroundSpatialPerception_ForceAllowTheseApps")]
    pub let_apps_access_background_spatial_perception__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessBackgroundSpatialPerception_ForceDenyTheseApps")]
    pub let_apps_access_background_spatial_perception__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessBackgroundSpatialPerception_UserInControlOfTheseApps")]
    pub let_apps_access_background_spatial_perception__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCalendar")]
    pub let_apps_access_calendar: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessCalendar_ForceAllowTheseApps")]
    pub let_apps_access_calendar__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCalendar_ForceDenyTheseApps")]
    pub let_apps_access_calendar__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCalendar_UserInControlOfTheseApps")]
    pub let_apps_access_calendar__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCallHistory")]
    pub let_apps_access_call_history: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessCallHistory_ForceAllowTheseApps")]
    pub let_apps_access_call_history__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCallHistory_ForceDenyTheseApps")]
    pub let_apps_access_call_history__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCallHistory_UserInControlOfTheseApps")]
    pub let_apps_access_call_history__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCamera")]
    pub let_apps_access_camera: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessCamera_ForceAllowTheseApps")]
    pub let_apps_access_camera__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCamera_ForceDenyTheseApps")]
    pub let_apps_access_camera__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessCamera_UserInControlOfTheseApps")]
    pub let_apps_access_camera__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessContacts")]
    pub let_apps_access_contacts: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessContacts_ForceAllowTheseApps")]
    pub let_apps_access_contacts__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessContacts_ForceDenyTheseApps")]
    pub let_apps_access_contacts__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessContacts_UserInControlOfTheseApps")]
    pub let_apps_access_contacts__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessEmail")]
    pub let_apps_access_email: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessEmail_ForceAllowTheseApps")]
    pub let_apps_access_email__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessEmail_ForceDenyTheseApps")]
    pub let_apps_access_email__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessEmail_UserInControlOfTheseApps")]
    pub let_apps_access_email__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGazeInput")]
    pub let_apps_access_gaze_input: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessGazeInput_ForceAllowTheseApps")]
    pub let_apps_access_gaze_input__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGazeInput_ForceDenyTheseApps")]
    pub let_apps_access_gaze_input__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGazeInput_UserInControlOfTheseApps")]
    pub let_apps_access_gaze_input__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureProgrammatic")]
    pub let_apps_access_graphics_capture_programmatic: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureProgrammatic_ForceAllowTheseApps")]
    pub let_apps_access_graphics_capture_programmatic__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureProgrammatic_ForceDenyTheseApps")]
    pub let_apps_access_graphics_capture_programmatic__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureProgrammatic_UserInControlOfTheseApps")]
    pub let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureWithoutBorder")]
    pub let_apps_access_graphics_capture_without_border: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureWithoutBorder_ForceAllowTheseApps")]
    pub let_apps_access_graphics_capture_without_border__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureWithoutBorder_ForceDenyTheseApps")]
    pub let_apps_access_graphics_capture_without_border__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessGraphicsCaptureWithoutBorder_UserInControlOfTheseApps")]
    pub let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessLocation")]
    pub let_apps_access_location: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessLocation_ForceAllowTheseApps")]
    pub let_apps_access_location__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessLocation_ForceDenyTheseApps")]
    pub let_apps_access_location__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessLocation_UserInControlOfTheseApps")]
    pub let_apps_access_location__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMessaging")]
    pub let_apps_access_messaging: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessMessaging_ForceAllowTheseApps")]
    pub let_apps_access_messaging__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMessaging_ForceDenyTheseApps")]
    pub let_apps_access_messaging__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMessaging_UserInControlOfTheseApps")]
    pub let_apps_access_messaging__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMicrophone")]
    pub let_apps_access_microphone: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessMicrophone_ForceAllowTheseApps")]
    pub let_apps_access_microphone__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMicrophone_ForceDenyTheseApps")]
    pub let_apps_access_microphone__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMicrophone_UserInControlOfTheseApps")]
    pub let_apps_access_microphone__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMotion")]
    pub let_apps_access_motion: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessMotion_ForceAllowTheseApps")]
    pub let_apps_access_motion__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMotion_ForceDenyTheseApps")]
    pub let_apps_access_motion__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessMotion_UserInControlOfTheseApps")]
    pub let_apps_access_motion__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessNotifications")]
    pub let_apps_access_notifications: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessNotifications_ForceAllowTheseApps")]
    pub let_apps_access_notifications__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessNotifications_ForceDenyTheseApps")]
    pub let_apps_access_notifications__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessNotifications_UserInControlOfTheseApps")]
    pub let_apps_access_notifications__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessPhone")]
    pub let_apps_access_phone: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessPhone_ForceAllowTheseApps")]
    pub let_apps_access_phone__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessPhone_ForceDenyTheseApps")]
    pub let_apps_access_phone__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessPhone_UserInControlOfTheseApps")]
    pub let_apps_access_phone__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessRadios")]
    pub let_apps_access_radios: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessRadios_ForceAllowTheseApps")]
    pub let_apps_access_radios__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessRadios_ForceDenyTheseApps")]
    pub let_apps_access_radios__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessRadios_UserInControlOfTheseApps")]
    pub let_apps_access_radios__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTasks")]
    pub let_apps_access_tasks: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessTasks_ForceAllowTheseApps")]
    pub let_apps_access_tasks__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTasks_ForceDenyTheseApps")]
    pub let_apps_access_tasks__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTasks_UserInControlOfTheseApps")]
    pub let_apps_access_tasks__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTrustedDevices")]
    pub let_apps_access_trusted_devices: Option<i32>,

/// 
    #[serde(rename = "LetAppsAccessTrustedDevices_ForceAllowTheseApps")]
    pub let_apps_access_trusted_devices__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTrustedDevices_ForceDenyTheseApps")]
    pub let_apps_access_trusted_devices__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsAccessTrustedDevices_UserInControlOfTheseApps")]
    pub let_apps_access_trusted_devices__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsActivateWithVoice")]
    pub let_apps_activate_with_voice: Option<i32>,

/// 
    #[serde(rename = "LetAppsActivateWithVoiceAboveLock")]
    pub let_apps_activate_with_voice_above_lock: Option<i32>,

/// 
    #[serde(rename = "LetAppsGetDiagnosticInfo")]
    pub let_apps_get_diagnostic_info: Option<i32>,

/// 
    #[serde(rename = "LetAppsGetDiagnosticInfo_ForceAllowTheseApps")]
    pub let_apps_get_diagnostic_info__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsGetDiagnosticInfo_ForceDenyTheseApps")]
    pub let_apps_get_diagnostic_info__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsGetDiagnosticInfo_UserInControlOfTheseApps")]
    pub let_apps_get_diagnostic_info__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsRunInBackground")]
    pub let_apps_run_in_background: Option<i32>,

/// 
    #[serde(rename = "LetAppsRunInBackground_ForceAllowTheseApps")]
    pub let_apps_run_in_background__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsRunInBackground_ForceDenyTheseApps")]
    pub let_apps_run_in_background__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsRunInBackground_UserInControlOfTheseApps")]
    pub let_apps_run_in_background__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsSyncWithDevices")]
    pub let_apps_sync_with_devices: Option<i32>,

/// 
    #[serde(rename = "LetAppsSyncWithDevices_ForceAllowTheseApps")]
    pub let_apps_sync_with_devices__force_allow_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsSyncWithDevices_ForceDenyTheseApps")]
    pub let_apps_sync_with_devices__force_deny_these_apps: Option<String>,

/// 
    #[serde(rename = "LetAppsSyncWithDevices_UserInControlOfTheseApps")]
    pub let_apps_sync_with_devices__user_in_control_of_these_apps: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "PublishUserActivities")]
    pub publish_user_activities: Option<i32>,

/// 
    #[serde(rename = "UploadUserActivities")]
    pub upload_user_activities: Option<i32>,
}

impl MDM_Policy_Config01_Privacy02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            allow_auto_accept_pairing_and_privacy_consent_prompts: None,
            allow_cross_device_clipboard: None,
            allow_input_personalization: None,
            disable_advertising_id: None,
            disable_privacy_experience: None,
            enable_activity_feed: None,
            instance_id: None,
            let_apps_access_account_info: None,
            let_apps_access_account_info__force_allow_these_apps: None,
            let_apps_access_account_info__force_deny_these_apps: None,
            let_apps_access_account_info__user_in_control_of_these_apps: None,
            let_apps_access_background_spatial_perception: None,
            let_apps_access_background_spatial_perception__force_allow_these_apps: None,
            let_apps_access_background_spatial_perception__force_deny_these_apps: None,
            let_apps_access_background_spatial_perception__user_in_control_of_these_apps: None,
            let_apps_access_calendar: None,
            let_apps_access_calendar__force_allow_these_apps: None,
            let_apps_access_calendar__force_deny_these_apps: None,
            let_apps_access_calendar__user_in_control_of_these_apps: None,
            let_apps_access_call_history: None,
            let_apps_access_call_history__force_allow_these_apps: None,
            let_apps_access_call_history__force_deny_these_apps: None,
            let_apps_access_call_history__user_in_control_of_these_apps: None,
            let_apps_access_camera: None,
            let_apps_access_camera__force_allow_these_apps: None,
            let_apps_access_camera__force_deny_these_apps: None,
            let_apps_access_camera__user_in_control_of_these_apps: None,
            let_apps_access_contacts: None,
            let_apps_access_contacts__force_allow_these_apps: None,
            let_apps_access_contacts__force_deny_these_apps: None,
            let_apps_access_contacts__user_in_control_of_these_apps: None,
            let_apps_access_email: None,
            let_apps_access_email__force_allow_these_apps: None,
            let_apps_access_email__force_deny_these_apps: None,
            let_apps_access_email__user_in_control_of_these_apps: None,
            let_apps_access_gaze_input: None,
            let_apps_access_gaze_input__force_allow_these_apps: None,
            let_apps_access_gaze_input__force_deny_these_apps: None,
            let_apps_access_gaze_input__user_in_control_of_these_apps: None,
            let_apps_access_graphics_capture_programmatic: None,
            let_apps_access_graphics_capture_programmatic__force_allow_these_apps: None,
            let_apps_access_graphics_capture_programmatic__force_deny_these_apps: None,
            let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps: None,
            let_apps_access_graphics_capture_without_border: None,
            let_apps_access_graphics_capture_without_border__force_allow_these_apps: None,
            let_apps_access_graphics_capture_without_border__force_deny_these_apps: None,
            let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps: None,
            let_apps_access_location: None,
            let_apps_access_location__force_allow_these_apps: None,
            let_apps_access_location__force_deny_these_apps: None,
            let_apps_access_location__user_in_control_of_these_apps: None,
            let_apps_access_messaging: None,
            let_apps_access_messaging__force_allow_these_apps: None,
            let_apps_access_messaging__force_deny_these_apps: None,
            let_apps_access_messaging__user_in_control_of_these_apps: None,
            let_apps_access_microphone: None,
            let_apps_access_microphone__force_allow_these_apps: None,
            let_apps_access_microphone__force_deny_these_apps: None,
            let_apps_access_microphone__user_in_control_of_these_apps: None,
            let_apps_access_motion: None,
            let_apps_access_motion__force_allow_these_apps: None,
            let_apps_access_motion__force_deny_these_apps: None,
            let_apps_access_motion__user_in_control_of_these_apps: None,
            let_apps_access_notifications: None,
            let_apps_access_notifications__force_allow_these_apps: None,
            let_apps_access_notifications__force_deny_these_apps: None,
            let_apps_access_notifications__user_in_control_of_these_apps: None,
            let_apps_access_phone: None,
            let_apps_access_phone__force_allow_these_apps: None,
            let_apps_access_phone__force_deny_these_apps: None,
            let_apps_access_phone__user_in_control_of_these_apps: None,
            let_apps_access_radios: None,
            let_apps_access_radios__force_allow_these_apps: None,
            let_apps_access_radios__force_deny_these_apps: None,
            let_apps_access_radios__user_in_control_of_these_apps: None,
            let_apps_access_tasks: None,
            let_apps_access_tasks__force_allow_these_apps: None,
            let_apps_access_tasks__force_deny_these_apps: None,
            let_apps_access_tasks__user_in_control_of_these_apps: None,
            let_apps_access_trusted_devices: None,
            let_apps_access_trusted_devices__force_allow_these_apps: None,
            let_apps_access_trusted_devices__force_deny_these_apps: None,
            let_apps_access_trusted_devices__user_in_control_of_these_apps: None,
            let_apps_activate_with_voice: None,
            let_apps_activate_with_voice_above_lock: None,
            let_apps_get_diagnostic_info: None,
            let_apps_get_diagnostic_info__force_allow_these_apps: None,
            let_apps_get_diagnostic_info__force_deny_these_apps: None,
            let_apps_get_diagnostic_info__user_in_control_of_these_apps: None,
            let_apps_run_in_background: None,
            let_apps_run_in_background__force_allow_these_apps: None,
            let_apps_run_in_background__force_deny_these_apps: None,
            let_apps_run_in_background__user_in_control_of_these_apps: None,
            let_apps_sync_with_devices: None,
            let_apps_sync_with_devices__force_allow_these_apps: None,
            let_apps_sync_with_devices__force_deny_these_apps: None,
            let_apps_sync_with_devices__user_in_control_of_these_apps: None,
            parent_id: None,
            publish_user_activities: None,
            upload_user_activities: None,
        }
    }


    /// Sets the value of AllowAutoAcceptPairingAndPrivacyConsentPrompts
    pub fn set_allow_auto_accept_pairing_and_privacy_consent_prompts(&mut self, value: i32) {
        self.allow_auto_accept_pairing_and_privacy_consent_prompts = Some(value);
    }

    /// Gets the value of AllowAutoAcceptPairingAndPrivacyConsentPrompts
    pub fn get_allow_auto_accept_pairing_and_privacy_consent_prompts(&self) -> Option<&i32> {
        self.allow_auto_accept_pairing_and_privacy_consent_prompts.as_ref()
    }

    /// Sets the value of AllowCrossDeviceClipboard
    pub fn set_allow_cross_device_clipboard(&mut self, value: i32) {
        self.allow_cross_device_clipboard = Some(value);
    }

    /// Gets the value of AllowCrossDeviceClipboard
    pub fn get_allow_cross_device_clipboard(&self) -> Option<&i32> {
        self.allow_cross_device_clipboard.as_ref()
    }

    /// Sets the value of AllowInputPersonalization
    pub fn set_allow_input_personalization(&mut self, value: i32) {
        self.allow_input_personalization = Some(value);
    }

    /// Gets the value of AllowInputPersonalization
    pub fn get_allow_input_personalization(&self) -> Option<&i32> {
        self.allow_input_personalization.as_ref()
    }

    /// Sets the value of DisableAdvertisingId
    pub fn set_disable_advertising_id(&mut self, value: i32) {
        self.disable_advertising_id = Some(value);
    }

    /// Gets the value of DisableAdvertisingId
    pub fn get_disable_advertising_id(&self) -> Option<&i32> {
        self.disable_advertising_id.as_ref()
    }

    /// Sets the value of DisablePrivacyExperience
    pub fn set_disable_privacy_experience(&mut self, value: i32) {
        self.disable_privacy_experience = Some(value);
    }

    /// Gets the value of DisablePrivacyExperience
    pub fn get_disable_privacy_experience(&self) -> Option<&i32> {
        self.disable_privacy_experience.as_ref()
    }

    /// Sets the value of EnableActivityFeed
    pub fn set_enable_activity_feed(&mut self, value: i32) {
        self.enable_activity_feed = Some(value);
    }

    /// Gets the value of EnableActivityFeed
    pub fn get_enable_activity_feed(&self) -> Option<&i32> {
        self.enable_activity_feed.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of LetAppsAccessAccountInfo
    pub fn set_let_apps_access_account_info(&mut self, value: i32) {
        self.let_apps_access_account_info = Some(value);
    }

    /// Gets the value of LetAppsAccessAccountInfo
    pub fn get_let_apps_access_account_info(&self) -> Option<&i32> {
        self.let_apps_access_account_info.as_ref()
    }

    /// Sets the value of LetAppsAccessAccountInfo_ForceAllowTheseApps
    pub fn set_let_apps_access_account_info__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_account_info__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessAccountInfo_ForceAllowTheseApps
    pub fn get_let_apps_access_account_info__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_account_info__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessAccountInfo_ForceDenyTheseApps
    pub fn set_let_apps_access_account_info__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_account_info__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessAccountInfo_ForceDenyTheseApps
    pub fn get_let_apps_access_account_info__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_account_info__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessAccountInfo_UserInControlOfTheseApps
    pub fn set_let_apps_access_account_info__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_account_info__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessAccountInfo_UserInControlOfTheseApps
    pub fn get_let_apps_access_account_info__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_account_info__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessBackgroundSpatialPerception
    pub fn set_let_apps_access_background_spatial_perception(&mut self, value: i32) {
        self.let_apps_access_background_spatial_perception = Some(value);
    }

    /// Gets the value of LetAppsAccessBackgroundSpatialPerception
    pub fn get_let_apps_access_background_spatial_perception(&self) -> Option<&i32> {
        self.let_apps_access_background_spatial_perception.as_ref()
    }

    /// Sets the value of LetAppsAccessBackgroundSpatialPerception_ForceAllowTheseApps
    pub fn set_let_apps_access_background_spatial_perception__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_background_spatial_perception__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessBackgroundSpatialPerception_ForceAllowTheseApps
    pub fn get_let_apps_access_background_spatial_perception__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_background_spatial_perception__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessBackgroundSpatialPerception_ForceDenyTheseApps
    pub fn set_let_apps_access_background_spatial_perception__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_background_spatial_perception__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessBackgroundSpatialPerception_ForceDenyTheseApps
    pub fn get_let_apps_access_background_spatial_perception__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_background_spatial_perception__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessBackgroundSpatialPerception_UserInControlOfTheseApps
    pub fn set_let_apps_access_background_spatial_perception__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_background_spatial_perception__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessBackgroundSpatialPerception_UserInControlOfTheseApps
    pub fn get_let_apps_access_background_spatial_perception__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_background_spatial_perception__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCalendar
    pub fn set_let_apps_access_calendar(&mut self, value: i32) {
        self.let_apps_access_calendar = Some(value);
    }

    /// Gets the value of LetAppsAccessCalendar
    pub fn get_let_apps_access_calendar(&self) -> Option<&i32> {
        self.let_apps_access_calendar.as_ref()
    }

    /// Sets the value of LetAppsAccessCalendar_ForceAllowTheseApps
    pub fn set_let_apps_access_calendar__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_calendar__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCalendar_ForceAllowTheseApps
    pub fn get_let_apps_access_calendar__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_calendar__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCalendar_ForceDenyTheseApps
    pub fn set_let_apps_access_calendar__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_calendar__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCalendar_ForceDenyTheseApps
    pub fn get_let_apps_access_calendar__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_calendar__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCalendar_UserInControlOfTheseApps
    pub fn set_let_apps_access_calendar__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_calendar__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCalendar_UserInControlOfTheseApps
    pub fn get_let_apps_access_calendar__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_calendar__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCallHistory
    pub fn set_let_apps_access_call_history(&mut self, value: i32) {
        self.let_apps_access_call_history = Some(value);
    }

    /// Gets the value of LetAppsAccessCallHistory
    pub fn get_let_apps_access_call_history(&self) -> Option<&i32> {
        self.let_apps_access_call_history.as_ref()
    }

    /// Sets the value of LetAppsAccessCallHistory_ForceAllowTheseApps
    pub fn set_let_apps_access_call_history__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_call_history__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCallHistory_ForceAllowTheseApps
    pub fn get_let_apps_access_call_history__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_call_history__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCallHistory_ForceDenyTheseApps
    pub fn set_let_apps_access_call_history__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_call_history__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCallHistory_ForceDenyTheseApps
    pub fn get_let_apps_access_call_history__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_call_history__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCallHistory_UserInControlOfTheseApps
    pub fn set_let_apps_access_call_history__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_call_history__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCallHistory_UserInControlOfTheseApps
    pub fn get_let_apps_access_call_history__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_call_history__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCamera
    pub fn set_let_apps_access_camera(&mut self, value: i32) {
        self.let_apps_access_camera = Some(value);
    }

    /// Gets the value of LetAppsAccessCamera
    pub fn get_let_apps_access_camera(&self) -> Option<&i32> {
        self.let_apps_access_camera.as_ref()
    }

    /// Sets the value of LetAppsAccessCamera_ForceAllowTheseApps
    pub fn set_let_apps_access_camera__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_camera__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCamera_ForceAllowTheseApps
    pub fn get_let_apps_access_camera__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_camera__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCamera_ForceDenyTheseApps
    pub fn set_let_apps_access_camera__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_camera__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCamera_ForceDenyTheseApps
    pub fn get_let_apps_access_camera__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_camera__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessCamera_UserInControlOfTheseApps
    pub fn set_let_apps_access_camera__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_camera__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessCamera_UserInControlOfTheseApps
    pub fn get_let_apps_access_camera__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_camera__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessContacts
    pub fn set_let_apps_access_contacts(&mut self, value: i32) {
        self.let_apps_access_contacts = Some(value);
    }

    /// Gets the value of LetAppsAccessContacts
    pub fn get_let_apps_access_contacts(&self) -> Option<&i32> {
        self.let_apps_access_contacts.as_ref()
    }

    /// Sets the value of LetAppsAccessContacts_ForceAllowTheseApps
    pub fn set_let_apps_access_contacts__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_contacts__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessContacts_ForceAllowTheseApps
    pub fn get_let_apps_access_contacts__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_contacts__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessContacts_ForceDenyTheseApps
    pub fn set_let_apps_access_contacts__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_contacts__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessContacts_ForceDenyTheseApps
    pub fn get_let_apps_access_contacts__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_contacts__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessContacts_UserInControlOfTheseApps
    pub fn set_let_apps_access_contacts__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_contacts__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessContacts_UserInControlOfTheseApps
    pub fn get_let_apps_access_contacts__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_contacts__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessEmail
    pub fn set_let_apps_access_email(&mut self, value: i32) {
        self.let_apps_access_email = Some(value);
    }

    /// Gets the value of LetAppsAccessEmail
    pub fn get_let_apps_access_email(&self) -> Option<&i32> {
        self.let_apps_access_email.as_ref()
    }

    /// Sets the value of LetAppsAccessEmail_ForceAllowTheseApps
    pub fn set_let_apps_access_email__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_email__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessEmail_ForceAllowTheseApps
    pub fn get_let_apps_access_email__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_email__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessEmail_ForceDenyTheseApps
    pub fn set_let_apps_access_email__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_email__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessEmail_ForceDenyTheseApps
    pub fn get_let_apps_access_email__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_email__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessEmail_UserInControlOfTheseApps
    pub fn set_let_apps_access_email__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_email__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessEmail_UserInControlOfTheseApps
    pub fn get_let_apps_access_email__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_email__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGazeInput
    pub fn set_let_apps_access_gaze_input(&mut self, value: i32) {
        self.let_apps_access_gaze_input = Some(value);
    }

    /// Gets the value of LetAppsAccessGazeInput
    pub fn get_let_apps_access_gaze_input(&self) -> Option<&i32> {
        self.let_apps_access_gaze_input.as_ref()
    }

    /// Sets the value of LetAppsAccessGazeInput_ForceAllowTheseApps
    pub fn set_let_apps_access_gaze_input__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_gaze_input__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGazeInput_ForceAllowTheseApps
    pub fn get_let_apps_access_gaze_input__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_gaze_input__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGazeInput_ForceDenyTheseApps
    pub fn set_let_apps_access_gaze_input__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_gaze_input__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGazeInput_ForceDenyTheseApps
    pub fn get_let_apps_access_gaze_input__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_gaze_input__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGazeInput_UserInControlOfTheseApps
    pub fn set_let_apps_access_gaze_input__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_gaze_input__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGazeInput_UserInControlOfTheseApps
    pub fn get_let_apps_access_gaze_input__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_gaze_input__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureProgrammatic
    pub fn set_let_apps_access_graphics_capture_programmatic(&mut self, value: i32) {
        self.let_apps_access_graphics_capture_programmatic = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureProgrammatic
    pub fn get_let_apps_access_graphics_capture_programmatic(&self) -> Option<&i32> {
        self.let_apps_access_graphics_capture_programmatic.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureProgrammatic_ForceAllowTheseApps
    pub fn set_let_apps_access_graphics_capture_programmatic__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_programmatic__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureProgrammatic_ForceAllowTheseApps
    pub fn get_let_apps_access_graphics_capture_programmatic__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_programmatic__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureProgrammatic_ForceDenyTheseApps
    pub fn set_let_apps_access_graphics_capture_programmatic__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_programmatic__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureProgrammatic_ForceDenyTheseApps
    pub fn get_let_apps_access_graphics_capture_programmatic__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_programmatic__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureProgrammatic_UserInControlOfTheseApps
    pub fn set_let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureProgrammatic_UserInControlOfTheseApps
    pub fn get_let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_programmatic__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureWithoutBorder
    pub fn set_let_apps_access_graphics_capture_without_border(&mut self, value: i32) {
        self.let_apps_access_graphics_capture_without_border = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureWithoutBorder
    pub fn get_let_apps_access_graphics_capture_without_border(&self) -> Option<&i32> {
        self.let_apps_access_graphics_capture_without_border.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureWithoutBorder_ForceAllowTheseApps
    pub fn set_let_apps_access_graphics_capture_without_border__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_without_border__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureWithoutBorder_ForceAllowTheseApps
    pub fn get_let_apps_access_graphics_capture_without_border__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_without_border__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureWithoutBorder_ForceDenyTheseApps
    pub fn set_let_apps_access_graphics_capture_without_border__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_without_border__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureWithoutBorder_ForceDenyTheseApps
    pub fn get_let_apps_access_graphics_capture_without_border__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_without_border__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessGraphicsCaptureWithoutBorder_UserInControlOfTheseApps
    pub fn set_let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessGraphicsCaptureWithoutBorder_UserInControlOfTheseApps
    pub fn get_let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_graphics_capture_without_border__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessLocation
    pub fn set_let_apps_access_location(&mut self, value: i32) {
        self.let_apps_access_location = Some(value);
    }

    /// Gets the value of LetAppsAccessLocation
    pub fn get_let_apps_access_location(&self) -> Option<&i32> {
        self.let_apps_access_location.as_ref()
    }

    /// Sets the value of LetAppsAccessLocation_ForceAllowTheseApps
    pub fn set_let_apps_access_location__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_location__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessLocation_ForceAllowTheseApps
    pub fn get_let_apps_access_location__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_location__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessLocation_ForceDenyTheseApps
    pub fn set_let_apps_access_location__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_location__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessLocation_ForceDenyTheseApps
    pub fn get_let_apps_access_location__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_location__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessLocation_UserInControlOfTheseApps
    pub fn set_let_apps_access_location__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_location__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessLocation_UserInControlOfTheseApps
    pub fn get_let_apps_access_location__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_location__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMessaging
    pub fn set_let_apps_access_messaging(&mut self, value: i32) {
        self.let_apps_access_messaging = Some(value);
    }

    /// Gets the value of LetAppsAccessMessaging
    pub fn get_let_apps_access_messaging(&self) -> Option<&i32> {
        self.let_apps_access_messaging.as_ref()
    }

    /// Sets the value of LetAppsAccessMessaging_ForceAllowTheseApps
    pub fn set_let_apps_access_messaging__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_messaging__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMessaging_ForceAllowTheseApps
    pub fn get_let_apps_access_messaging__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_messaging__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMessaging_ForceDenyTheseApps
    pub fn set_let_apps_access_messaging__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_messaging__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMessaging_ForceDenyTheseApps
    pub fn get_let_apps_access_messaging__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_messaging__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMessaging_UserInControlOfTheseApps
    pub fn set_let_apps_access_messaging__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_messaging__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMessaging_UserInControlOfTheseApps
    pub fn get_let_apps_access_messaging__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_messaging__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMicrophone
    pub fn set_let_apps_access_microphone(&mut self, value: i32) {
        self.let_apps_access_microphone = Some(value);
    }

    /// Gets the value of LetAppsAccessMicrophone
    pub fn get_let_apps_access_microphone(&self) -> Option<&i32> {
        self.let_apps_access_microphone.as_ref()
    }

    /// Sets the value of LetAppsAccessMicrophone_ForceAllowTheseApps
    pub fn set_let_apps_access_microphone__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_microphone__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMicrophone_ForceAllowTheseApps
    pub fn get_let_apps_access_microphone__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_microphone__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMicrophone_ForceDenyTheseApps
    pub fn set_let_apps_access_microphone__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_microphone__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMicrophone_ForceDenyTheseApps
    pub fn get_let_apps_access_microphone__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_microphone__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMicrophone_UserInControlOfTheseApps
    pub fn set_let_apps_access_microphone__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_microphone__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMicrophone_UserInControlOfTheseApps
    pub fn get_let_apps_access_microphone__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_microphone__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMotion
    pub fn set_let_apps_access_motion(&mut self, value: i32) {
        self.let_apps_access_motion = Some(value);
    }

    /// Gets the value of LetAppsAccessMotion
    pub fn get_let_apps_access_motion(&self) -> Option<&i32> {
        self.let_apps_access_motion.as_ref()
    }

    /// Sets the value of LetAppsAccessMotion_ForceAllowTheseApps
    pub fn set_let_apps_access_motion__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_motion__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMotion_ForceAllowTheseApps
    pub fn get_let_apps_access_motion__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_motion__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMotion_ForceDenyTheseApps
    pub fn set_let_apps_access_motion__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_motion__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMotion_ForceDenyTheseApps
    pub fn get_let_apps_access_motion__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_motion__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessMotion_UserInControlOfTheseApps
    pub fn set_let_apps_access_motion__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_motion__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessMotion_UserInControlOfTheseApps
    pub fn get_let_apps_access_motion__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_motion__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessNotifications
    pub fn set_let_apps_access_notifications(&mut self, value: i32) {
        self.let_apps_access_notifications = Some(value);
    }

    /// Gets the value of LetAppsAccessNotifications
    pub fn get_let_apps_access_notifications(&self) -> Option<&i32> {
        self.let_apps_access_notifications.as_ref()
    }

    /// Sets the value of LetAppsAccessNotifications_ForceAllowTheseApps
    pub fn set_let_apps_access_notifications__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_notifications__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessNotifications_ForceAllowTheseApps
    pub fn get_let_apps_access_notifications__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_notifications__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessNotifications_ForceDenyTheseApps
    pub fn set_let_apps_access_notifications__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_notifications__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessNotifications_ForceDenyTheseApps
    pub fn get_let_apps_access_notifications__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_notifications__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessNotifications_UserInControlOfTheseApps
    pub fn set_let_apps_access_notifications__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_notifications__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessNotifications_UserInControlOfTheseApps
    pub fn get_let_apps_access_notifications__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_notifications__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessPhone
    pub fn set_let_apps_access_phone(&mut self, value: i32) {
        self.let_apps_access_phone = Some(value);
    }

    /// Gets the value of LetAppsAccessPhone
    pub fn get_let_apps_access_phone(&self) -> Option<&i32> {
        self.let_apps_access_phone.as_ref()
    }

    /// Sets the value of LetAppsAccessPhone_ForceAllowTheseApps
    pub fn set_let_apps_access_phone__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_phone__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessPhone_ForceAllowTheseApps
    pub fn get_let_apps_access_phone__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_phone__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessPhone_ForceDenyTheseApps
    pub fn set_let_apps_access_phone__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_phone__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessPhone_ForceDenyTheseApps
    pub fn get_let_apps_access_phone__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_phone__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessPhone_UserInControlOfTheseApps
    pub fn set_let_apps_access_phone__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_phone__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessPhone_UserInControlOfTheseApps
    pub fn get_let_apps_access_phone__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_phone__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessRadios
    pub fn set_let_apps_access_radios(&mut self, value: i32) {
        self.let_apps_access_radios = Some(value);
    }

    /// Gets the value of LetAppsAccessRadios
    pub fn get_let_apps_access_radios(&self) -> Option<&i32> {
        self.let_apps_access_radios.as_ref()
    }

    /// Sets the value of LetAppsAccessRadios_ForceAllowTheseApps
    pub fn set_let_apps_access_radios__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_radios__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessRadios_ForceAllowTheseApps
    pub fn get_let_apps_access_radios__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_radios__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessRadios_ForceDenyTheseApps
    pub fn set_let_apps_access_radios__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_radios__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessRadios_ForceDenyTheseApps
    pub fn get_let_apps_access_radios__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_radios__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessRadios_UserInControlOfTheseApps
    pub fn set_let_apps_access_radios__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_radios__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessRadios_UserInControlOfTheseApps
    pub fn get_let_apps_access_radios__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_radios__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTasks
    pub fn set_let_apps_access_tasks(&mut self, value: i32) {
        self.let_apps_access_tasks = Some(value);
    }

    /// Gets the value of LetAppsAccessTasks
    pub fn get_let_apps_access_tasks(&self) -> Option<&i32> {
        self.let_apps_access_tasks.as_ref()
    }

    /// Sets the value of LetAppsAccessTasks_ForceAllowTheseApps
    pub fn set_let_apps_access_tasks__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_tasks__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTasks_ForceAllowTheseApps
    pub fn get_let_apps_access_tasks__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_tasks__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTasks_ForceDenyTheseApps
    pub fn set_let_apps_access_tasks__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_tasks__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTasks_ForceDenyTheseApps
    pub fn get_let_apps_access_tasks__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_tasks__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTasks_UserInControlOfTheseApps
    pub fn set_let_apps_access_tasks__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_tasks__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTasks_UserInControlOfTheseApps
    pub fn get_let_apps_access_tasks__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_tasks__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTrustedDevices
    pub fn set_let_apps_access_trusted_devices(&mut self, value: i32) {
        self.let_apps_access_trusted_devices = Some(value);
    }

    /// Gets the value of LetAppsAccessTrustedDevices
    pub fn get_let_apps_access_trusted_devices(&self) -> Option<&i32> {
        self.let_apps_access_trusted_devices.as_ref()
    }

    /// Sets the value of LetAppsAccessTrustedDevices_ForceAllowTheseApps
    pub fn set_let_apps_access_trusted_devices__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_access_trusted_devices__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTrustedDevices_ForceAllowTheseApps
    pub fn get_let_apps_access_trusted_devices__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_access_trusted_devices__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTrustedDevices_ForceDenyTheseApps
    pub fn set_let_apps_access_trusted_devices__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_access_trusted_devices__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTrustedDevices_ForceDenyTheseApps
    pub fn get_let_apps_access_trusted_devices__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_access_trusted_devices__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsAccessTrustedDevices_UserInControlOfTheseApps
    pub fn set_let_apps_access_trusted_devices__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_access_trusted_devices__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsAccessTrustedDevices_UserInControlOfTheseApps
    pub fn get_let_apps_access_trusted_devices__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_access_trusted_devices__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsActivateWithVoice
    pub fn set_let_apps_activate_with_voice(&mut self, value: i32) {
        self.let_apps_activate_with_voice = Some(value);
    }

    /// Gets the value of LetAppsActivateWithVoice
    pub fn get_let_apps_activate_with_voice(&self) -> Option<&i32> {
        self.let_apps_activate_with_voice.as_ref()
    }

    /// Sets the value of LetAppsActivateWithVoiceAboveLock
    pub fn set_let_apps_activate_with_voice_above_lock(&mut self, value: i32) {
        self.let_apps_activate_with_voice_above_lock = Some(value);
    }

    /// Gets the value of LetAppsActivateWithVoiceAboveLock
    pub fn get_let_apps_activate_with_voice_above_lock(&self) -> Option<&i32> {
        self.let_apps_activate_with_voice_above_lock.as_ref()
    }

    /// Sets the value of LetAppsGetDiagnosticInfo
    pub fn set_let_apps_get_diagnostic_info(&mut self, value: i32) {
        self.let_apps_get_diagnostic_info = Some(value);
    }

    /// Gets the value of LetAppsGetDiagnosticInfo
    pub fn get_let_apps_get_diagnostic_info(&self) -> Option<&i32> {
        self.let_apps_get_diagnostic_info.as_ref()
    }

    /// Sets the value of LetAppsGetDiagnosticInfo_ForceAllowTheseApps
    pub fn set_let_apps_get_diagnostic_info__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_get_diagnostic_info__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsGetDiagnosticInfo_ForceAllowTheseApps
    pub fn get_let_apps_get_diagnostic_info__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_get_diagnostic_info__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsGetDiagnosticInfo_ForceDenyTheseApps
    pub fn set_let_apps_get_diagnostic_info__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_get_diagnostic_info__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsGetDiagnosticInfo_ForceDenyTheseApps
    pub fn get_let_apps_get_diagnostic_info__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_get_diagnostic_info__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsGetDiagnosticInfo_UserInControlOfTheseApps
    pub fn set_let_apps_get_diagnostic_info__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_get_diagnostic_info__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsGetDiagnosticInfo_UserInControlOfTheseApps
    pub fn get_let_apps_get_diagnostic_info__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_get_diagnostic_info__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsRunInBackground
    pub fn set_let_apps_run_in_background(&mut self, value: i32) {
        self.let_apps_run_in_background = Some(value);
    }

    /// Gets the value of LetAppsRunInBackground
    pub fn get_let_apps_run_in_background(&self) -> Option<&i32> {
        self.let_apps_run_in_background.as_ref()
    }

    /// Sets the value of LetAppsRunInBackground_ForceAllowTheseApps
    pub fn set_let_apps_run_in_background__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_run_in_background__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsRunInBackground_ForceAllowTheseApps
    pub fn get_let_apps_run_in_background__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_run_in_background__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsRunInBackground_ForceDenyTheseApps
    pub fn set_let_apps_run_in_background__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_run_in_background__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsRunInBackground_ForceDenyTheseApps
    pub fn get_let_apps_run_in_background__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_run_in_background__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsRunInBackground_UserInControlOfTheseApps
    pub fn set_let_apps_run_in_background__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_run_in_background__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsRunInBackground_UserInControlOfTheseApps
    pub fn get_let_apps_run_in_background__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_run_in_background__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of LetAppsSyncWithDevices
    pub fn set_let_apps_sync_with_devices(&mut self, value: i32) {
        self.let_apps_sync_with_devices = Some(value);
    }

    /// Gets the value of LetAppsSyncWithDevices
    pub fn get_let_apps_sync_with_devices(&self) -> Option<&i32> {
        self.let_apps_sync_with_devices.as_ref()
    }

    /// Sets the value of LetAppsSyncWithDevices_ForceAllowTheseApps
    pub fn set_let_apps_sync_with_devices__force_allow_these_apps(&mut self, value: String) {
        self.let_apps_sync_with_devices__force_allow_these_apps = Some(value);
    }

    /// Gets the value of LetAppsSyncWithDevices_ForceAllowTheseApps
    pub fn get_let_apps_sync_with_devices__force_allow_these_apps(&self) -> Option<&String> {
        self.let_apps_sync_with_devices__force_allow_these_apps.as_ref()
    }

    /// Sets the value of LetAppsSyncWithDevices_ForceDenyTheseApps
    pub fn set_let_apps_sync_with_devices__force_deny_these_apps(&mut self, value: String) {
        self.let_apps_sync_with_devices__force_deny_these_apps = Some(value);
    }

    /// Gets the value of LetAppsSyncWithDevices_ForceDenyTheseApps
    pub fn get_let_apps_sync_with_devices__force_deny_these_apps(&self) -> Option<&String> {
        self.let_apps_sync_with_devices__force_deny_these_apps.as_ref()
    }

    /// Sets the value of LetAppsSyncWithDevices_UserInControlOfTheseApps
    pub fn set_let_apps_sync_with_devices__user_in_control_of_these_apps(&mut self, value: String) {
        self.let_apps_sync_with_devices__user_in_control_of_these_apps = Some(value);
    }

    /// Gets the value of LetAppsSyncWithDevices_UserInControlOfTheseApps
    pub fn get_let_apps_sync_with_devices__user_in_control_of_these_apps(&self) -> Option<&String> {
        self.let_apps_sync_with_devices__user_in_control_of_these_apps.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of PublishUserActivities
    pub fn set_publish_user_activities(&mut self, value: i32) {
        self.publish_user_activities = Some(value);
    }

    /// Gets the value of PublishUserActivities
    pub fn get_publish_user_activities(&self) -> Option<&i32> {
        self.publish_user_activities.as_ref()
    }

    /// Sets the value of UploadUserActivities
    pub fn set_upload_user_activities(&mut self, value: i32) {
        self.upload_user_activities = Some(value);
    }

    /// Gets the value of UploadUserActivities
    pub fn get_upload_user_activities(&self) -> Option<&i32> {
        self.upload_user_activities.as_ref()
    }
}

