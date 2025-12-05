// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_BIOSFeatureBIOSElements struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_BIOSFeatureBIOSElements {
    #[serde(flatten)]
    pub base: CIM_SoftwareFeatureSoftwareElements,
}

impl CIM_BIOSFeatureBIOSElements {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SoftwareFeatureSoftwareElements::new(),
        }
    }

}

