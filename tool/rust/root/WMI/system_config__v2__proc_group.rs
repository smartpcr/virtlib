// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SystemConfig_V2_ProcGroup struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemConfig_V2_ProcGroup {
    #[serde(flatten)]
    pub base: SystemConfig_V2,

/// 
    #[serde(rename = "Affinity")]
    pub affinity: Vec<u32>,

/// 
    #[serde(rename = "GroupCount")]
    pub group_count: Option<u32>,
}

impl SystemConfig_V2_ProcGroup {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: SystemConfig_V2::new(),
            affinity: Vec::new(),
            group_count: None,
        }
    }


    /// Sets the value of Affinity
    pub fn set_affinity(&mut self, value: Vec<u32>) {
        self.affinity = value;
    }

    /// Gets the value of Affinity
    pub fn get_affinity(&self) -> &Vec<u32> {
        &self.affinity
    }

    /// Sets the value of GroupCount
    pub fn set_group_count(&mut self, value: u32) {
        self.group_count = Some(value);
    }

    /// Gets the value of GroupCount
    pub fn get_group_count(&self) -> Option<&u32> {
        self.group_count.as_ref()
    }
}

