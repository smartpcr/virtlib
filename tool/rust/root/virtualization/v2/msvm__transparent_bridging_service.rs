// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_TransparentBridgingService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_TransparentBridgingService {
    #[serde(flatten)]
    pub base: CIM_TransparentBridgingService,
}

impl Msvm_TransparentBridgingService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_TransparentBridgingService::new(),
        }
    }

}

impl Msvm_TransparentBridgingService {
    /// Gets the related Msvm_DynamicForwardingEntry object(s)
    pub fn get_related__dynamic_forwarding_entry(&self) -> Result<Vec<Msvm_DynamicForwardingEntry>, WmiError> {
        self.get_all_related("Msvm_DynamicForwardingEntry")
    }

    /// Gets the related Msvm_VirtualEthernetSwitch object(s)
    pub fn get_related__virtual_ethernet_switch(&self) -> Result<Msvm_VirtualEthernetSwitch, WmiError> {
        self.get_related("Msvm_VirtualEthernetSwitch")
    }

}

