// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_GpuPartitionSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_GpuPartitionSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "MaxPartitionCompute")]
    pub max_partition_compute: Option<u64>,

/// 
    #[serde(rename = "MaxPartitionDecode")]
    pub max_partition_decode: Option<u64>,

/// 
    #[serde(rename = "MaxPartitionEncode")]
    pub max_partition_encode: Option<u64>,

/// 
    #[serde(rename = "MaxPartitionVRAM")]
    pub max_partition_vram: Option<u64>,

/// 
    #[serde(rename = "MinPartitionCompute")]
    pub min_partition_compute: Option<u64>,

/// 
    #[serde(rename = "MinPartitionDecode")]
    pub min_partition_decode: Option<u64>,

/// 
    #[serde(rename = "MinPartitionEncode")]
    pub min_partition_encode: Option<u64>,

/// 
    #[serde(rename = "MinPartitionVRAM")]
    pub min_partition_vram: Option<u64>,

/// 
    #[serde(rename = "NumaAwarePlacement")]
    pub numa_aware_placement: Option<bool>,

/// 
    #[serde(rename = "OptimalPartitionCompute")]
    pub optimal_partition_compute: Option<u64>,

/// 
    #[serde(rename = "OptimalPartitionDecode")]
    pub optimal_partition_decode: Option<u64>,

/// 
    #[serde(rename = "OptimalPartitionEncode")]
    pub optimal_partition_encode: Option<u64>,

/// 
    #[serde(rename = "OptimalPartitionVRAM")]
    pub optimal_partition_vram: Option<u64>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_GpuPartitionSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            max_partition_compute: None,
            max_partition_decode: None,
            max_partition_encode: None,
            max_partition_vram: None,
            min_partition_compute: None,
            min_partition_decode: None,
            min_partition_encode: None,
            min_partition_vram: None,
            numa_aware_placement: None,
            optimal_partition_compute: None,
            optimal_partition_decode: None,
            optimal_partition_encode: None,
            optimal_partition_vram: None,
            virtual_system_identifiers: Vec::new(),
        }
    }


    /// Sets the value of MaxPartitionCompute
    pub fn set_max_partition_compute(&mut self, value: u64) {
        self.max_partition_compute = Some(value);
    }

    /// Gets the value of MaxPartitionCompute
    pub fn get_max_partition_compute(&self) -> Option<&u64> {
        self.max_partition_compute.as_ref()
    }

    /// Sets the value of MaxPartitionDecode
    pub fn set_max_partition_decode(&mut self, value: u64) {
        self.max_partition_decode = Some(value);
    }

    /// Gets the value of MaxPartitionDecode
    pub fn get_max_partition_decode(&self) -> Option<&u64> {
        self.max_partition_decode.as_ref()
    }

    /// Sets the value of MaxPartitionEncode
    pub fn set_max_partition_encode(&mut self, value: u64) {
        self.max_partition_encode = Some(value);
    }

    /// Gets the value of MaxPartitionEncode
    pub fn get_max_partition_encode(&self) -> Option<&u64> {
        self.max_partition_encode.as_ref()
    }

    /// Sets the value of MaxPartitionVRAM
    pub fn set_max_partition_vram(&mut self, value: u64) {
        self.max_partition_vram = Some(value);
    }

    /// Gets the value of MaxPartitionVRAM
    pub fn get_max_partition_vram(&self) -> Option<&u64> {
        self.max_partition_vram.as_ref()
    }

    /// Sets the value of MinPartitionCompute
    pub fn set_min_partition_compute(&mut self, value: u64) {
        self.min_partition_compute = Some(value);
    }

    /// Gets the value of MinPartitionCompute
    pub fn get_min_partition_compute(&self) -> Option<&u64> {
        self.min_partition_compute.as_ref()
    }

    /// Sets the value of MinPartitionDecode
    pub fn set_min_partition_decode(&mut self, value: u64) {
        self.min_partition_decode = Some(value);
    }

    /// Gets the value of MinPartitionDecode
    pub fn get_min_partition_decode(&self) -> Option<&u64> {
        self.min_partition_decode.as_ref()
    }

    /// Sets the value of MinPartitionEncode
    pub fn set_min_partition_encode(&mut self, value: u64) {
        self.min_partition_encode = Some(value);
    }

    /// Gets the value of MinPartitionEncode
    pub fn get_min_partition_encode(&self) -> Option<&u64> {
        self.min_partition_encode.as_ref()
    }

    /// Sets the value of MinPartitionVRAM
    pub fn set_min_partition_vram(&mut self, value: u64) {
        self.min_partition_vram = Some(value);
    }

    /// Gets the value of MinPartitionVRAM
    pub fn get_min_partition_vram(&self) -> Option<&u64> {
        self.min_partition_vram.as_ref()
    }

    /// Sets the value of NumaAwarePlacement
    pub fn set_numa_aware_placement(&mut self, value: bool) {
        self.numa_aware_placement = Some(value);
    }

    /// Gets the value of NumaAwarePlacement
    pub fn get_numa_aware_placement(&self) -> Option<&bool> {
        self.numa_aware_placement.as_ref()
    }

    /// Sets the value of OptimalPartitionCompute
    pub fn set_optimal_partition_compute(&mut self, value: u64) {
        self.optimal_partition_compute = Some(value);
    }

    /// Gets the value of OptimalPartitionCompute
    pub fn get_optimal_partition_compute(&self) -> Option<&u64> {
        self.optimal_partition_compute.as_ref()
    }

    /// Sets the value of OptimalPartitionDecode
    pub fn set_optimal_partition_decode(&mut self, value: u64) {
        self.optimal_partition_decode = Some(value);
    }

    /// Gets the value of OptimalPartitionDecode
    pub fn get_optimal_partition_decode(&self) -> Option<&u64> {
        self.optimal_partition_decode.as_ref()
    }

    /// Sets the value of OptimalPartitionEncode
    pub fn set_optimal_partition_encode(&mut self, value: u64) {
        self.optimal_partition_encode = Some(value);
    }

    /// Gets the value of OptimalPartitionEncode
    pub fn get_optimal_partition_encode(&self) -> Option<&u64> {
        self.optimal_partition_encode.as_ref()
    }

    /// Sets the value of OptimalPartitionVRAM
    pub fn set_optimal_partition_vram(&mut self, value: u64) {
        self.optimal_partition_vram = Some(value);
    }

    /// Gets the value of OptimalPartitionVRAM
    pub fn get_optimal_partition_vram(&self) -> Option<&u64> {
        self.optimal_partition_vram.as_ref()
    }

    /// Sets the value of VirtualSystemIdentifiers
    pub fn set_virtual_system_identifiers(&mut self, value: Vec<String>) {
        self.virtual_system_identifiers = value;
    }

    /// Gets the value of VirtualSystemIdentifiers
    pub fn get_virtual_system_identifiers(&self) -> &Vec<String> {
        &self.virtual_system_identifiers
    }
}

impl Msvm_GpuPartitionSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

