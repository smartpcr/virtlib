// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ResourcePoolConfigurationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ResourcePoolConfigurationCapabilities {
    #[serde(flatten)]
    pub base: CIM_Capabilities,

/// This property reflects the methods of the associated service class that are supported that may return a Job.
    #[serde(rename = "AsynchronousMethodsSupported")]
    pub asynchronous_methods_supported: Vec<ResourcePoolConfigurationCapabilities_AsynchronousMethodsSupported>,

/// This property reflects the methods of the associated service class that are supported and block until completed (no Job is returned.)
    #[serde(rename = "SynchronousMethodsSupported")]
    pub synchronous_methods_supported: Vec<ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported>,
}

impl Msvm_ResourcePoolConfigurationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Capabilities::new(),
            asynchronous_methods_supported: Vec::new(),
            synchronous_methods_supported: Vec::new(),
        }
    }


    /// Sets the value of AsynchronousMethodsSupported
    pub fn set_asynchronous_methods_supported(&mut self, value: Vec<ResourcePoolConfigurationCapabilities_AsynchronousMethodsSupported>) {
        self.asynchronous_methods_supported = value;
    }

    /// Gets the value of AsynchronousMethodsSupported
    pub fn get_asynchronous_methods_supported(&self) -> &Vec<ResourcePoolConfigurationCapabilities_AsynchronousMethodsSupported> {
        &self.asynchronous_methods_supported
    }

    /// Sets the value of SynchronousMethodsSupported
    pub fn set_synchronous_methods_supported(&mut self, value: Vec<ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported>) {
        self.synchronous_methods_supported = value;
    }

    /// Gets the value of SynchronousMethodsSupported
    pub fn get_synchronous_methods_supported(&self) -> &Vec<ResourcePoolConfigurationCapabilities_SynchronousMethodsSupported> {
        &self.synchronous_methods_supported
    }
}

impl Msvm_ResourcePoolConfigurationCapabilities {
    /// Gets the related Msvm_ResourcePoolConfigurationService object(s)
    pub fn get_related__resource_pool_configuration_service(&self) -> Result<Msvm_ResourcePoolConfigurationService, WmiError> {
        self.get_related("Msvm_ResourcePoolConfigurationService")
    }

}

