// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// FinalizeKTimer2 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FinalizeKTimer2 {
    #[serde(flatten)]
    pub base: PerfInfo_V2,

/// 
    #[serde(rename = "DisableCallback")]
    pub disable_callback: Option<u32>,

/// 
    #[serde(rename = "DisableContext")]
    pub disable_context: Option<u32>,

/// 
    #[serde(rename = "Timer")]
    pub timer: Option<u32>,
}

impl FinalizeKTimer2 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: PerfInfo_V2::new(),
            disable_callback: None,
            disable_context: None,
            timer: None,
        }
    }


    /// Sets the value of DisableCallback
    pub fn set_disable_callback(&mut self, value: u32) {
        self.disable_callback = Some(value);
    }

    /// Gets the value of DisableCallback
    pub fn get_disable_callback(&self) -> Option<&u32> {
        self.disable_callback.as_ref()
    }

    /// Sets the value of DisableContext
    pub fn set_disable_context(&mut self, value: u32) {
        self.disable_context = Some(value);
    }

    /// Gets the value of DisableContext
    pub fn get_disable_context(&self) -> Option<&u32> {
        self.disable_context.as_ref()
    }

    /// Sets the value of Timer
    pub fn set_timer(&mut self, value: u32) {
        self.timer = Some(value);
    }

    /// Gets the value of Timer
    pub fn get_timer(&self) -> Option<&u32> {
        self.timer.as_ref()
    }
}

