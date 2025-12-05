// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskSettings2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskSettings2 {
    #[serde(flatten)]
    pub base: MSFT_TaskSettings,

/// 
    #[serde(rename = "DisallowStartOnRemoteAppSession")]
    pub disallow_start_on_remote_app_session: Option<bool>,

/// 
    #[serde(rename = "UseUnifiedSchedulingEngine")]
    pub use_unified_scheduling_engine: Option<bool>,
}

impl MSFT_TaskSettings2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskSettings::new(),
            disallow_start_on_remote_app_session: None,
            use_unified_scheduling_engine: None,
        }
    }


    /// Sets the value of DisallowStartOnRemoteAppSession
    pub fn set_disallow_start_on_remote_app_session(&mut self, value: bool) {
        self.disallow_start_on_remote_app_session = Some(value);
    }

    /// Gets the value of DisallowStartOnRemoteAppSession
    pub fn get_disallow_start_on_remote_app_session(&self) -> Option<&bool> {
        self.disallow_start_on_remote_app_session.as_ref()
    }

    /// Sets the value of UseUnifiedSchedulingEngine
    pub fn set_use_unified_scheduling_engine(&mut self, value: bool) {
        self.use_unified_scheduling_engine = Some(value);
    }

    /// Gets the value of UseUnifiedSchedulingEngine
    pub fn get_use_unified_scheduling_engine(&self) -> Option<&bool> {
        self.use_unified_scheduling_engine.as_ref()
    }
}

