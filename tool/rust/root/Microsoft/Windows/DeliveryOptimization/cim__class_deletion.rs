// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_ClassDeletion struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_ClassDeletion {
    #[serde(flatten)]
    pub base: CIM_ClassIndication,
}

impl CIM_ClassDeletion {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ClassIndication::new(),
        }
    }

}

