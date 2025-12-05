// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskSettings3 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskSettings3 {
    #[serde(flatten)]
    pub base: MSFT_TaskSettings2,

/// 
    #[serde(rename = "MaintenanceSettings")]
    pub maintenance_settings: Option<MSFT_TaskMaintenanceSettings>,

/// 
    #[serde(rename = "volatile")]
    pub volatile: Option<bool>,
}

impl MSFT_TaskSettings3 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskSettings2::new(),
            maintenance_settings: None,
            volatile: None,
        }
    }


    /// Sets the value of MaintenanceSettings
    pub fn set_maintenance_settings(&mut self, value: MSFT_TaskMaintenanceSettings) {
        self.maintenance_settings = Some(value);
    }

    /// Gets the value of MaintenanceSettings
    pub fn get_maintenance_settings(&self) -> Option<&MSFT_TaskMaintenanceSettings> {
        self.maintenance_settings.as_ref()
    }

    /// Sets the value of volatile
    pub fn set_volatile(&mut self, value: bool) {
        self.volatile = Some(value);
    }

    /// Gets the value of volatile
    pub fn get_volatile(&self) -> Option<&bool> {
        self.volatile.as_ref()
    }
}

