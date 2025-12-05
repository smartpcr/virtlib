// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerServiceDetail struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerServiceDetail {

/// 
    #[serde(rename = "DependentServices")]
    pub dependent_services: Vec<String>,

/// 
    #[serde(rename = "Description")]
    pub description: Option<String>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<u64>,

/// 
    #[serde(rename = "IsDelayedAutoStart")]
    pub is_delayed_auto_start: Option<bool>,

/// 
    #[serde(rename = "IsTriggered")]
    pub is_triggered: Option<bool>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "StartupType")]
    pub startup_type: Option<u32>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 
    #[serde(rename = "SupportedControlCodes")]
    pub supported_control_codes: Option<u32>,
}

impl MSFT_ServerServiceDetail {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            dependent_services: Vec::new(),
            description: None,
            display_name: None,
            exit_code: None,
            is_delayed_auto_start: None,
            is_triggered: None,
            name: None,
            startup_type: None,
            status: None,
            supported_control_codes: None,
        }
    }


    /// Sets the value of DependentServices
    pub fn set_dependent_services(&mut self, value: Vec<String>) {
        self.dependent_services = value;
    }

    /// Gets the value of DependentServices
    pub fn get_dependent_services(&self) -> &Vec<String> {
        &self.dependent_services
    }

    /// Sets the value of Description
    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    /// Gets the value of Description
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of ExitCode
    pub fn set_exit_code(&mut self, value: u64) {
        self.exit_code = Some(value);
    }

    /// Gets the value of ExitCode
    pub fn get_exit_code(&self) -> Option<&u64> {
        self.exit_code.as_ref()
    }

    /// Sets the value of IsDelayedAutoStart
    pub fn set_is_delayed_auto_start(&mut self, value: bool) {
        self.is_delayed_auto_start = Some(value);
    }

    /// Gets the value of IsDelayedAutoStart
    pub fn get_is_delayed_auto_start(&self) -> Option<&bool> {
        self.is_delayed_auto_start.as_ref()
    }

    /// Sets the value of IsTriggered
    pub fn set_is_triggered(&mut self, value: bool) {
        self.is_triggered = Some(value);
    }

    /// Gets the value of IsTriggered
    pub fn get_is_triggered(&self) -> Option<&bool> {
        self.is_triggered.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of StartupType
    pub fn set_startup_type(&mut self, value: u32) {
        self.startup_type = Some(value);
    }

    /// Gets the value of StartupType
    pub fn get_startup_type(&self) -> Option<&u32> {
        self.startup_type.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of SupportedControlCodes
    pub fn set_supported_control_codes(&mut self, value: u32) {
        self.supported_control_codes = Some(value);
    }

    /// Gets the value of SupportedControlCodes
    pub fn get_supported_control_codes(&self) -> Option<&u32> {
        self.supported_control_codes.as_ref()
    }
}

