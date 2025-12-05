// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_TransparentBridgingService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_TransparentBridgingService {
    #[serde(flatten)]
    pub base: CIM_ForwardingService,

/// The timeout period in seconds for aging out dynamically learned forwarding information. 802.1D-1990 recommends a default of 300 seconds.
    #[serde(rename = "AgingTime")]
    pub aging_time: Option<u32>,

/// Filtering Database Identifier used by VLAN-aware switches that have more than one filtering database.
    #[serde(rename = "FID")]
    pub fid: Option<u32>,
}

impl CIM_TransparentBridgingService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ForwardingService::new(),
            aging_time: None,
            fid: None,
        }
    }


    /// Sets the value of AgingTime
    pub fn set_aging_time(&mut self, value: u32) {
        self.aging_time = Some(value);
    }

    /// Gets the value of AgingTime
    pub fn get_aging_time(&self) -> Option<&u32> {
        self.aging_time.as_ref()
    }

    /// Sets the value of FID
    pub fn set_fid(&mut self, value: u32) {
        self.fid = Some(value);
    }

    /// Gets the value of FID
    pub fn get_fid(&self) -> Option<&u32> {
        self.fid.as_ref()
    }
}

