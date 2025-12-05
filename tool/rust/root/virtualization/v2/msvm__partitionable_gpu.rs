// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_PartitionableGpu struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_PartitionableGpu {
    #[serde(flatten)]
    pub base: CIM_ComputerSystem,

/// 
    #[serde(rename = "AvailableCompute")]
    pub available_compute: Option<u64>,

/// 
    #[serde(rename = "AvailableDecode")]
    pub available_decode: Option<u64>,

/// 
    #[serde(rename = "AvailableEncode")]
    pub available_encode: Option<u64>,

/// 
    #[serde(rename = "AvailableVRAM")]
    pub available_vram: Option<u64>,

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
    #[serde(rename = "PartitionCount")]
    pub partition_count: Option<u16>,

/// 
    #[serde(rename = "SupportsIncomingLiveMigration")]
    pub supports_incoming_live_migration: Option<bool>,

/// 
    #[serde(rename = "TotalCompute")]
    pub total_compute: Option<u64>,

/// 
    #[serde(rename = "TotalDecode")]
    pub total_decode: Option<u64>,

/// 
    #[serde(rename = "TotalEncode")]
    pub total_encode: Option<u64>,

/// 
    #[serde(rename = "TotalVRAM")]
    pub total_vram: Option<u64>,

/// 
    #[serde(rename = "ValidPartitionCounts")]
    pub valid_partition_counts: Vec<u16>,
}

impl Msvm_PartitionableGpu {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ComputerSystem::new(),
            available_compute: None,
            available_decode: None,
            available_encode: None,
            available_vram: None,
            max_partition_compute: None,
            max_partition_decode: None,
            max_partition_encode: None,
            max_partition_vram: None,
            min_partition_compute: None,
            min_partition_decode: None,
            min_partition_encode: None,
            min_partition_vram: None,
            optimal_partition_compute: None,
            optimal_partition_decode: None,
            optimal_partition_encode: None,
            optimal_partition_vram: None,
            partition_count: None,
            supports_incoming_live_migration: None,
            total_compute: None,
            total_decode: None,
            total_encode: None,
            total_vram: None,
            valid_partition_counts: Vec::new(),
        }
    }


    /// Sets the value of AvailableCompute
    pub fn set_available_compute(&mut self, value: u64) {
        self.available_compute = Some(value);
    }

    /// Gets the value of AvailableCompute
    pub fn get_available_compute(&self) -> Option<&u64> {
        self.available_compute.as_ref()
    }

    /// Sets the value of AvailableDecode
    pub fn set_available_decode(&mut self, value: u64) {
        self.available_decode = Some(value);
    }

    /// Gets the value of AvailableDecode
    pub fn get_available_decode(&self) -> Option<&u64> {
        self.available_decode.as_ref()
    }

    /// Sets the value of AvailableEncode
    pub fn set_available_encode(&mut self, value: u64) {
        self.available_encode = Some(value);
    }

    /// Gets the value of AvailableEncode
    pub fn get_available_encode(&self) -> Option<&u64> {
        self.available_encode.as_ref()
    }

    /// Sets the value of AvailableVRAM
    pub fn set_available_vram(&mut self, value: u64) {
        self.available_vram = Some(value);
    }

    /// Gets the value of AvailableVRAM
    pub fn get_available_vram(&self) -> Option<&u64> {
        self.available_vram.as_ref()
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

    /// Sets the value of PartitionCount
    pub fn set_partition_count(&mut self, value: u16) {
        self.partition_count = Some(value);
    }

    /// Gets the value of PartitionCount
    pub fn get_partition_count(&self) -> Option<&u16> {
        self.partition_count.as_ref()
    }

    /// Sets the value of SupportsIncomingLiveMigration
    pub fn set_supports_incoming_live_migration(&mut self, value: bool) {
        self.supports_incoming_live_migration = Some(value);
    }

    /// Gets the value of SupportsIncomingLiveMigration
    pub fn get_supports_incoming_live_migration(&self) -> Option<&bool> {
        self.supports_incoming_live_migration.as_ref()
    }

    /// Sets the value of TotalCompute
    pub fn set_total_compute(&mut self, value: u64) {
        self.total_compute = Some(value);
    }

    /// Gets the value of TotalCompute
    pub fn get_total_compute(&self) -> Option<&u64> {
        self.total_compute.as_ref()
    }

    /// Sets the value of TotalDecode
    pub fn set_total_decode(&mut self, value: u64) {
        self.total_decode = Some(value);
    }

    /// Gets the value of TotalDecode
    pub fn get_total_decode(&self) -> Option<&u64> {
        self.total_decode.as_ref()
    }

    /// Sets the value of TotalEncode
    pub fn set_total_encode(&mut self, value: u64) {
        self.total_encode = Some(value);
    }

    /// Gets the value of TotalEncode
    pub fn get_total_encode(&self) -> Option<&u64> {
        self.total_encode.as_ref()
    }

    /// Sets the value of TotalVRAM
    pub fn set_total_vram(&mut self, value: u64) {
        self.total_vram = Some(value);
    }

    /// Gets the value of TotalVRAM
    pub fn get_total_vram(&self) -> Option<&u64> {
        self.total_vram.as_ref()
    }

    /// Sets the value of ValidPartitionCounts
    pub fn set_valid_partition_counts(&mut self, value: Vec<u16>) {
        self.valid_partition_counts = value;
    }

    /// Gets the value of ValidPartitionCounts
    pub fn get_valid_partition_counts(&self) -> &Vec<u16> {
        &self.valid_partition_counts
    }
}

