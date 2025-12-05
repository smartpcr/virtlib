// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_Policy_Result01_MixedReality02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_Policy_Result01_MixedReality02 {

/// 
    #[serde(rename = "AADGroupMembershipCacheValidityInDays")]
    pub aadgroup_membership_cache_validity_in_days: Option<i32>,

/// 
    #[serde(rename = "AutomaticDisplayAdjustment")]
    pub automatic_display_adjustment: Option<i32>,

/// 
    #[serde(rename = "BrightnessButtonDisabled")]
    pub brightness_button_disabled: Option<i32>,

/// 
    #[serde(rename = "EyeTrackingCalibrationPrompt")]
    pub eye_tracking_calibration_prompt: Option<i32>,

/// 
    #[serde(rename = "FallbackDiagnostics")]
    pub fallback_diagnostics: Option<i32>,

/// 
    #[serde(rename = "HeadTrackingMode")]
    pub head_tracking_mode: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "MicrophoneDisabled")]
    pub microphone_disabled: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "VisitorAutoLogon")]
    pub visitor_auto_logon: Option<i32>,

/// 
    #[serde(rename = "VolumeButtonDisabled")]
    pub volume_button_disabled: Option<i32>,
}

impl MDM_Policy_Result01_MixedReality02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            aadgroup_membership_cache_validity_in_days: None,
            automatic_display_adjustment: None,
            brightness_button_disabled: None,
            eye_tracking_calibration_prompt: None,
            fallback_diagnostics: None,
            head_tracking_mode: None,
            instance_id: None,
            microphone_disabled: None,
            parent_id: None,
            visitor_auto_logon: None,
            volume_button_disabled: None,
        }
    }


    /// Sets the value of AADGroupMembershipCacheValidityInDays
    pub fn set_aadgroup_membership_cache_validity_in_days(&mut self, value: i32) {
        self.aadgroup_membership_cache_validity_in_days = Some(value);
    }

    /// Gets the value of AADGroupMembershipCacheValidityInDays
    pub fn get_aadgroup_membership_cache_validity_in_days(&self) -> Option<&i32> {
        self.aadgroup_membership_cache_validity_in_days.as_ref()
    }

    /// Sets the value of AutomaticDisplayAdjustment
    pub fn set_automatic_display_adjustment(&mut self, value: i32) {
        self.automatic_display_adjustment = Some(value);
    }

    /// Gets the value of AutomaticDisplayAdjustment
    pub fn get_automatic_display_adjustment(&self) -> Option<&i32> {
        self.automatic_display_adjustment.as_ref()
    }

    /// Sets the value of BrightnessButtonDisabled
    pub fn set_brightness_button_disabled(&mut self, value: i32) {
        self.brightness_button_disabled = Some(value);
    }

    /// Gets the value of BrightnessButtonDisabled
    pub fn get_brightness_button_disabled(&self) -> Option<&i32> {
        self.brightness_button_disabled.as_ref()
    }

    /// Sets the value of EyeTrackingCalibrationPrompt
    pub fn set_eye_tracking_calibration_prompt(&mut self, value: i32) {
        self.eye_tracking_calibration_prompt = Some(value);
    }

    /// Gets the value of EyeTrackingCalibrationPrompt
    pub fn get_eye_tracking_calibration_prompt(&self) -> Option<&i32> {
        self.eye_tracking_calibration_prompt.as_ref()
    }

    /// Sets the value of FallbackDiagnostics
    pub fn set_fallback_diagnostics(&mut self, value: i32) {
        self.fallback_diagnostics = Some(value);
    }

    /// Gets the value of FallbackDiagnostics
    pub fn get_fallback_diagnostics(&self) -> Option<&i32> {
        self.fallback_diagnostics.as_ref()
    }

    /// Sets the value of HeadTrackingMode
    pub fn set_head_tracking_mode(&mut self, value: i32) {
        self.head_tracking_mode = Some(value);
    }

    /// Gets the value of HeadTrackingMode
    pub fn get_head_tracking_mode(&self) -> Option<&i32> {
        self.head_tracking_mode.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of MicrophoneDisabled
    pub fn set_microphone_disabled(&mut self, value: i32) {
        self.microphone_disabled = Some(value);
    }

    /// Gets the value of MicrophoneDisabled
    pub fn get_microphone_disabled(&self) -> Option<&i32> {
        self.microphone_disabled.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of VisitorAutoLogon
    pub fn set_visitor_auto_logon(&mut self, value: i32) {
        self.visitor_auto_logon = Some(value);
    }

    /// Gets the value of VisitorAutoLogon
    pub fn get_visitor_auto_logon(&self) -> Option<&i32> {
        self.visitor_auto_logon.as_ref()
    }

    /// Sets the value of VolumeButtonDisabled
    pub fn set_volume_button_disabled(&mut self, value: i32) {
        self.volume_button_disabled = Some(value);
    }

    /// Gets the value of VolumeButtonDisabled
    pub fn get_volume_button_disabled(&self) -> Option<&i32> {
        self.volume_button_disabled.as_ref()
    }
}

