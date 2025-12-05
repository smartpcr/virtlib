// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// EnumTrstedDomEx_Start struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnumTrstedDomEx_Start {
    #[serde(flatten)]
    pub base: EnumTrstedDomEx,
}

impl EnumTrstedDomEx_Start {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: EnumTrstedDomEx::new(),
        }
    }

}

