// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemManagementCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemManagementCapabilities {
    #[serde(flatten)]
    pub base: CIM_EnabledLogicalElementCapabilities,

/// Enumeration of method identifiers each identifying a method of class CIM_VirtualSystemManagementService that is supported asynchronously by the implementation.
    #[serde(rename = "AsynchronousMethodsSupported")]
    pub asynchronous_methods_supported: Vec<VirtualSystemManagementCapabilities_AsynchronousMethodsSupported>,

/// Enumeration of indication identifiers each identifying an indication that is supported by the implementation.
/// VirtualSystemStateChangeIndicationsSupported indicates whether or not the implementation supports notification on state changes of CIM_ComputerSystem instances representing virtual systems.
/// VirtualResourceStateChangeIndicationsSupported indicates whether or not the implementation supports notification on state changes of CIM_LogicalDevice instances representing resources of virtual systems.
/// ConcreteJobStateChangeIndicationsSupported indicates whether or not the implementation supports notification on state changes of CIM_ConcreteJob instances.
    #[serde(rename = "IndicationsSupported")]
    pub indications_supported: Vec<VirtualSystemManagementCapabilities_IndicationsSupported>,

/// Enumeration of method identifiers each identifying a method of class CIM_VirtualSystemManagementService that is supported synchronously by the implementation.
    #[serde(rename = "SynchronousMethodsSupported")]
    pub synchronous_methods_supported: Vec<VirtualSystemManagementCapabilities_SynchronousMethodsSupported>,

/// Enumeration of strings each designating a type of virtual system that the implementation supports.
/// The value of each non-NULL array element shall conform to the format defined for the CIM_VirtualSystemSettingData.VirtualSystemType property.
    #[serde(rename = "VirtualSystemTypesSupported")]
    pub virtual_system_types_supported: Vec<String>,
}

impl CIM_VirtualSystemManagementCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_EnabledLogicalElementCapabilities::new(),
            asynchronous_methods_supported: Vec::new(),
            indications_supported: Vec::new(),
            synchronous_methods_supported: Vec::new(),
            virtual_system_types_supported: Vec::new(),
        }
    }


    /// Sets the value of AsynchronousMethodsSupported
    pub fn set_asynchronous_methods_supported(&mut self, value: Vec<VirtualSystemManagementCapabilities_AsynchronousMethodsSupported>) {
        self.asynchronous_methods_supported = value;
    }

    /// Gets the value of AsynchronousMethodsSupported
    pub fn get_asynchronous_methods_supported(&self) -> &Vec<VirtualSystemManagementCapabilities_AsynchronousMethodsSupported> {
        &self.asynchronous_methods_supported
    }

    /// Sets the value of IndicationsSupported
    pub fn set_indications_supported(&mut self, value: Vec<VirtualSystemManagementCapabilities_IndicationsSupported>) {
        self.indications_supported = value;
    }

    /// Gets the value of IndicationsSupported
    pub fn get_indications_supported(&self) -> &Vec<VirtualSystemManagementCapabilities_IndicationsSupported> {
        &self.indications_supported
    }

    /// Sets the value of SynchronousMethodsSupported
    pub fn set_synchronous_methods_supported(&mut self, value: Vec<VirtualSystemManagementCapabilities_SynchronousMethodsSupported>) {
        self.synchronous_methods_supported = value;
    }

    /// Gets the value of SynchronousMethodsSupported
    pub fn get_synchronous_methods_supported(&self) -> &Vec<VirtualSystemManagementCapabilities_SynchronousMethodsSupported> {
        &self.synchronous_methods_supported
    }

    /// Sets the value of VirtualSystemTypesSupported
    pub fn set_virtual_system_types_supported(&mut self, value: Vec<String>) {
        self.virtual_system_types_supported = value;
    }

    /// Gets the value of VirtualSystemTypesSupported
    pub fn get_virtual_system_types_supported(&self) -> &Vec<String> {
        &self.virtual_system_types_supported
    }
}

