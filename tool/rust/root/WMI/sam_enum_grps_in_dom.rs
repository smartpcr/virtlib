// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SamEnumGrpsInDom struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SamEnumGrpsInDom {
    #[serde(flatten)]
    pub base: MSSAMTrace,
}

impl SamEnumGrpsInDom {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSSAMTrace::new(),
        }
    }

}

