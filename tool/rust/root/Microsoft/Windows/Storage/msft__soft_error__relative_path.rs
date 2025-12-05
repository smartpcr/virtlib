// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SoftError_RelativePath struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SoftError_RelativePath {
    #[serde(flatten)]
    pub base: MSFT_SoftError,
}

impl MSFT_SoftError_RelativePath {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_SoftError::new(),
        }
    }

}

