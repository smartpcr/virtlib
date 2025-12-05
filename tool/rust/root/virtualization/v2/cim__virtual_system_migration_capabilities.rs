// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemMigrationCapabilities struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemMigrationCapabilities {
    #[serde(flatten)]
    pub base: CIM_Capabilities,

/// Enumeration of method identifiers whose implementation may be asynchronous; that is, the operation may not complete immediately and instead the method may return a job.
    #[serde(rename = "AsynchronousMethodsSupported")]
    pub asynchronous_methods_supported: Vec<VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported>,

/// Array of format designators. Values indicate that the designated format is supported for input values of the DestinationHost parameter of the MigrateVirtualSystemToHost( ) method or the CheckVirtualSystemIsMigratableToHost( ) method of the associated instance of the CIM_VirtualSystemMigrationService class.
/// Format designators designate the following formats:
/// 2 - Support of the Domain Name text format according to RFC 1035
/// 3 - Support of the IPv4 dotted decimal format according to RFC 1208
/// 4 - Support of the IPv6 text format according to RFC 4291
    #[serde(rename = "DestinationHostFormatsSupported")]
    pub destination_host_formats_supported: Vec<VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported>,

/// Enumeration of method identifiers whose implementation may be synchronous; that is, the operation may complete immediately and therefore the method may not return a job.
    #[serde(rename = "SynchronousMethodsSupported")]
    pub synchronous_methods_supported: Vec<VirtualSystemMigrationCapabilities_SynchronousMethodsSupported>,
}

impl CIM_VirtualSystemMigrationCapabilities {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Capabilities::new(),
            asynchronous_methods_supported: Vec::new(),
            destination_host_formats_supported: Vec::new(),
            synchronous_methods_supported: Vec::new(),
        }
    }


    /// Sets the value of AsynchronousMethodsSupported
    pub fn set_asynchronous_methods_supported(&mut self, value: Vec<VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported>) {
        self.asynchronous_methods_supported = value;
    }

    /// Gets the value of AsynchronousMethodsSupported
    pub fn get_asynchronous_methods_supported(&self) -> &Vec<VirtualSystemMigrationCapabilities_AsynchronousMethodsSupported> {
        &self.asynchronous_methods_supported
    }

    /// Sets the value of DestinationHostFormatsSupported
    pub fn set_destination_host_formats_supported(&mut self, value: Vec<VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported>) {
        self.destination_host_formats_supported = value;
    }

    /// Gets the value of DestinationHostFormatsSupported
    pub fn get_destination_host_formats_supported(&self) -> &Vec<VirtualSystemMigrationCapabilities_DestinationHostFormatsSupported> {
        &self.destination_host_formats_supported
    }

    /// Sets the value of SynchronousMethodsSupported
    pub fn set_synchronous_methods_supported(&mut self, value: Vec<VirtualSystemMigrationCapabilities_SynchronousMethodsSupported>) {
        self.synchronous_methods_supported = value;
    }

    /// Gets the value of SynchronousMethodsSupported
    pub fn get_synchronous_methods_supported(&self) -> &Vec<VirtualSystemMigrationCapabilities_SynchronousMethodsSupported> {
        &self.synchronous_methods_supported
    }
}

