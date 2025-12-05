// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_CollectionStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_CollectionStatistics {

/// 
    #[serde(rename = "Collection")]
    pub collection: Option<CIM_CollectionOfMSEs>,

/// 
    #[serde(rename = "Stats")]
    pub stats: Option<CIM_StatisticalInformation>,
}

impl Win32_CollectionStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collection: None,
            stats: None,
        }
    }


    /// Sets the value of Collection
    pub fn set_collection(&mut self, value: CIM_CollectionOfMSEs) {
        self.collection = Some(value);
    }

    /// Gets the value of Collection
    pub fn get_collection(&self) -> Option<&CIM_CollectionOfMSEs> {
        self.collection.as_ref()
    }

    /// Sets the value of Stats
    pub fn set_stats(&mut self, value: CIM_StatisticalInformation) {
        self.stats = Some(value);
    }

    /// Gets the value of Stats
    pub fn get_stats(&self) -> Option<&CIM_StatisticalInformation> {
        self.stats.as_ref()
    }
}

