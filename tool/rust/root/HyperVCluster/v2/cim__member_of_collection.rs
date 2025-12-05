// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_MemberOfCollection struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_MemberOfCollection {

/// The Collection that aggregates members.
    #[serde(rename = "Collection")]
    pub collection: Option<CIM_Collection>,

/// The aggregated member of the Collection.
    #[serde(rename = "Member")]
    pub member: Option<CIM_ManagedElement>,
}

impl CIM_MemberOfCollection {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collection: None,
            member: None,
        }
    }


    /// Sets the value of Collection
    pub fn set_collection(&mut self, value: CIM_Collection) {
        self.collection = Some(value);
    }

    /// Gets the value of Collection
    pub fn get_collection(&self) -> Option<&CIM_Collection> {
        self.collection.as_ref()
    }

    /// Sets the value of Member
    pub fn set_member(&mut self, value: CIM_ManagedElement) {
        self.member = Some(value);
    }

    /// Gets the value of Member
    pub fn get_member(&self) -> Option<&CIM_ManagedElement> {
        self.member.as_ref()
    }
}

