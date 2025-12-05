// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.HyperVCluster.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CollectedMSEs struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CollectedMSEs {
    #[serde(flatten)]
    pub base: CIM_MemberOfCollection,
}

impl CIM_CollectedMSEs {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_MemberOfCollection::new(),
        }
    }

}

