// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_HealthActionEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_HealthActionEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageEvent,

/// 
    #[serde(rename = "ChangeType")]
    pub change_type: Option<u16>,

/// 
    #[serde(rename = "HealthActionId")]
    pub health_action_id: Option<String>,

/// 
    #[serde(rename = "HealthActionType")]
    pub health_action_type: Option<String>,

/// 
    #[serde(rename = "PercentComplete")]
    pub percent_complete: Option<u16>,

/// 
    #[serde(rename = "Reason")]
    pub reason: Option<String>,

/// 
    #[serde(rename = "StartTime")]
    pub start_time: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u16>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<String>,

/// 
    #[serde(rename = "StorageSubsystemUniqueId")]
    pub storage_subsystem_unique_id: Option<String>,
}

impl MSFT_HealthActionEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageEvent::new(),
            change_type: None,
            health_action_id: None,
            health_action_type: None,
            percent_complete: None,
            reason: None,
            start_time: None,
            state: None,
            status: None,
            storage_subsystem_unique_id: None,
        }
    }


    /// Sets the value of ChangeType
    pub fn set_change_type(&mut self, value: u16) {
        self.change_type = Some(value);
    }

    /// Gets the value of ChangeType
    pub fn get_change_type(&self) -> Option<&u16> {
        self.change_type.as_ref()
    }

    /// Sets the value of HealthActionId
    pub fn set_health_action_id(&mut self, value: String) {
        self.health_action_id = Some(value);
    }

    /// Gets the value of HealthActionId
    pub fn get_health_action_id(&self) -> Option<&String> {
        self.health_action_id.as_ref()
    }

    /// Sets the value of HealthActionType
    pub fn set_health_action_type(&mut self, value: String) {
        self.health_action_type = Some(value);
    }

    /// Gets the value of HealthActionType
    pub fn get_health_action_type(&self) -> Option<&String> {
        self.health_action_type.as_ref()
    }

    /// Sets the value of PercentComplete
    pub fn set_percent_complete(&mut self, value: u16) {
        self.percent_complete = Some(value);
    }

    /// Gets the value of PercentComplete
    pub fn get_percent_complete(&self) -> Option<&u16> {
        self.percent_complete.as_ref()
    }

    /// Sets the value of Reason
    pub fn set_reason(&mut self, value: String) {
        self.reason = Some(value);
    }

    /// Gets the value of Reason
    pub fn get_reason(&self) -> Option<&String> {
        self.reason.as_ref()
    }

    /// Sets the value of StartTime
    pub fn set_start_time(&mut self, value: String) {
        self.start_time = Some(value);
    }

    /// Gets the value of StartTime
    pub fn get_start_time(&self) -> Option<&String> {
        self.start_time.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u16) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u16> {
        self.state.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: String) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&String> {
        self.status.as_ref()
    }

    /// Sets the value of StorageSubsystemUniqueId
    pub fn set_storage_subsystem_unique_id(&mut self, value: String) {
        self.storage_subsystem_unique_id = Some(value);
    }

    /// Gets the value of StorageSubsystemUniqueId
    pub fn get_storage_subsystem_unique_id(&self) -> Option<&String> {
        self.storage_subsystem_unique_id.as_ref()
    }
}

