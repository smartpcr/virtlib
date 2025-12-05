// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_EnrollmentStatusTracking_Tracking03_02 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_EnrollmentStatusTracking_Tracking03_02 {

/// 
    #[serde(rename = "AppId")]
    pub app_id: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "ErrorHresult")]
    pub error_hresult: Option<i32>,

/// 
    #[serde(rename = "InstallationState")]
    pub installation_state: Option<i32>,

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,

/// 
    #[serde(rename = "RebootRequired")]
    pub reboot_required: Option<i32>,

/// 
    #[serde(rename = "TrackingUri")]
    pub tracking_uri: Option<String>,
}

impl MDM_EnrollmentStatusTracking_Tracking03_02 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            app_id: None,
            display_name: None,
            error_hresult: None,
            installation_state: None,
            instance_id: None,
            parent_id: None,
            reboot_required: None,
            tracking_uri: None,
        }
    }


    /// Sets the value of AppId
    pub fn set_app_id(&mut self, value: String) {
        self.app_id = Some(value);
    }

    /// Gets the value of AppId
    pub fn get_app_id(&self) -> Option<&String> {
        self.app_id.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of ErrorHresult
    pub fn set_error_hresult(&mut self, value: i32) {
        self.error_hresult = Some(value);
    }

    /// Gets the value of ErrorHresult
    pub fn get_error_hresult(&self) -> Option<&i32> {
        self.error_hresult.as_ref()
    }

    /// Sets the value of InstallationState
    pub fn set_installation_state(&mut self, value: i32) {
        self.installation_state = Some(value);
    }

    /// Gets the value of InstallationState
    pub fn get_installation_state(&self) -> Option<&i32> {
        self.installation_state.as_ref()
    }

    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }

    /// Sets the value of RebootRequired
    pub fn set_reboot_required(&mut self, value: i32) {
        self.reboot_required = Some(value);
    }

    /// Gets the value of RebootRequired
    pub fn get_reboot_required(&self) -> Option<&i32> {
        self.reboot_required.as_ref()
    }

    /// Sets the value of TrackingUri
    pub fn set_tracking_uri(&mut self, value: String) {
        self.tracking_uri = Some(value);
    }

    /// Gets the value of TrackingUri
    pub fn get_tracking_uri(&self) -> Option<&String> {
        self.tracking_uri.as_ref()
    }
}

