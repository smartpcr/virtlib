// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_MemorySettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_MemorySettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "BackingPageSize")]
    pub backing_page_size: Option<u8>,

/// 
    #[serde(rename = "BackingType")]
    pub backing_type: Option<u8>,

/// 
    #[serde(rename = "DynamicMemoryEnabled")]
    pub dynamic_memory_enabled: Option<bool>,

/// 
    #[serde(rename = "EnableColdHint")]
    pub enable_cold_hint: Option<bool>,

/// 
    #[serde(rename = "EnableEpf")]
    pub enable_epf: Option<bool>,

/// 
    #[serde(rename = "EnableHotHint")]
    pub enable_hot_hint: Option<bool>,

/// 
    #[serde(rename = "EnablePrivateCompressionStore")]
    pub enable_private_compression_store: Option<bool>,

/// 
    #[serde(rename = "HugePagesEnabled")]
    pub huge_pages_enabled: Option<bool>,

/// 
    #[serde(rename = "IsVirtualized")]
    pub is_virtualized: Option<bool>,

/// 
    #[serde(rename = "MaxMemoryBlocksPerNumaNode")]
    pub max_memory_blocks_per_numa_node: Option<u64>,

/// 
    #[serde(rename = "MemoryAccessTrackingPolicy")]
    pub memory_access_tracking_policy: Option<u8>,

/// 
    #[serde(rename = "MemoryAccessTrackingState")]
    pub memory_access_tracking_state: Option<u8>,

/// 
    #[serde(rename = "MemoryEncryptionPolicy")]
    pub memory_encryption_policy: Option<u8>,

/// 
    #[serde(rename = "SgxEnabled")]
    pub sgx_enabled: Option<bool>,

/// 
    #[serde(rename = "SgxLaunchControlDefault")]
    pub sgx_launch_control_default: Option<String>,

/// 
    #[serde(rename = "SgxLaunchControlMode")]
    pub sgx_launch_control_mode: Option<u32>,

/// 
    #[serde(rename = "SgxSize")]
    pub sgx_size: Option<u64>,

/// 
    #[serde(rename = "SwapFilesInUse")]
    pub swap_files_in_use: Option<bool>,

/// 
    #[serde(rename = "TargetMemoryBuffer")]
    pub target_memory_buffer: Option<u32>,
}

