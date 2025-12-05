// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// SubProcessTagChanged struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SubProcessTagChanged {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "NewTag")]
    pub new_tag: Option<u32>,

/// 
    #[serde(rename = "OldTag")]
    pub old_tag: Option<u32>,
}

impl SubProcessTagChanged {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            new_tag: None,
            old_tag: None,
        }
    }


    /// Sets the value of NewTag
    pub fn set_new_tag(&mut self, value: u32) {
        self.new_tag = Some(value);
    }

    /// Gets the value of NewTag
    pub fn get_new_tag(&self) -> Option<&u32> {
        self.new_tag.as_ref()
    }

    /// Sets the value of OldTag
    pub fn set_old_tag(&mut self, value: u32) {
        self.old_tag = Some(value);
    }

    /// Gets the value of OldTag
    pub fn get_old_tag(&self) -> Option<&u32> {
        self.old_tag.as_ref()
    }
}

