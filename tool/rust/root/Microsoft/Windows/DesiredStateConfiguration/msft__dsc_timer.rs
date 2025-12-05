// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DesiredStateConfiguration
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DscTimer struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DscTimer {
}

impl MSFT_DscTimer {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `consistency_timer_value` -  (u32)
    /// * `refresh_timer_value` -  (u32)

    /// * `return_value` -  (u32)
    pub fn start_dsc_timer(&self, consistency_timer_value: u32, refresh_timer_value: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConsistencyTimerValue".to_string(), value: consistency_timer_value.into() });
        args.push(MethodParameter { name: "RefreshTimerValue".to_string(), value: refresh_timer_value.into() });
        self.invoke_method("StartDscTimer", &args)

    }


/// 

    /// * `reporting_timer_value` -  (u32)

    /// * `return_value` -  (u32)
    pub fn start_dsc_reporting_timer(&self, reporting_timer_value: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReportingTimerValue".to_string(), value: reporting_timer_value.into() });
        self.invoke_method("StartDscReportingTimer", &args)

    }

}

