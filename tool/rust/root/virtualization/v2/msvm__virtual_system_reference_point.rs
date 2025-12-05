// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemReferencePoint struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemReferencePoint {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "ConsistencyLevel")]
    pub consistency_level: Option<u16>,

/// 
    #[serde(rename = "HasAssociatedData")]
    pub has_associated_data: Option<bool>,

/// 
    #[serde(rename = "ReferencePointType")]
    pub reference_point_type: Option<u16>,

/// 
    #[serde(rename = "ResilientChangeTrackingIdentifiers")]
    pub resilient_change_tracking_identifiers: Vec<String>,

/// 
    #[serde(rename = "VirtualDiskIdentifiers")]
    pub virtual_disk_identifiers: Vec<String>,

/// 
    #[serde(rename = "VirtualSystemIdentifier")]
    pub virtual_system_identifier: Option<String>,
}

impl Msvm_VirtualSystemReferencePoint {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            consistency_level: None,
            has_associated_data: None,
            reference_point_type: None,
            resilient_change_tracking_identifiers: Vec::new(),
            virtual_disk_identifiers: Vec::new(),
            virtual_system_identifier: None,
        }
    }


    /// Sets the value of ConsistencyLevel
    pub fn set_consistency_level(&mut self, value: u16) {
        self.consistency_level = Some(value);
    }

    /// Gets the value of ConsistencyLevel
    pub fn get_consistency_level(&self) -> Option<&u16> {
        self.consistency_level.as_ref()
    }

    /// Sets the value of HasAssociatedData
    pub fn set_has_associated_data(&mut self, value: bool) {
        self.has_associated_data = Some(value);
    }

    /// Gets the value of HasAssociatedData
    pub fn get_has_associated_data(&self) -> Option<&bool> {
        self.has_associated_data.as_ref()
    }

    /// Sets the value of ReferencePointType
    pub fn set_reference_point_type(&mut self, value: u16) {
        self.reference_point_type = Some(value);
    }

    /// Gets the value of ReferencePointType
    pub fn get_reference_point_type(&self) -> Option<&u16> {
        self.reference_point_type.as_ref()
    }

    /// Sets the value of ResilientChangeTrackingIdentifiers
    pub fn set_resilient_change_tracking_identifiers(&mut self, value: Vec<String>) {
        self.resilient_change_tracking_identifiers = value;
    }

    /// Gets the value of ResilientChangeTrackingIdentifiers
    pub fn get_resilient_change_tracking_identifiers(&self) -> &Vec<String> {
        &self.resilient_change_tracking_identifiers
    }

    /// Sets the value of VirtualDiskIdentifiers
    pub fn set_virtual_disk_identifiers(&mut self, value: Vec<String>) {
        self.virtual_disk_identifiers = value;
    }

    /// Gets the value of VirtualDiskIdentifiers
    pub fn get_virtual_disk_identifiers(&self) -> &Vec<String> {
        &self.virtual_disk_identifiers
    }

    /// Sets the value of VirtualSystemIdentifier
    pub fn set_virtual_system_identifier(&mut self, value: String) {
        self.virtual_system_identifier = Some(value);
    }

    /// Gets the value of VirtualSystemIdentifier
    pub fn get_virtual_system_identifier(&self) -> Option<&String> {
        self.virtual_system_identifier.as_ref()
    }
}

