// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_OfflineFilesUserConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_OfflineFilesUserConfiguration {

/// 
    #[serde(rename = "AssignedOfflineFiles")]
    pub assigned_offline_files: Vec<String>,

/// 
    #[serde(rename = "IsConfiguredByWMI")]
    pub is_configured_by_wmi: Option<bool>,

/// 
    #[serde(rename = "MakeAvailableOfflineButtonRemoved")]
    pub make_available_offline_button_removed: Option<bool>,

/// 
    #[serde(rename = "WorkOfflineButtonRemoved")]
    pub work_offline_button_removed: Option<bool>,
}

impl Win32_OfflineFilesUserConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            assigned_offline_files: Vec::new(),
            is_configured_by_wmi: None,
            make_available_offline_button_removed: None,
            work_offline_button_removed: None,
        }
    }


    /// Sets the value of AssignedOfflineFiles
    pub fn set_assigned_offline_files(&mut self, value: Vec<String>) {
        self.assigned_offline_files = value;
    }

    /// Gets the value of AssignedOfflineFiles
    pub fn get_assigned_offline_files(&self) -> &Vec<String> {
        &self.assigned_offline_files
    }

    /// Sets the value of IsConfiguredByWMI
    pub fn set_is_configured_by_wmi(&mut self, value: bool) {
        self.is_configured_by_wmi = Some(value);
    }

    /// Gets the value of IsConfiguredByWMI
    pub fn get_is_configured_by_wmi(&self) -> Option<&bool> {
        self.is_configured_by_wmi.as_ref()
    }

    /// Sets the value of MakeAvailableOfflineButtonRemoved
    pub fn set_make_available_offline_button_removed(&mut self, value: bool) {
        self.make_available_offline_button_removed = Some(value);
    }

    /// Gets the value of MakeAvailableOfflineButtonRemoved
    pub fn get_make_available_offline_button_removed(&self) -> Option<&bool> {
        self.make_available_offline_button_removed.as_ref()
    }

    /// Sets the value of WorkOfflineButtonRemoved
    pub fn set_work_offline_button_removed(&mut self, value: bool) {
        self.work_offline_button_removed = Some(value);
    }

    /// Gets the value of WorkOfflineButtonRemoved
    pub fn get_work_offline_button_removed(&self) -> Option<&bool> {
        self.work_offline_button_removed.as_ref()
    }
}

