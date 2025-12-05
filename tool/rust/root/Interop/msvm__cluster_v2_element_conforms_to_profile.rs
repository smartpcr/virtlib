// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Interop
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ClusterV2ElementConformsToProfile struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ClusterV2ElementConformsToProfile {
    #[serde(flatten)]
    pub base: Msvm_ElementConformsToProfile,
}

impl Msvm_ClusterV2ElementConformsToProfile {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Msvm_ElementConformsToProfile::new(),
        }
    }

}

