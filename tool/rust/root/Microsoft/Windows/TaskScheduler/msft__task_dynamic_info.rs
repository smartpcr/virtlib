// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_TaskDynamicInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_TaskDynamicInfo {

/// 
    #[serde(rename = "LastRunTime")]
    pub last_run_time: Option<String>,

/// 
    #[serde(rename = "LastTaskResult")]
    pub last_task_result: Option<u32>,

/// 
    #[serde(rename = "NextRunTime")]
    pub next_run_time: Option<String>,

/// 
    #[serde(rename = "NumberOfMissedRuns")]
    pub number_of_missed_runs: Option<u32>,

/// 
    #[serde(rename = "TaskName")]
    pub task_name: Option<String>,

/// 
    #[serde(rename = "TaskPath")]
    pub task_path: Option<String>,
}

impl MSFT_TaskDynamicInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            last_run_time: None,
            last_task_result: None,
            next_run_time: None,
            number_of_missed_runs: None,
            task_name: None,
            task_path: None,
        }
    }


    /// Sets the value of LastRunTime
    pub fn set_last_run_time(&mut self, value: String) {
        self.last_run_time = Some(value);
    }

    /// Gets the value of LastRunTime
    pub fn get_last_run_time(&self) -> Option<&String> {
        self.last_run_time.as_ref()
    }

    /// Sets the value of LastTaskResult
    pub fn set_last_task_result(&mut self, value: u32) {
        self.last_task_result = Some(value);
    }

    /// Gets the value of LastTaskResult
    pub fn get_last_task_result(&self) -> Option<&u32> {
        self.last_task_result.as_ref()
    }

    /// Sets the value of NextRunTime
    pub fn set_next_run_time(&mut self, value: String) {
        self.next_run_time = Some(value);
    }

    /// Gets the value of NextRunTime
    pub fn get_next_run_time(&self) -> Option<&String> {
        self.next_run_time.as_ref()
    }

    /// Sets the value of NumberOfMissedRuns
    pub fn set_number_of_missed_runs(&mut self, value: u32) {
        self.number_of_missed_runs = Some(value);
    }

    /// Gets the value of NumberOfMissedRuns
    pub fn get_number_of_missed_runs(&self) -> Option<&u32> {
        self.number_of_missed_runs.as_ref()
    }

    /// Sets the value of TaskName
    pub fn set_task_name(&mut self, value: String) {
        self.task_name = Some(value);
    }

    /// Gets the value of TaskName
    pub fn get_task_name(&self) -> Option<&String> {
        self.task_name.as_ref()
    }

    /// Sets the value of TaskPath
    pub fn set_task_path(&mut self, value: String) {
        self.task_path = Some(value);
    }

    /// Gets the value of TaskPath
    pub fn get_task_path(&self) -> Option<&String> {
        self.task_path.as_ref()
    }
}

