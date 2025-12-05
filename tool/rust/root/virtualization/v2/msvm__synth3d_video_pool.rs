// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_Synth3dVideoPool struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_Synth3dVideoPool {
    #[serde(flatten)]
    pub base: CIM_ResourcePool,

/// 
    #[serde(rename = "DirectXVersion")]
    pub direct_xversion: Option<String>,

/// 
    #[serde(rename = "Is3dVideoSupported")]
    pub is3d_video_supported: Option<bool>,

/// 
    #[serde(rename = "IsGPUCapable")]
    pub is_gpucapable: Option<bool>,

/// 
    #[serde(rename = "IsSLATCapable")]
    pub is_slatcapable: Option<bool>,

/// 
    #[serde(rename = "RequiredMinimumDirectXVersion")]
    pub required_minimum_direct_xversion: Option<String>,
}

impl Msvm_Synth3dVideoPool {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourcePool::new(),
            direct_xversion: None,
            is3d_video_supported: None,
            is_gpucapable: None,
            is_slatcapable: None,
            required_minimum_direct_xversion: None,
        }
    }


    /// Sets the value of DirectXVersion
    pub fn set_direct_xversion(&mut self, value: String) {
        self.direct_xversion = Some(value);
    }

    /// Gets the value of DirectXVersion
    pub fn get_direct_xversion(&self) -> Option<&String> {
        self.direct_xversion.as_ref()
    }

    /// Sets the value of Is3dVideoSupported
    pub fn set_is3d_video_supported(&mut self, value: bool) {
        self.is3d_video_supported = Some(value);
    }

    /// Gets the value of Is3dVideoSupported
    pub fn get_is3d_video_supported(&self) -> Option<&bool> {
        self.is3d_video_supported.as_ref()
    }

    /// Sets the value of IsGPUCapable
    pub fn set_is_gpucapable(&mut self, value: bool) {
        self.is_gpucapable = Some(value);
    }

    /// Gets the value of IsGPUCapable
    pub fn get_is_gpucapable(&self) -> Option<&bool> {
        self.is_gpucapable.as_ref()
    }

    /// Sets the value of IsSLATCapable
    pub fn set_is_slatcapable(&mut self, value: bool) {
        self.is_slatcapable = Some(value);
    }

    /// Gets the value of IsSLATCapable
    pub fn get_is_slatcapable(&self) -> Option<&bool> {
        self.is_slatcapable.as_ref()
    }

    /// Sets the value of RequiredMinimumDirectXVersion
    pub fn set_required_minimum_direct_xversion(&mut self, value: String) {
        self.required_minimum_direct_xversion = Some(value);
    }

    /// Gets the value of RequiredMinimumDirectXVersion
    pub fn get_required_minimum_direct_xversion(&self) -> Option<&String> {
        self.required_minimum_direct_xversion.as_ref()
    }

/// 

    /// * `monitor_resolution` -  (u32)
    /// * `number_of_monitors` -  (u32)

    /// * `required_video_memory` -  (u64)
    /// * `return_value` -  (u32)
    pub fn calculate_video_memory_requirements(&self, monitor_resolution: u32, number_of_monitors: u32, required_video_memory: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "monitorResolution".to_string(), value: monitor_resolution.into() });
        args.push(MethodParameter { name: "numberOfMonitors".to_string(), value: number_of_monitors.into() });

        let result = self.invoke_method("CalculateVideoMemoryRequirements", &args)?;
        let required_video_memory = result.get_value("requiredVideoMemory")?;
        Ok(result.return_value)

    }

}

impl Msvm_Synth3dVideoPool {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_ResourcePoolSettingData object(s)
    pub fn get_related__resource_pool_setting_data(&self) -> Result<Msvm_ResourcePoolSettingData, WmiError> {
        self.get_related("Msvm_ResourcePoolSettingData")
    }

    /// Gets the related Msvm_ResourcePoolConfigurationService object(s)
    pub fn get_related__resource_pool_configuration_service(&self) -> Result<Msvm_ResourcePoolConfigurationService, WmiError> {
        self.get_related("Msvm_ResourcePoolConfigurationService")
    }

    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

