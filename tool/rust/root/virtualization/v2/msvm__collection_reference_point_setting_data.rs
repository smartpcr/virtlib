// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_CollectionReferencePointSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_CollectionReferencePointSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// 
    #[serde(rename = "ConsistencyLevel")]
    pub consistency_level: Option<u8>,
}

impl Msvm_CollectionReferencePointSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            consistency_level: None,
        }
    }


    /// Sets the value of ConsistencyLevel
    pub fn set_consistency_level(&mut self, value: u8) {
        self.consistency_level = Some(value);
    }

    /// Gets the value of ConsistencyLevel
    pub fn get_consistency_level(&self) -> Option<&u8> {
        self.consistency_level.as_ref()
    }
}

