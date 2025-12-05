// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_QuotaSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_QuotaSetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "DefaultLimit")]
    pub default_limit: Option<i64>,

/// 
    #[serde(rename = "DefaultWarningLimit")]
    pub default_warning_limit: Option<i64>,

/// 
    #[serde(rename = "ExceededNotification")]
    pub exceeded_notification: Option<bool>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "VolumePath")]
    pub volume_path: Option<String>,

/// 
    #[serde(rename = "WarningExceededNotification")]
    pub warning_exceeded_notification: Option<bool>,
}

impl Win32_QuotaSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            default_limit: None,
            default_warning_limit: None,
            exceeded_notification: None,
            state: None,
            volume_path: None,
            warning_exceeded_notification: None,
        }
    }


    /// Sets the value of DefaultLimit
    pub fn set_default_limit(&mut self, value: i64) {
        self.default_limit = Some(value);
    }

    /// Gets the value of DefaultLimit
    pub fn get_default_limit(&self) -> Option<&i64> {
        self.default_limit.as_ref()
    }

    /// Sets the value of DefaultWarningLimit
    pub fn set_default_warning_limit(&mut self, value: i64) {
        self.default_warning_limit = Some(value);
    }

    /// Gets the value of DefaultWarningLimit
    pub fn get_default_warning_limit(&self) -> Option<&i64> {
        self.default_warning_limit.as_ref()
    }

    /// Sets the value of ExceededNotification
    pub fn set_exceeded_notification(&mut self, value: bool) {
        self.exceeded_notification = Some(value);
    }

    /// Gets the value of ExceededNotification
    pub fn get_exceeded_notification(&self) -> Option<&bool> {
        self.exceeded_notification.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of VolumePath
    pub fn set_volume_path(&mut self, value: String) {
        self.volume_path = Some(value);
    }

    /// Gets the value of VolumePath
    pub fn get_volume_path(&self) -> Option<&String> {
        self.volume_path.as_ref()
    }

    /// Sets the value of WarningExceededNotification
    pub fn set_warning_exceeded_notification(&mut self, value: bool) {
        self.warning_exceeded_notification = Some(value);
    }

    /// Gets the value of WarningExceededNotification
    pub fn get_warning_exceeded_notification(&self) -> Option<&bool> {
        self.warning_exceeded_notification.as_ref()
    }
}

