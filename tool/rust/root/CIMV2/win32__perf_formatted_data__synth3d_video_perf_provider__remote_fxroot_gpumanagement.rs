// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Synth3dVideoPerfProvider_RemoteFXRootGPUManagement struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Synth3dVideoPerfProvider_RemoteFXRootGPUManagement {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "ResourcesVMsrunningRemoteFX")]
    pub resources_vmsrunning_remote_fx: Option<u64>,

/// 
    #[serde(rename = "VRAMAvailableMBperGPU")]
    pub vramavailable_mbper_gpu: Option<u64>,

/// 
    #[serde(rename = "VRAMReservedPercentperGPU")]
    pub vramreserved_percentper_gpu: Option<u64>,
}

impl Win32_PerfFormattedData_Synth3dVideoPerfProvider_RemoteFXRootGPUManagement {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            resources_vmsrunning_remote_fx: None,
            vramavailable_mbper_gpu: None,
            vramreserved_percentper_gpu: None,
        }
    }


    /// Sets the value of ResourcesVMsrunningRemoteFX
    pub fn set_resources_vmsrunning_remote_fx(&mut self, value: u64) {
        self.resources_vmsrunning_remote_fx = Some(value);
    }

    /// Gets the value of ResourcesVMsrunningRemoteFX
    pub fn get_resources_vmsrunning_remote_fx(&self) -> Option<&u64> {
        self.resources_vmsrunning_remote_fx.as_ref()
    }

    /// Sets the value of VRAMAvailableMBperGPU
    pub fn set_vramavailable_mbper_gpu(&mut self, value: u64) {
        self.vramavailable_mbper_gpu = Some(value);
    }

    /// Gets the value of VRAMAvailableMBperGPU
    pub fn get_vramavailable_mbper_gpu(&self) -> Option<&u64> {
        self.vramavailable_mbper_gpu.as_ref()
    }

    /// Sets the value of VRAMReservedPercentperGPU
    pub fn set_vramreserved_percentper_gpu(&mut self, value: u64) {
        self.vramreserved_percentper_gpu = Some(value);
    }

    /// Gets the value of VRAMReservedPercentperGPU
    pub fn get_vramreserved_percentper_gpu(&self) -> Option<&u64> {
        self.vramreserved_percentper_gpu.as_ref()
    }
}

