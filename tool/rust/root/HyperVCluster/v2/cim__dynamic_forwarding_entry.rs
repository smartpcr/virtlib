// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_DynamicForwardingEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_DynamicForwardingEntry {
    #[serde(flatten)]
    pub base: CIM_LogicalElement,

/// CreationClassName indicates the name of the class or the subclass used in the creation of an instance. When used with the other key properties of this class, this property allows all instances of this class and its subclasses to be uniquely identified.
    #[serde(rename = "CreationClassName")]
    pub creation_class_name: Option<String>,

/// The status of the entry.
    #[serde(rename = "DynamicStatus")]
    pub dynamic_status: Option<DynamicForwardingEntry_DynamicStatus>,

/// Unicast MAC address for which the TransparentBridging Service has forwarding and/or filtering information. Note that the MAC address is formatted as twelve hexadecimal digits (e.g., "010203040506"), with each pair representing one of the six octets of the MAC address in "canonical" bit order according to RFC 2469.
    #[serde(rename = "MACAddress")]
    pub macaddress: Option<String>,

/// The scoping Service's CreationClassName.
    #[serde(rename = "ServiceCreationClassName")]
    pub service_creation_class_name: Option<String>,

/// The scoping Service's Name.
    #[serde(rename = "ServiceName")]
    pub service_name: Option<String>,

/// The scoping System's CreationClassName.
    #[serde(rename = "SystemCreationClassName")]
    pub system_creation_class_name: Option<String>,

/// The scoping System's Name.
    #[serde(rename = "SystemName")]
    pub system_name: Option<String>,
}

impl CIM_DynamicForwardingEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalElement::new(),
            creation_class_name: None,
            dynamic_status: None,
            macaddress: None,
            service_creation_class_name: None,
            service_name: None,
            system_creation_class_name: None,
            system_name: None,
        }
    }


    /// Sets the value of CreationClassName
    pub fn set_creation_class_name(&mut self, value: String) {
        self.creation_class_name = Some(value);
    }

    /// Gets the value of CreationClassName
    pub fn get_creation_class_name(&self) -> Option<&String> {
        self.creation_class_name.as_ref()
    }

    /// Sets the value of DynamicStatus
    pub fn set_dynamic_status(&mut self, value: DynamicForwardingEntry_DynamicStatus) {
        self.dynamic_status = Some(value);
    }

    /// Gets the value of DynamicStatus
    pub fn get_dynamic_status(&self) -> Option<&DynamicForwardingEntry_DynamicStatus> {
        self.dynamic_status.as_ref()
    }

    /// Sets the value of MACAddress
    pub fn set_macaddress(&mut self, value: String) {
        self.macaddress = Some(value);
    }

    /// Gets the value of MACAddress
    pub fn get_macaddress(&self) -> Option<&String> {
        self.macaddress.as_ref()
    }

    /// Sets the value of ServiceCreationClassName
    pub fn set_service_creation_class_name(&mut self, value: String) {
        self.service_creation_class_name = Some(value);
    }

    /// Gets the value of ServiceCreationClassName
    pub fn get_service_creation_class_name(&self) -> Option<&String> {
        self.service_creation_class_name.as_ref()
    }

    /// Sets the value of ServiceName
    pub fn set_service_name(&mut self, value: String) {
        self.service_name = Some(value);
    }

    /// Gets the value of ServiceName
    pub fn get_service_name(&self) -> Option<&String> {
        self.service_name.as_ref()
    }

    /// Sets the value of SystemCreationClassName
    pub fn set_system_creation_class_name(&mut self, value: String) {
        self.system_creation_class_name = Some(value);
    }

    /// Gets the value of SystemCreationClassName
    pub fn get_system_creation_class_name(&self) -> Option<&String> {
        self.system_creation_class_name.as_ref()
    }

    /// Sets the value of SystemName
    pub fn set_system_name(&mut self, value: String) {
        self.system_name = Some(value);
    }

    /// Gets the value of SystemName
    pub fn get_system_name(&self) -> Option<&String> {
        self.system_name.as_ref()
    }
}

