// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_RelatedStatistics struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_RelatedStatistics {

/// 
    #[serde(rename = "RelatedStats")]
    pub related_stats: Option<CIM_StatisticalInformation>,

/// 
    #[serde(rename = "Stats")]
    pub stats: Option<CIM_StatisticalInformation>,
}

impl CIM_RelatedStatistics {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            related_stats: None,
            stats: None,
        }
    }


    /// Sets the value of RelatedStats
    pub fn set_related_stats(&mut self, value: CIM_StatisticalInformation) {
        self.related_stats = Some(value);
    }

    /// Gets the value of RelatedStats
    pub fn get_related_stats(&self) -> Option<&CIM_StatisticalInformation> {
        self.related_stats.as_ref()
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

