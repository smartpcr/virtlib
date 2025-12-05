// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Physical3dGraphicsProcessor struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Physical3dGraphicsProcessor {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,

/// 
    #[serde(rename = "AdapterIndexID")]
    pub adapter_index_id: Option<u64>,

/// 
    #[serde(rename = "AvailableVideoMemory")]
    pub available_video_memory: Option<u64>,

/// 
    #[serde(rename = "CompatibleForVirtualization")]
    pub compatible_for_virtualization: Option<bool>,

/// 
    #[serde(rename = "DedicatedSystemMemory")]
    pub dedicated_system_memory: Option<u64>,

/// 
    #[serde(rename = "DedicatedVideoMemory")]
    pub dedicated_video_memory: Option<u64>,

/// 
    #[serde(rename = "DirectXVersion")]
    pub direct_xversion: Option<String>,

/// 
    #[serde(rename = "DriverDate")]
    pub driver_date: Option<String>,

/// 
    #[serde(rename = "DriverInstalled")]
    pub driver_installed: Option<String>,

/// 
    #[serde(rename = "DriverModelVersion")]
    pub driver_model_version: Option<String>,

/// 
    #[serde(rename = "DriverProvider")]
    pub driver_provider: Option<String>,

/// 
    #[serde(rename = "DriverVersion")]
    pub driver_version: Option<String>,

/// 
    #[serde(rename = "EnabledForVirtualization")]
    pub enabled_for_virtualization: Option<bool>,

/// 
    #[serde(rename = "GPUID")]
    pub gpuid: Option<String>,

/// 
    #[serde(rename = "PixelShaderVersion")]
    pub pixel_shader_version: Option<String>,

/// 
    #[serde(rename = "Rating")]
    pub rating: Option<u64>,

/// 
    #[serde(rename = "SharedSystemMemory")]
    pub shared_system_memory: Option<u64>,

/// 
    #[serde(rename = "TotalVideoMemory")]
    pub total_video_memory: Option<u64>,
}

