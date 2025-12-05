// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorageSpacesVirtualDiskMap struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorageSpacesVirtualDiskMap {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "VirtualDiskMapBytesAverage")]
    pub virtual_disk_map_bytes_average: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMapBytesAverage_Base")]
    pub virtual_disk_map_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapBytesPersec")]
    pub virtual_disk_map_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMapExtentLatencyms")]
    pub virtual_disk_map_extent_latencyms: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapExtentLatencyms_Base")]
    pub virtual_disk_map_extent_latencyms__base: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapExtentsAverage")]
    pub virtual_disk_map_extents_average: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMapExtentsAverage_Base")]
    pub virtual_disk_map_extents_average__base: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapExtentsPersec")]
    pub virtual_disk_map_extents_persec: Option<u64>,

/// 
    #[serde(rename = "VirtualDiskMapLatencyms")]
    pub virtual_disk_map_latencyms: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapLatencyms_Base")]
    pub virtual_disk_map_latencyms__base: Option<u32>,

/// 
    #[serde(rename = "VirtualDiskMapsPersec")]
    pub virtual_disk_maps_persec: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorageSpacesVirtualDiskMap {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            virtual_disk_map_bytes_average: None,
            virtual_disk_map_bytes_average__base: None,
            virtual_disk_map_bytes_persec: None,
            virtual_disk_map_extent_latencyms: None,
            virtual_disk_map_extent_latencyms__base: None,
            virtual_disk_map_extents_average: None,
            virtual_disk_map_extents_average__base: None,
            virtual_disk_map_extents_persec: None,
            virtual_disk_map_latencyms: None,
            virtual_disk_map_latencyms__base: None,
            virtual_disk_maps_persec: None,
        }
    }


    /// Sets the value of VirtualDiskMapBytesAverage
    pub fn set_virtual_disk_map_bytes_average(&mut self, value: u64) {
        self.virtual_disk_map_bytes_average = Some(value);
    }

    /// Gets the value of VirtualDiskMapBytesAverage
    pub fn get_virtual_disk_map_bytes_average(&self) -> Option<&u64> {
        self.virtual_disk_map_bytes_average.as_ref()
    }

    /// Sets the value of VirtualDiskMapBytesAverage_Base
    pub fn set_virtual_disk_map_bytes_average__base(&mut self, value: u32) {
        self.virtual_disk_map_bytes_average__base = Some(value);
    }

    /// Gets the value of VirtualDiskMapBytesAverage_Base
    pub fn get_virtual_disk_map_bytes_average__base(&self) -> Option<&u32> {
        self.virtual_disk_map_bytes_average__base.as_ref()
    }

    /// Sets the value of VirtualDiskMapBytesPersec
    pub fn set_virtual_disk_map_bytes_persec(&mut self, value: u64) {
        self.virtual_disk_map_bytes_persec = Some(value);
    }

    /// Gets the value of VirtualDiskMapBytesPersec
    pub fn get_virtual_disk_map_bytes_persec(&self) -> Option<&u64> {
        self.virtual_disk_map_bytes_persec.as_ref()
    }

    /// Sets the value of VirtualDiskMapExtentLatencyms
    pub fn set_virtual_disk_map_extent_latencyms(&mut self, value: u32) {
        self.virtual_disk_map_extent_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskMapExtentLatencyms
    pub fn get_virtual_disk_map_extent_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_map_extent_latencyms.as_ref()
    }

    /// Sets the value of VirtualDiskMapExtentLatencyms_Base
    pub fn set_virtual_disk_map_extent_latencyms__base(&mut self, value: u32) {
        self.virtual_disk_map_extent_latencyms__base = Some(value);
    }

    /// Gets the value of VirtualDiskMapExtentLatencyms_Base
    pub fn get_virtual_disk_map_extent_latencyms__base(&self) -> Option<&u32> {
        self.virtual_disk_map_extent_latencyms__base.as_ref()
    }

    /// Sets the value of VirtualDiskMapExtentsAverage
    pub fn set_virtual_disk_map_extents_average(&mut self, value: u64) {
        self.virtual_disk_map_extents_average = Some(value);
    }

    /// Gets the value of VirtualDiskMapExtentsAverage
    pub fn get_virtual_disk_map_extents_average(&self) -> Option<&u64> {
        self.virtual_disk_map_extents_average.as_ref()
    }

    /// Sets the value of VirtualDiskMapExtentsAverage_Base
    pub fn set_virtual_disk_map_extents_average__base(&mut self, value: u32) {
        self.virtual_disk_map_extents_average__base = Some(value);
    }

    /// Gets the value of VirtualDiskMapExtentsAverage_Base
    pub fn get_virtual_disk_map_extents_average__base(&self) -> Option<&u32> {
        self.virtual_disk_map_extents_average__base.as_ref()
    }

    /// Sets the value of VirtualDiskMapExtentsPersec
    pub fn set_virtual_disk_map_extents_persec(&mut self, value: u64) {
        self.virtual_disk_map_extents_persec = Some(value);
    }

    /// Gets the value of VirtualDiskMapExtentsPersec
    pub fn get_virtual_disk_map_extents_persec(&self) -> Option<&u64> {
        self.virtual_disk_map_extents_persec.as_ref()
    }

    /// Sets the value of VirtualDiskMapLatencyms
    pub fn set_virtual_disk_map_latencyms(&mut self, value: u32) {
        self.virtual_disk_map_latencyms = Some(value);
    }

    /// Gets the value of VirtualDiskMapLatencyms
    pub fn get_virtual_disk_map_latencyms(&self) -> Option<&u32> {
        self.virtual_disk_map_latencyms.as_ref()
    }

    /// Sets the value of VirtualDiskMapLatencyms_Base
    pub fn set_virtual_disk_map_latencyms__base(&mut self, value: u32) {
        self.virtual_disk_map_latencyms__base = Some(value);
    }

    /// Gets the value of VirtualDiskMapLatencyms_Base
    pub fn get_virtual_disk_map_latencyms__base(&self) -> Option<&u32> {
        self.virtual_disk_map_latencyms__base.as_ref()
    }

    /// Sets the value of VirtualDiskMapsPersec
    pub fn set_virtual_disk_maps_persec(&mut self, value: u64) {
        self.virtual_disk_maps_persec = Some(value);
    }

    /// Gets the value of VirtualDiskMapsPersec
    pub fn get_virtual_disk_maps_persec(&self) -> Option<&u64> {
        self.virtual_disk_maps_persec.as_ref()
    }
}

