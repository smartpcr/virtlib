// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// AuthorizedPrivilege struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthorizedPrivilege {
    #[serde(flatten)]
    pub base: CIM_AuthorizedPrivilege,
}

impl AuthorizedPrivilege {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_AuthorizedPrivilege::new(),
        }
    }

}

