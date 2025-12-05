// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_ReFSDedupPerfCounters struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_ReFSDedupPerfCounters {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ClustersduplicatedPersecond")]
    pub clustersduplicated_persecond: Option<u32>,

/// 
    #[serde(rename = "ClustersprocessedPersecond")]
    pub clustersprocessed_persecond: Option<u32>,

/// 
    #[serde(rename = "TableinsertioncountPersecond")]
    pub tableinsertioncount_persecond: Option<u32>,
}

impl Win32_PerfFormattedData_Counters_ReFSDedupPerfCounters {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            clustersduplicated_persecond: None,
            clustersprocessed_persecond: None,
            tableinsertioncount_persecond: None,
        }
    }


    /// Sets the value of ClustersduplicatedPersecond
    pub fn set_clustersduplicated_persecond(&mut self, value: u32) {
        self.clustersduplicated_persecond = Some(value);
    }

    /// Gets the value of ClustersduplicatedPersecond
    pub fn get_clustersduplicated_persecond(&self) -> Option<&u32> {
        self.clustersduplicated_persecond.as_ref()
    }

    /// Sets the value of ClustersprocessedPersecond
    pub fn set_clustersprocessed_persecond(&mut self, value: u32) {
        self.clustersprocessed_persecond = Some(value);
    }

    /// Gets the value of ClustersprocessedPersecond
    pub fn get_clustersprocessed_persecond(&self) -> Option<&u32> {
        self.clustersprocessed_persecond.as_ref()
    }

    /// Sets the value of TableinsertioncountPersecond
    pub fn set_tableinsertioncount_persecond(&mut self, value: u32) {
        self.tableinsertioncount_persecond = Some(value);
    }

    /// Gets the value of TableinsertioncountPersecond
    pub fn get_tableinsertioncount_persecond(&self) -> Option<&u32> {
        self.tableinsertioncount_persecond.as_ref()
    }
}

