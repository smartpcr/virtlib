// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ReferencePointCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ReferencePointCollection {
    #[serde(flatten)]
    pub base: CIM_Collection,

/// 
    #[serde(rename = "CollectionID")]
    pub collection_id: Option<String>,

/// 
    #[serde(rename = "ConsistencyLevel")]
    pub consistency_level: Option<u16>,

/// 
    #[serde(rename = "HasAssociatedLog")]
    pub has_associated_log: Option<bool>,

/// 
    #[serde(rename = "ReferencePointType")]
    pub reference_point_type: Option<ReferencePointCollection_ReferencePointType>,

/// 
    #[serde(rename = "VirtualSystemCollectionId")]
    pub virtual_system_collection_id: Option<String>,
}

impl Msvm_ReferencePointCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Collection::new(),
            collection_id: None,
            consistency_level: None,
            has_associated_log: None,
            reference_point_type: None,
            virtual_system_collection_id: None,
        }
    }


    /// Sets the value of CollectionID
    pub fn set_collection_id(&mut self, value: String) {
        self.collection_id = Some(value);
    }

    /// Gets the value of CollectionID
    pub fn get_collection_id(&self) -> Option<&String> {
        self.collection_id.as_ref()
    }

    /// Sets the value of ConsistencyLevel
    pub fn set_consistency_level(&mut self, value: u16) {
        self.consistency_level = Some(value);
    }

    /// Gets the value of ConsistencyLevel
    pub fn get_consistency_level(&self) -> Option<&u16> {
        self.consistency_level.as_ref()
    }

    /// Sets the value of HasAssociatedLog
    pub fn set_has_associated_log(&mut self, value: bool) {
        self.has_associated_log = Some(value);
    }

    /// Gets the value of HasAssociatedLog
    pub fn get_has_associated_log(&self) -> Option<&bool> {
        self.has_associated_log.as_ref()
    }

    /// Sets the value of ReferencePointType
    pub fn set_reference_point_type(&mut self, value: ReferencePointCollection_ReferencePointType) {
        self.reference_point_type = Some(value);
    }

    /// Gets the value of ReferencePointType
    pub fn get_reference_point_type(&self) -> Option<&ReferencePointCollection_ReferencePointType> {
        self.reference_point_type.as_ref()
    }

    /// Sets the value of VirtualSystemCollectionId
    pub fn set_virtual_system_collection_id(&mut self, value: String) {
        self.virtual_system_collection_id = Some(value);
    }

    /// Gets the value of VirtualSystemCollectionId
    pub fn get_virtual_system_collection_id(&self) -> Option<&String> {
        self.virtual_system_collection_id.as_ref()
    }
}

