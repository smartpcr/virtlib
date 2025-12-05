// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageHealthReport struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageHealthReport {

/// 
    #[serde(rename = "Records")]
    pub records: Vec<MSFT_HealthRecord>,

/// 
    #[serde(rename = "ReportedObjectUniqueId")]
    pub reported_object_unique_id: Option<String>,

/// 
    #[serde(rename = "StorageSubsystemUniqueId")]
    pub storage_subsystem_unique_id: Option<String>,
}

impl MSFT_StorageHealthReport {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            reported_object_unique_id: None,
            storage_subsystem_unique_id: None,
        }
    }


    /// Sets the value of Records
    pub fn set_records(&mut self, value: Vec<MSFT_HealthRecord>) {
        self.records = value;
    }

    /// Gets the value of Records
    pub fn get_records(&self) -> &Vec<MSFT_HealthRecord> {
        &self.records
    }

    /// Sets the value of ReportedObjectUniqueId
    pub fn set_reported_object_unique_id(&mut self, value: String) {
        self.reported_object_unique_id = Some(value);
    }

    /// Gets the value of ReportedObjectUniqueId
    pub fn get_reported_object_unique_id(&self) -> Option<&String> {
        self.reported_object_unique_id.as_ref()
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

