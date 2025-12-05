// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CollectedMSEs struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CollectedMSEs {

/// 
    #[serde(rename = "Collection")]
    pub collection: Option<CIM_CollectionOfMSEs>,

/// 
    #[serde(rename = "Member")]
    pub member: Option<CIM_ManagedSystemElement>,
}

impl CIM_CollectedMSEs {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collection: None,
            member: None,
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

    /// Sets the value of Member
    pub fn set_member(&mut self, value: CIM_ManagedSystemElement) {
        self.member = Some(value);
    }

    /// Gets the value of Member
    pub fn get_member(&self) -> Option<&CIM_ManagedSystemElement> {
        self.member.as_ref()
    }
}

