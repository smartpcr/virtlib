// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// QueryInfoPolicy2_End struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryInfoPolicy2_End {
    #[serde(flatten)]
    pub base: QueryInfoPolicy2,
}

impl QueryInfoPolicy2_End {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: QueryInfoPolicy2::new(),
        }
    }

}

