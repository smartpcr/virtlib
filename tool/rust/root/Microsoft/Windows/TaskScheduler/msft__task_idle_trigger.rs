// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskIdleTrigger struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskIdleTrigger {
    #[serde(flatten)]
    pub base: MSFT_TaskTrigger,
}

impl MSFT_TaskIdleTrigger {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskTrigger::new(),
        }
    }

}

