// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.mdm.dmmap
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MDM_DevicePreparation struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MDM_DevicePreparation {

/// 
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,

/// 
    #[serde(rename = "PageEnabled")]
    pub page_enabled: Option<bool>,

/// 
    #[serde(rename = "PageErrorCode")]
    pub page_error_code: Option<i32>,

/// 
    #[serde(rename = "PageErrorDetails")]
    pub page_error_details: Option<String>,

/// 
    #[serde(rename = "PageErrorPhase")]
    pub page_error_phase: Option<i32>,

/// 
    #[serde(rename = "PageSettings")]
    pub page_settings: Option<String>,

/// 
    #[serde(rename = "PageStatus")]
    pub page_status: Option<i32>,

/// 
    #[serde(rename = "ParentID")]
    pub parent_id: Option<String>,
}

impl MDM_DevicePreparation {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instance_id: None,
            page_enabled: None,
            page_error_code: None,
            page_error_details: None,
            page_error_phase: None,
            page_settings: None,
            page_status: None,
            parent_id: None,
        }
    }


    /// Sets the value of InstanceID
    pub fn set_instance_id(&mut self, value: String) {
        self.instance_id = Some(value);
    }

    /// Gets the value of InstanceID
    pub fn get_instance_id(&self) -> Option<&String> {
        self.instance_id.as_ref()
    }

    /// Sets the value of PageEnabled
    pub fn set_page_enabled(&mut self, value: bool) {
        self.page_enabled = Some(value);
    }

    /// Gets the value of PageEnabled
    pub fn get_page_enabled(&self) -> Option<&bool> {
        self.page_enabled.as_ref()
    }

    /// Sets the value of PageErrorCode
    pub fn set_page_error_code(&mut self, value: i32) {
        self.page_error_code = Some(value);
    }

    /// Gets the value of PageErrorCode
    pub fn get_page_error_code(&self) -> Option<&i32> {
        self.page_error_code.as_ref()
    }

    /// Sets the value of PageErrorDetails
    pub fn set_page_error_details(&mut self, value: String) {
        self.page_error_details = Some(value);
    }

    /// Gets the value of PageErrorDetails
    pub fn get_page_error_details(&self) -> Option<&String> {
        self.page_error_details.as_ref()
    }

    /// Sets the value of PageErrorPhase
    pub fn set_page_error_phase(&mut self, value: i32) {
        self.page_error_phase = Some(value);
    }

    /// Gets the value of PageErrorPhase
    pub fn get_page_error_phase(&self) -> Option<&i32> {
        self.page_error_phase.as_ref()
    }

    /// Sets the value of PageSettings
    pub fn set_page_settings(&mut self, value: String) {
        self.page_settings = Some(value);
    }

    /// Gets the value of PageSettings
    pub fn get_page_settings(&self) -> Option<&String> {
        self.page_settings.as_ref()
    }

    /// Sets the value of PageStatus
    pub fn set_page_status(&mut self, value: i32) {
        self.page_status = Some(value);
    }

    /// Gets the value of PageStatus
    pub fn get_page_status(&self) -> Option<&i32> {
        self.page_status.as_ref()
    }

    /// Sets the value of ParentID
    pub fn set_parent_id(&mut self, value: String) {
        self.parent_id = Some(value);
    }

    /// Gets the value of ParentID
    pub fn get_parent_id(&self) -> Option<&String> {
        self.parent_id.as_ref()
    }
}

