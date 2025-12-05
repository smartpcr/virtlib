// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_CollectionSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_CollectionSetting {

/// 
    #[serde(rename = "Collection")]
    pub collection: Option<CIM_CollectionOfMSEs>,

/// 
    #[serde(rename = "Setting")]
    pub setting: Option<CIM_Setting>,
}

impl CIM_CollectionSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            collection: None,
            setting: None,
        }
    }


    /// Sets the value of Collection
    pub fn set_collection(&mut self, value: CIM_CollectionOfMSEs) {
        self.collection = Some(value);
    }

    /// Gets the value of Collection
    pub fn get_collection(&self) -> Option<&CIM_CollectionOfMSEs> {
        self.collection.as_ref()
    }

    /// Sets the value of Setting
    pub fn set_setting(&mut self, value: CIM_Setting) {
        self.setting = Some(value);
    }

    /// Gets the value of Setting
    pub fn get_setting(&self) -> Option<&CIM_Setting> {
        self.setting.as_ref()
    }
}

