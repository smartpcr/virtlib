// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_TapeDrive struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_TapeDrive {
    #[serde(flatten)]
    pub base: CIM_MediaAccessDevice,

/// EOTWarningZoneSize indicates the size, in bytes, of the area designated as 'end of tape'. Access in this area generates an 'end of tape' warning.
    #[serde(rename = "EOTWarningZoneSize")]
    pub eotwarning_zone_size: Option<u32>,

/// Maximum partition count for the TapeDrive.
    #[serde(rename = "MaxPartitionCount")]
    pub max_partition_count: Option<u32>,

/// Time in milliseconds to move from the most physically distant point on the tape to the beginning.
    #[serde(rename = "MaxRewindTime")]
    pub max_rewind_time: Option<u64>,

/// Padding indicates the number of bytes inserted between blocks on a tape Media.
    #[serde(rename = "Padding")]
    pub padding: Option<u32>,
}

impl CIM_TapeDrive {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MediaAccessDevice::new(),
            eotwarning_zone_size: None,
            max_partition_count: None,
            max_rewind_time: None,
            padding: None,
        }
    }


    /// Sets the value of EOTWarningZoneSize
    pub fn set_eotwarning_zone_size(&mut self, value: u32) {
        self.eotwarning_zone_size = Some(value);
    }

    /// Gets the value of EOTWarningZoneSize
    pub fn get_eotwarning_zone_size(&self) -> Option<&u32> {
        self.eotwarning_zone_size.as_ref()
    }

    /// Sets the value of MaxPartitionCount
    pub fn set_max_partition_count(&mut self, value: u32) {
        self.max_partition_count = Some(value);
    }

    /// Gets the value of MaxPartitionCount
    pub fn get_max_partition_count(&self) -> Option<&u32> {
        self.max_partition_count.as_ref()
    }

    /// Sets the value of MaxRewindTime
    pub fn set_max_rewind_time(&mut self, value: u64) {
        self.max_rewind_time = Some(value);
    }

    /// Gets the value of MaxRewindTime
    pub fn get_max_rewind_time(&self) -> Option<&u64> {
        self.max_rewind_time.as_ref()
    }

    /// Sets the value of Padding
    pub fn set_padding(&mut self, value: u32) {
        self.padding = Some(value);
    }

    /// Gets the value of Padding
    pub fn get_padding(&self) -> Option<&u32> {
        self.padding.as_ref()
    }
}

