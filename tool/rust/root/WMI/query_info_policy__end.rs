// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// QueryInfoPolicy_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryInfoPolicy_End {
    #[serde(flatten)]
    pub base: QueryInfoPolicy,
}

impl QueryInfoPolicy_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: QueryInfoPolicy::new(),
        }
    }

}

