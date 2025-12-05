// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskExecAction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskExecAction {
    #[serde(flatten)]
    pub base: MSFT_TaskAction,

/// 
    #[serde(rename = "Arguments")]
    pub arguments: Option<String>,

/// 
    #[serde(rename = "Execute")]
    pub execute: Option<String>,

/// 
    #[serde(rename = "WorkingDirectory")]
    pub working_directory: Option<String>,
}

impl MSFT_TaskExecAction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_TaskAction::new(),
            arguments: None,
            execute: None,
            working_directory: None,
        }
    }


    /// Sets the value of Arguments
    pub fn set_arguments(&mut self, value: String) {
        self.arguments = Some(value);
    }

    /// Gets the value of Arguments
    pub fn get_arguments(&self) -> Option<&String> {
        self.arguments.as_ref()
    }

    /// Sets the value of Execute
    pub fn set_execute(&mut self, value: String) {
        self.execute = Some(value);
    }

    /// Gets the value of Execute
    pub fn get_execute(&self) -> Option<&String> {
        self.execute.as_ref()
    }

    /// Sets the value of WorkingDirectory
    pub fn set_working_directory(&mut self, value: String) {
        self.working_directory = Some(value);
    }

    /// Gets the value of WorkingDirectory
    pub fn get_working_directory(&self) -> Option<&String> {
        self.working_directory.as_ref()
    }
}

