// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PrivilegeGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrivilegeGroup {
    #[serde(flatten)]
    pub base: CIM_Group,
}

impl PrivilegeGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Group::new(),
        }
    }

}

