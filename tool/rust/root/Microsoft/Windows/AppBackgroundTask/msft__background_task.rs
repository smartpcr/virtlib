// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.AppBackgroundTask
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_BackgroundTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_BackgroundTask {

/// 
    #[serde(rename = "EntryPoint")]
    pub entry_point: Vec<String>,

/// 
    #[serde(rename = "PackageFullName")]
    pub package_full_name: Option<String>,

/// 
    #[serde(rename = "PerfInfo")]
    pub perf_info: Vec<String>,

/// 
    #[serde(rename = "TaskID")]
    pub task_id: Vec<String>,

/// 
    #[serde(rename = "TaskName")]
    pub task_name: Vec<String>,
}

impl MSFT_BackgroundTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            entry_point: Vec::new(),
            package_full_name: None,
            perf_info: Vec::new(),
            task_id: Vec::new(),
            task_name: Vec::new(),
        }
    }


    /// Sets the value of EntryPoint
    pub fn set_entry_point(&mut self, value: Vec<String>) {
        self.entry_point = value;
    }

    /// Gets the value of EntryPoint
    pub fn get_entry_point(&self) -> &Vec<String> {
        &self.entry_point
    }

    /// Sets the value of PackageFullName
    pub fn set_package_full_name(&mut self, value: String) {
        self.package_full_name = Some(value);
    }

    /// Gets the value of PackageFullName
    pub fn get_package_full_name(&self) -> Option<&String> {
        self.package_full_name.as_ref()
    }

    /// Sets the value of PerfInfo
    pub fn set_perf_info(&mut self, value: Vec<String>) {
        self.perf_info = value;
    }

    /// Gets the value of PerfInfo
    pub fn get_perf_info(&self) -> &Vec<String> {
        &self.perf_info
    }

    /// Sets the value of TaskID
    pub fn set_task_id(&mut self, value: Vec<String>) {
        self.task_id = value;
    }

    /// Gets the value of TaskID
    pub fn get_task_id(&self) -> &Vec<String> {
        &self.task_id
    }

    /// Sets the value of TaskName
    pub fn set_task_name(&mut self, value: Vec<String>) {
        self.task_name = value;
    }

    /// Gets the value of TaskName
    pub fn get_task_name(&self) -> &Vec<String> {
        &self.task_name
    }
}

