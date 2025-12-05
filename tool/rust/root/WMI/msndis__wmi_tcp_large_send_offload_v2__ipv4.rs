// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSNdis_WmiTcpLargeSendOffloadV2_IPv4 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSNdis_WmiTcpLargeSendOffloadV2_IPv4 {
    #[serde(flatten)]
    pub base: MSNdis,

/// 
    #[serde(rename = "Encapsulation")]
    pub encapsulation: Option<u32>,

/// 
    #[serde(rename = "MaxOffLoadSize")]
    pub max_off_load_size: Option<u32>,

/// 
    #[serde(rename = "MinSegmentCount")]
    pub min_segment_count: Option<u32>,
}

impl MSNdis_WmiTcpLargeSendOffloadV2_IPv4 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSNdis::new(),
            encapsulation: None,
            max_off_load_size: None,
            min_segment_count: None,
        }
    }


    /// Sets the value of Encapsulation
    pub fn set_encapsulation(&mut self, value: u32) {
        self.encapsulation = Some(value);
    }

    /// Gets the value of Encapsulation
    pub fn get_encapsulation(&self) -> Option<&u32> {
        self.encapsulation.as_ref()
    }

    /// Sets the value of MaxOffLoadSize
    pub fn set_max_off_load_size(&mut self, value: u32) {
        self.max_off_load_size = Some(value);
    }

    /// Gets the value of MaxOffLoadSize
    pub fn get_max_off_load_size(&self) -> Option<&u32> {
        self.max_off_load_size.as_ref()
    }

    /// Sets the value of MinSegmentCount
    pub fn set_min_segment_count(&mut self, value: u32) {
        self.min_segment_count = Some(value);
    }

    /// Gets the value of MinSegmentCount
    pub fn get_min_segment_count(&self) -> Option<&u32> {
        self.min_segment_count.as_ref()
    }
}

