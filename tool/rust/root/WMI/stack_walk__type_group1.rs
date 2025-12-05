// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// StackWalk_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StackWalk_TypeGroup1 {
    #[serde(flatten)]
    pub base: StackWalk,

/// 
    #[serde(rename = "key")]
    pub key: Option<u32>,

/// 
    #[serde(rename = "StackFrame")]
    pub stack_frame: Vec<u32>,
}

impl StackWalk_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: StackWalk::new(),
            key: None,
            stack_frame: Vec::new(),
        }
    }


    /// Sets the value of key
    pub fn set_key(&mut self, value: u32) {
        self.key = Some(value);
    }

    /// Gets the value of key
    pub fn get_key(&self) -> Option<&u32> {
        self.key.as_ref()
    }

    /// Sets the value of StackFrame
    pub fn set_stack_frame(&mut self, value: Vec<u32>) {
        self.stack_frame = value;
    }

    /// Gets the value of StackFrame
    pub fn get_stack_frame(&self) -> &Vec<u32> {
        &self.stack_frame
    }
}

