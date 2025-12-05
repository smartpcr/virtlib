// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CollectedCollections struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CollectedCollections {

/// 
    #[serde(rename = "Collection")]
    pub collection: Option<CIM_CollectionOfMSEs>,

/// 
    #[serde(rename = "CollectionInCollection")]
    pub collection_in_collection: Option<CIM_CollectionOfMSEs>,
}

impl CIM_CollectedCollections {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collection: None,
            collection_in_collection: None,
        }
    }


    /// Sets the value of Collection
    pub fn set_collection(&mut self, value: CIM_CollectionOfMSEs) {
        self.collection = Some(value);
    }

    /// Gets the value of Collection
    pub fn get_collection(&self) -> Option<&CIM_CollectionOfMSEs> {
        self.collection.as_ref()
    }

    /// Sets the value of CollectionInCollection
    pub fn set_collection_in_collection(&mut self, value: CIM_CollectionOfMSEs) {
        self.collection_in_collection = Some(value);
    }

    /// Gets the value of CollectionInCollection
    pub fn get_collection_in_collection(&self) -> Option<&CIM_CollectionOfMSEs> {
        self.collection_in_collection.as_ref()
    }
}

