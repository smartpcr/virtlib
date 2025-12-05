// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.ServiceModel
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MetadataExporter struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetadataExporter {

/// The policy version used in metadata retrieved from the service.
    #[serde(rename = "PolicyVersion")]
    pub policy_version: Option<String>,
}

impl MetadataExporter {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            policy_version: None,
        }
    }


    /// Sets the value of PolicyVersion
    pub fn set_policy_version(&mut self, value: String) {
        self.policy_version = Some(value);
    }

    /// Gets the value of PolicyVersion
    pub fn get_policy_version(&self) -> Option<&String> {
        self.policy_version.as_ref()
    }
}

