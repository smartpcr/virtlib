// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageJobToAffectedStorageObject struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageJobToAffectedStorageObject {

/// 
    #[serde(rename = "AffectedStorageObject")]
    pub affected_storage_object: Option<MSFT_StorageObject>,

/// 
    #[serde(rename = "StorageJob")]
    pub storage_job: Option<MSFT_StorageJob>,
}

impl MSFT_StorageJobToAffectedStorageObject {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            affected_storage_object: None,
            storage_job: None,
        }
    }


    /// Sets the value of AffectedStorageObject
    pub fn set_affected_storage_object(&mut self, value: MSFT_StorageObject) {
        self.affected_storage_object = Some(value);
    }

    /// Gets the value of AffectedStorageObject
    pub fn get_affected_storage_object(&self) -> Option<&MSFT_StorageObject> {
        self.affected_storage_object.as_ref()
    }

    /// Sets the value of StorageJob
    pub fn set_storage_job(&mut self, value: MSFT_StorageJob) {
        self.storage_job = Some(value);
    }

    /// Gets the value of StorageJob
    pub fn get_storage_job(&self) -> Option<&MSFT_StorageJob> {
        self.storage_job.as_ref()
    }
}