impl Msvm_MemorySettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            backing_page_size: None,
            backing_type: None,
            dynamic_memory_enabled: None,
            enable_cold_hint: None,
            enable_epf: None,
            enable_hot_hint: None,
            enable_private_compression_store: None,
            huge_pages_enabled: None,
            is_virtualized: None,
            max_memory_blocks_per_numa_node: None,
            memory_access_tracking_policy: None,
            memory_access_tracking_state: None,
            memory_encryption_policy: None,
            sgx_enabled: None,
            sgx_launch_control_default: None,
            sgx_launch_control_mode: None,
            sgx_size: None,
            swap_files_in_use: None,
            target_memory_buffer: None,
        }
    }


    /// Sets the value of BackingPageSize
    pub fn set_backing_page_size(&mut self, value: u8) {
        self.backing_page_size = Some(value);
    }

    /// Gets the value of BackingPageSize
    pub fn get_backing_page_size(&self) -> Option<&u8> {
        self.backing_page_size.as_ref()
    }

    /// Sets the value of BackingType
    pub fn set_backing_type(&mut self, value: u8) {
        self.backing_type = Some(value);
    }

    /// Gets the value of BackingType
    pub fn get_backing_type(&self) -> Option<&u8> {
        self.backing_type.as_ref()
    }

    /// Sets the value of DynamicMemoryEnabled
    pub fn set_dynamic_memory_enabled(&mut self, value: bool) {
        self.dynamic_memory_enabled = Some(value);
    }

    /// Gets the value of DynamicMemoryEnabled
    pub fn get_dynamic_memory_enabled(&self) -> Option<&bool> {
        self.dynamic_memory_enabled.as_ref()
    }

    /// Sets the value of EnableColdHint
    pub fn set_enable_cold_hint(&mut self, value: bool) {
        self.enable_cold_hint = Some(value);
    }

    /// Gets the value of EnableColdHint
    pub fn get_enable_cold_hint(&self) -> Option<&bool> {
        self.enable_cold_hint.as_ref()
    }

    /// Sets the value of EnableEpf
    pub fn set_enable_epf(&mut self, value: bool) {
        self.enable_epf = Some(value);
    }

    /// Gets the value of EnableEpf
    pub fn get_enable_epf(&self) -> Option<&bool> {
        self.enable_epf.as_ref()
    }

    /// Sets the value of EnableHotHint
    pub fn set_enable_hot_hint(&mut self, value: bool) {
        self.enable_hot_hint = Some(value);
    }

    /// Gets the value of EnableHotHint
    pub fn get_enable_hot_hint(&self) -> Option<&bool> {
        self.enable_hot_hint.as_ref()
    }

    /// Sets the value of EnablePrivateCompressionStore
    pub fn set_enable_private_compression_store(&mut self, value: bool) {
        self.enable_private_compression_store = Some(value);
    }

    /// Gets the value of EnablePrivateCompressionStore
    pub fn get_enable_private_compression_store(&self) -> Option<&bool> {
        self.enable_private_compression_store.as_ref()
    }

    /// Sets the value of HugePagesEnabled
    pub fn set_huge_pages_enabled(&mut self, value: bool) {
        self.huge_pages_enabled = Some(value);
    }

    /// Gets the value of HugePagesEnabled
    pub fn get_huge_pages_enabled(&self) -> Option<&bool> {
        self.huge_pages_enabled.as_ref()
    }

    /// Sets the value of IsVirtualized
    pub fn set_is_virtualized(&mut self, value: bool) {
        self.is_virtualized = Some(value);
    }

    /// Gets the value of IsVirtualized
    pub fn get_is_virtualized(&self) -> Option<&bool> {
        self.is_virtualized.as_ref()
    }

    /// Sets the value of MaxMemoryBlocksPerNumaNode
    pub fn set_max_memory_blocks_per_numa_node(&mut self, value: u64) {
        self.max_memory_blocks_per_numa_node = Some(value);
    }

    /// Gets the value of MaxMemoryBlocksPerNumaNode
    pub fn get_max_memory_blocks_per_numa_node(&self) -> Option<&u64> {
        self.max_memory_blocks_per_numa_node.as_ref()
    }

    /// Sets the value of MemoryAccessTrackingPolicy
    pub fn set_memory_access_tracking_policy(&mut self, value: u8) {
        self.memory_access_tracking_policy = Some(value);
    }

    /// Gets the value of MemoryAccessTrackingPolicy
    pub fn get_memory_access_tracking_policy(&self) -> Option<&u8> {
        self.memory_access_tracking_policy.as_ref()
    }

    /// Sets the value of MemoryAccessTrackingState
    pub fn set_memory_access_tracking_state(&mut self, value: u8) {
        self.memory_access_tracking_state = Some(value);
    }

    /// Gets the value of MemoryAccessTrackingState
    pub fn get_memory_access_tracking_state(&self) -> Option<&u8> {
        self.memory_access_tracking_state.as_ref()
    }

    /// Sets the value of MemoryEncryptionPolicy
    pub fn set_memory_encryption_policy(&mut self, value: u8) {
        self.memory_encryption_policy = Some(value);
    }

    /// Gets the value of MemoryEncryptionPolicy
    pub fn get_memory_encryption_policy(&self) -> Option<&u8> {
        self.memory_encryption_policy.as_ref()
    }

    /// Sets the value of SgxEnabled
    pub fn set_sgx_enabled(&mut self, value: bool) {
        self.sgx_enabled = Some(value);
    }

    /// Gets the value of SgxEnabled
    pub fn get_sgx_enabled(&self) -> Option<&bool> {
        self.sgx_enabled.as_ref()
    }

    /// Sets the value of SgxLaunchControlDefault
    pub fn set_sgx_launch_control_default(&mut self, value: String) {
        self.sgx_launch_control_default = Some(value);
    }

    /// Gets the value of SgxLaunchControlDefault
    pub fn get_sgx_launch_control_default(&self) -> Option<&String> {
        self.sgx_launch_control_default.as_ref()
    }

    /// Sets the value of SgxLaunchControlMode
    pub fn set_sgx_launch_control_mode(&mut self, value: u32) {
        self.sgx_launch_control_mode = Some(value);
    }

    /// Gets the value of SgxLaunchControlMode
    pub fn get_sgx_launch_control_mode(&self) -> Option<&u32> {
        self.sgx_launch_control_mode.as_ref()
    }

    /// Sets the value of SgxSize
    pub fn set_sgx_size(&mut self, value: u64) {
        self.sgx_size = Some(value);
    }

    /// Gets the value of SgxSize
    pub fn get_sgx_size(&self) -> Option<&u64> {
        self.sgx_size.as_ref()
    }

    /// Sets the value of SwapFilesInUse
    pub fn set_swap_files_in_use(&mut self, value: bool) {
        self.swap_files_in_use = Some(value);
    }

    /// Gets the value of SwapFilesInUse
    pub fn get_swap_files_in_use(&self) -> Option<&bool> {
        self.swap_files_in_use.as_ref()
    }

    /// Sets the value of TargetMemoryBuffer
    pub fn set_target_memory_buffer(&mut self, value: u32) {
        self.target_memory_buffer = Some(value);
    }

    /// Gets the value of TargetMemoryBuffer
    pub fn get_target_memory_buffer(&self) -> Option<&u32> {
        self.target_memory_buffer.as_ref()
    }
}

impl Msvm_MemorySettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

