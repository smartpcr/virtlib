// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Hardware
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Account struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    #[serde(flatten)]
    pub base: CIM_Account,
}

impl Account {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Account::new(),
        }
    }

}

