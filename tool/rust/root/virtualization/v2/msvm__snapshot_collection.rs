// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SnapshotCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SnapshotCollection {
    #[serde(flatten)]
    pub base: CIM_Collection,

/// The unique identification of the Collection object.
    #[serde(rename = "CollectionID")]
    pub collection_id: Option<String>,
}

impl Msvm_SnapshotCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Collection::new(),
            collection_id: None,
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
}

