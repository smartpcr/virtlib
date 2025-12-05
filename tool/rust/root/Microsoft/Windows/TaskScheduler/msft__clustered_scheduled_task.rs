// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.TaskScheduler
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ClusteredScheduledTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ClusteredScheduledTask {

/// 
    #[serde(rename = "ClusterName")]
    pub cluster_name: Option<String>,

/// 
    #[serde(rename = "CurrentOwner")]
    pub current_owner: Option<String>,

/// 
    #[serde(rename = "Resource")]
    pub resource: Option<String>,

/// 
    #[serde(rename = "TaskDefinition")]
    pub task_definition: Option<MSFT_ScheduledTask>,

/// 
    #[serde(rename = "TaskName")]
    pub task_name: Option<String>,

/// 
    #[serde(rename = "TaskType")]
    pub task_type: Option<u32>,
}

impl MSFT_ClusteredScheduledTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            cluster_name: None,
            current_owner: None,
            resource: None,
            task_definition: None,
            task_name: None,
            task_type: None,
        }
    }


    /// Sets the value of ClusterName
    pub fn set_cluster_name(&mut self, value: String) {
        self.cluster_name = Some(value);
    }

    /// Gets the value of ClusterName
    pub fn get_cluster_name(&self) -> Option<&String> {
        self.cluster_name.as_ref()
    }

    /// Sets the value of CurrentOwner
    pub fn set_current_owner(&mut self, value: String) {
        self.current_owner = Some(value);
    }

    /// Gets the value of CurrentOwner
    pub fn get_current_owner(&self) -> Option<&String> {
        self.current_owner.as_ref()
    }

    /// Sets the value of Resource
    pub fn set_resource(&mut self, value: String) {
        self.resource = Some(value);
    }

    /// Gets the value of Resource
    pub fn get_resource(&self) -> Option<&String> {
        self.resource.as_ref()
    }

    /// Sets the value of TaskDefinition
    pub fn set_task_definition(&mut self, value: MSFT_ScheduledTask) {
        self.task_definition = Some(value);
    }

    /// Gets the value of TaskDefinition
    pub fn get_task_definition(&self) -> Option<&MSFT_ScheduledTask> {
        self.task_definition.as_ref()
    }

    /// Sets the value of TaskName
    pub fn set_task_name(&mut self, value: String) {
        self.task_name = Some(value);
    }

    /// Gets the value of TaskName
    pub fn get_task_name(&self) -> Option<&String> {
        self.task_name.as_ref()
    }

    /// Sets the value of TaskType
    pub fn set_task_type(&mut self, value: u32) {
        self.task_type = Some(value);
    }

    /// Gets the value of TaskType
    pub fn get_task_type(&self) -> Option<&u32> {
        self.task_type.as_ref()
    }
}

