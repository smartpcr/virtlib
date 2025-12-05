// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_ThreadStartTrace struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_ThreadStartTrace {
    #[serde(flatten)]
    pub base: Win32_ThreadTrace,

/// 
    #[serde(rename = "StackBase")]
    pub stack_base: Option<u64>,

/// 
    #[serde(rename = "StackLimit")]
    pub stack_limit: Option<u64>,

/// 
    #[serde(rename = "StartAddr")]
    pub start_addr: Option<u64>,

/// 
    #[serde(rename = "UserStackBase")]
    pub user_stack_base: Option<u64>,

/// 
    #[serde(rename = "UserStackLimit")]
    pub user_stack_limit: Option<u64>,

/// 
    #[serde(rename = "WaitMode")]
    pub wait_mode: Option<u32>,

/// 
    #[serde(rename = "Win32StartAddr")]
    pub win32_start_addr: Option<u64>,
}

impl Win32_ThreadStartTrace {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_ThreadTrace::new(),
            stack_base: None,
            stack_limit: None,
            start_addr: None,
            user_stack_base: None,
            user_stack_limit: None,
            wait_mode: None,
            win32_start_addr: None,
        }
    }


    /// Sets the value of StackBase
    pub fn set_stack_base(&mut self, value: u64) {
        self.stack_base = Some(value);
    }

    /// Gets the value of StackBase
    pub fn get_stack_base(&self) -> Option<&u64> {
        self.stack_base.as_ref()
    }

    /// Sets the value of StackLimit
    pub fn set_stack_limit(&mut self, value: u64) {
        self.stack_limit = Some(value);
    }

    /// Gets the value of StackLimit
    pub fn get_stack_limit(&self) -> Option<&u64> {
        self.stack_limit.as_ref()
    }

    /// Sets the value of StartAddr
    pub fn set_start_addr(&mut self, value: u64) {
        self.start_addr = Some(value);
    }

    /// Gets the value of StartAddr
    pub fn get_start_addr(&self) -> Option<&u64> {
        self.start_addr.as_ref()
    }

    /// Sets the value of UserStackBase
    pub fn set_user_stack_base(&mut self, value: u64) {
        self.user_stack_base = Some(value);
    }

    /// Gets the value of UserStackBase
    pub fn get_user_stack_base(&self) -> Option<&u64> {
        self.user_stack_base.as_ref()
    }

    /// Sets the value of UserStackLimit
    pub fn set_user_stack_limit(&mut self, value: u64) {
        self.user_stack_limit = Some(value);
    }

    /// Gets the value of UserStackLimit
    pub fn get_user_stack_limit(&self) -> Option<&u64> {
        self.user_stack_limit.as_ref()
    }

    /// Sets the value of WaitMode
    pub fn set_wait_mode(&mut self, value: u32) {
        self.wait_mode = Some(value);
    }

    /// Gets the value of WaitMode
    pub fn get_wait_mode(&self) -> Option<&u32> {
        self.wait_mode.as_ref()
    }

    /// Sets the value of Win32StartAddr
    pub fn set_win32_start_addr(&mut self, value: u64) {
        self.win32_start_addr = Some(value);
    }

    /// Gets the value of Win32StartAddr
    pub fn get_win32_start_addr(&self) -> Option<&u64> {
        self.win32_start_addr.as_ref()
    }
}