impl Msvm_Physical3dGraphicsProcessor {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
            adapter_index_id: None,
            available_video_memory: None,
            compatible_for_virtualization: None,
            dedicated_system_memory: None,
            dedicated_video_memory: None,
            direct_xversion: None,
            driver_date: None,
            driver_installed: None,
            driver_model_version: None,
            driver_provider: None,
            driver_version: None,
            enabled_for_virtualization: None,
            gpuid: None,
            pixel_shader_version: None,
            rating: None,
            shared_system_memory: None,
            total_video_memory: None,
        }
    }


    /// Sets the value of AdapterIndexID
    pub fn set_adapter_index_id(&mut self, value: u64) {
        self.adapter_index_id = Some(value);
    }

    /// Gets the value of AdapterIndexID
    pub fn get_adapter_index_id(&self) -> Option<&u64> {
        self.adapter_index_id.as_ref()
    }

    /// Sets the value of AvailableVideoMemory
    pub fn set_available_video_memory(&mut self, value: u64) {
        self.available_video_memory = Some(value);
    }

    /// Gets the value of AvailableVideoMemory
    pub fn get_available_video_memory(&self) -> Option<&u64> {
        self.available_video_memory.as_ref()
    }

    /// Sets the value of CompatibleForVirtualization
    pub fn set_compatible_for_virtualization(&mut self, value: bool) {
        self.compatible_for_virtualization = Some(value);
    }

    /// Gets the value of CompatibleForVirtualization
    pub fn get_compatible_for_virtualization(&self) -> Option<&bool> {
        self.compatible_for_virtualization.as_ref()
    }

    /// Sets the value of DedicatedSystemMemory
    pub fn set_dedicated_system_memory(&mut self, value: u64) {
        self.dedicated_system_memory = Some(value);
    }

    /// Gets the value of DedicatedSystemMemory
    pub fn get_dedicated_system_memory(&self) -> Option<&u64> {
        self.dedicated_system_memory.as_ref()
    }

    /// Sets the value of DedicatedVideoMemory
    pub fn set_dedicated_video_memory(&mut self, value: u64) {
        self.dedicated_video_memory = Some(value);
    }

    /// Gets the value of DedicatedVideoMemory
    pub fn get_dedicated_video_memory(&self) -> Option<&u64> {
        self.dedicated_video_memory.as_ref()
    }

    /// Sets the value of DirectXVersion
    pub fn set_direct_xversion(&mut self, value: String) {
        self.direct_xversion = Some(value);
    }

    /// Gets the value of DirectXVersion
    pub fn get_direct_xversion(&self) -> Option<&String> {
        self.direct_xversion.as_ref()
    }

    /// Sets the value of DriverDate
    pub fn set_driver_date(&mut self, value: String) {
        self.driver_date = Some(value);
    }

    /// Gets the value of DriverDate
    pub fn get_driver_date(&self) -> Option<&String> {
        self.driver_date.as_ref()
    }

    /// Sets the value of DriverInstalled
    pub fn set_driver_installed(&mut self, value: String) {
        self.driver_installed = Some(value);
    }

    /// Gets the value of DriverInstalled
    pub fn get_driver_installed(&self) -> Option<&String> {
        self.driver_installed.as_ref()
    }

    /// Sets the value of DriverModelVersion
    pub fn set_driver_model_version(&mut self, value: String) {
        self.driver_model_version = Some(value);
    }

    /// Gets the value of DriverModelVersion
    pub fn get_driver_model_version(&self) -> Option<&String> {
        self.driver_model_version.as_ref()
    }

    /// Sets the value of DriverProvider
    pub fn set_driver_provider(&mut self, value: String) {
        self.driver_provider = Some(value);
    }

    /// Gets the value of DriverProvider
    pub fn get_driver_provider(&self) -> Option<&String> {
        self.driver_provider.as_ref()
    }

    /// Sets the value of DriverVersion
    pub fn set_driver_version(&mut self, value: String) {
        self.driver_version = Some(value);
    }

    /// Gets the value of DriverVersion
    pub fn get_driver_version(&self) -> Option<&String> {
        self.driver_version.as_ref()
    }

    /// Sets the value of EnabledForVirtualization
    pub fn set_enabled_for_virtualization(&mut self, value: bool) {
        self.enabled_for_virtualization = Some(value);
    }

    /// Gets the value of EnabledForVirtualization
    pub fn get_enabled_for_virtualization(&self) -> Option<&bool> {
        self.enabled_for_virtualization.as_ref()
    }

    /// Sets the value of GPUID
    pub fn set_gpuid(&mut self, value: String) {
        self.gpuid = Some(value);
    }

    /// Gets the value of GPUID
    pub fn get_gpuid(&self) -> Option<&String> {
        self.gpuid.as_ref()
    }

    /// Sets the value of PixelShaderVersion
    pub fn set_pixel_shader_version(&mut self, value: String) {
        self.pixel_shader_version = Some(value);
    }

    /// Gets the value of PixelShaderVersion
    pub fn get_pixel_shader_version(&self) -> Option<&String> {
        self.pixel_shader_version.as_ref()
    }

    /// Sets the value of Rating
    pub fn set_rating(&mut self, value: u64) {
        self.rating = Some(value);
    }

    /// Gets the value of Rating
    pub fn get_rating(&self) -> Option<&u64> {
        self.rating.as_ref()
    }

    /// Sets the value of SharedSystemMemory
    pub fn set_shared_system_memory(&mut self, value: u64) {
        self.shared_system_memory = Some(value);
    }

    /// Gets the value of SharedSystemMemory
    pub fn get_shared_system_memory(&self) -> Option<&u64> {
        self.shared_system_memory.as_ref()
    }

    /// Sets the value of TotalVideoMemory
    pub fn set_total_video_memory(&mut self, value: u64) {
        self.total_video_memory = Some(value);
    }

    /// Gets the value of TotalVideoMemory
    pub fn get_total_video_memory(&self) -> Option<&u64> {
        self.total_video_memory.as_ref()
    }
}

