// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_PciExpressSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_PciExpressSettingData {
    #[serde(flatten)]
    pub base: CIM_ResourceAllocationSettingData,

/// 
    #[serde(rename = "AllowDirectTranslatedP2P")]
    pub allow_direct_translated_p2_p: Vec<bool>,

/// 
    #[serde(rename = "NumaAwarePlacement")]
    pub numa_aware_placement: Option<bool>,

/// 
    #[serde(rename = "TargetVtl")]
    pub target_vtl: Option<u8>,

/// 
    #[serde(rename = "VirtualFunctions")]
    pub virtual_functions: Vec<u16>,

/// 
    #[serde(rename = "VirtualSystemIdentifiers")]
    pub virtual_system_identifiers: Vec<String>,
}

impl Msvm_PciExpressSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ResourceAllocationSettingData::new(),
            allow_direct_translated_p2_p: Vec::new(),
            numa_aware_placement: None,
            target_vtl: None,
            virtual_functions: Vec::new(),
            virtual_system_identifiers: Vec::new(),
        }
    }


    /// Sets the value of AllowDirectTranslatedP2P
    pub fn set_allow_direct_translated_p2_p(&mut self, value: Vec<bool>) {
        self.allow_direct_translated_p2_p = value;
    }

    /// Gets the value of AllowDirectTranslatedP2P
    pub fn get_allow_direct_translated_p2_p(&self) -> &Vec<bool> {
        &self.allow_direct_translated_p2_p
    }

    /// Sets the value of NumaAwarePlacement
    pub fn set_numa_aware_placement(&mut self, value: bool) {
        self.numa_aware_placement = Some(value);
    }

    /// Gets the value of NumaAwarePlacement
    pub fn get_numa_aware_placement(&self) -> Option<&bool> {
        self.numa_aware_placement.as_ref()
    }

    /// Sets the value of TargetVtl
    pub fn set_target_vtl(&mut self, value: u8) {
        self.target_vtl = Some(value);
    }

    /// Gets the value of TargetVtl
    pub fn get_target_vtl(&self) -> Option<&u8> {
        self.target_vtl.as_ref()
    }

    /// Sets the value of VirtualFunctions
    pub fn set_virtual_functions(&mut self, value: Vec<u16>) {
        self.virtual_functions = value;
    }

    /// Gets the value of VirtualFunctions
    pub fn get_virtual_functions(&self) -> &Vec<u16> {
        &self.virtual_functions
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

impl Msvm_PciExpressSettingData {
    /// Gets the related Msvm_AllocationCapabilities object(s)
    pub fn get_related__allocation_capabilities(&self) -> Result<Msvm_AllocationCapabilities, WmiError> {
        self.get_related("Msvm_AllocationCapabilities")
    }

}

