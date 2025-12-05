// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskComHandlerAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskComHandlerAction {
    #[serde(flatten)]
    pub base: MSFT_TaskAction,

/// 
    #[serde(rename = "ClassId")]
    pub class_id: Option<String>,

/// 
    #[serde(rename = "Data")]
    pub data: Option<String>,
}

impl MSFT_TaskComHandlerAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskAction::new(),
            class_id: None,
            data: None,
        }
    }


    /// Sets the value of ClassId
    pub fn set_class_id(&mut self, value: String) {
        self.class_id = Some(value);
    }

    /// Gets the value of ClassId
    pub fn get_class_id(&self) -> Option<&String> {
        self.class_id.as_ref()
    }

    /// Sets the value of Data
    pub fn set_data(&mut self, value: String) {
        self.data = Some(value);
    }

    /// Gets the value of Data
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }
}

