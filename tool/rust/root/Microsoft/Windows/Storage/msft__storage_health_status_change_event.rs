// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageHealthStatusChangeEvent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageHealthStatusChangeEvent {
    #[serde(flatten)]
    pub base: MSFT_StorageEvent,

/// 
    #[serde(rename = "CurrentHealthStatus")]
    pub current_health_status: Option<u16>,

/// 
    #[serde(rename = "PreviousHealthStatus")]
    pub previous_health_status: Option<u16>,

/// 
    #[serde(rename = "SourceUniqueId")]
    pub source_unique_id: Option<String>,

/// 
    #[serde(rename = "StorageSubsystemUniqueId")]
    pub storage_subsystem_unique_id: Option<String>,
}

impl MSFT_StorageHealthStatusChangeEvent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageEvent::new(),
            current_health_status: None,
            previous_health_status: None,
            source_unique_id: None,
            storage_subsystem_unique_id: None,
        }
    }


    /// Sets the value of CurrentHealthStatus
    pub fn set_current_health_status(&mut self, value: u16) {
        self.current_health_status = Some(value);
    }

    /// Gets the value of CurrentHealthStatus
    pub fn get_current_health_status(&self) -> Option<&u16> {
        self.current_health_status.as_ref()
    }

    /// Sets the value of PreviousHealthStatus
    pub fn set_previous_health_status(&mut self, value: u16) {
        self.previous_health_status = Some(value);
    }

    /// Gets the value of PreviousHealthStatus
    pub fn get_previous_health_status(&self) -> Option<&u16> {
        self.previous_health_status.as_ref()
    }

    /// Sets the value of SourceUniqueId
    pub fn set_source_unique_id(&mut self, value: String) {
        self.source_unique_id = Some(value);
    }

    /// Gets the value of SourceUniqueId
    pub fn get_source_unique_id(&self) -> Option<&String> {
        self.source_unique_id.as_ref()
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

