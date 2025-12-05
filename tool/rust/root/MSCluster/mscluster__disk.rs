// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.MSCluster
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSCluster_Disk struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSCluster_Disk {
    #[serde(flatten)]
    pub base: MSCluster_ClusterDisk,

/// 
    #[serde(rename = "UniqueId")]
    pub unique_id: Option<String>,

/// 
    #[serde(rename = "UniqueIdFormat")]
    pub unique_id_format: Option<u16>,
}

impl MSCluster_Disk {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSCluster_ClusterDisk::new(),
            unique_id: None,
            unique_id_format: None,
        }
    }


    /// Sets the value of UniqueId
    pub fn set_unique_id(&mut self, value: String) {
        self.unique_id = Some(value);
    }

    /// Gets the value of UniqueId
    pub fn get_unique_id(&self) -> Option<&String> {
        self.unique_id.as_ref()
    }

    /// Sets the value of UniqueIdFormat
    pub fn set_unique_id_format(&mut self, value: u16) {
        self.unique_id_format = Some(value);
    }

    /// Gets the value of UniqueIdFormat
    pub fn get_unique_id_format(&self) -> Option<&u16> {
        self.unique_id_format.as_ref()
    }
}

