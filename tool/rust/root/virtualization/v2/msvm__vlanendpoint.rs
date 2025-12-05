// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VLANEndpoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VLANEndpoint {
    #[serde(flatten)]
    pub base: CIM_VLANEndpoint,

/// 
    #[serde(rename = "SupportedEndpointModes")]
    pub supported_endpoint_modes: Vec<u16>,
}

impl Msvm_VLANEndpoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_VLANEndpoint::new(),
            supported_endpoint_modes: Vec::new(),
        }
    }


    /// Sets the value of SupportedEndpointModes
    pub fn set_supported_endpoint_modes(&mut self, value: Vec<u16>) {
        self.supported_endpoint_modes = value;
    }

    /// Gets the value of SupportedEndpointModes
    pub fn get_supported_endpoint_modes(&self) -> &Vec<u16> {
        &self.supported_endpoint_modes
    }
}

impl Msvm_VLANEndpoint {
    /// Gets the related Msvm_LANEndpoint object(s)
    pub fn get_related__lanendpoint(&self) -> Result<Msvm_LANEndpoint, WmiError> {
        self.get_related("Msvm_LANEndpoint")
    }

}

