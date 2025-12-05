// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// QueryInfoPolicy2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryInfoPolicy2 {
    #[serde(flatten)]
    pub base: MSLSATrace,
}

impl QueryInfoPolicy2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSLSATrace::new(),
        }
    }

}

