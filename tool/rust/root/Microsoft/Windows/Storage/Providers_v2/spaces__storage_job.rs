// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SPACES_StorageJob struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SPACES_StorageJob {
    #[serde(flatten)]
    pub base: MSFT_StorageJob,

/// ObjectId for internal use only.
    #[serde(rename = "UpdatedObjectId")]
    pub updated_object_id: Option<String>,
}

impl SPACES_StorageJob {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageJob::new(),
            updated_object_id: None,
        }
    }


    /// Sets the value of UpdatedObjectId
    pub fn set_updated_object_id(&mut self, value: String) {
        self.updated_object_id = Some(value);
    }

    /// Gets the value of UpdatedObjectId
    pub fn get_updated_object_id(&self) -> Option<&String> {
        self.updated_object_id.as_ref()
    }
}

