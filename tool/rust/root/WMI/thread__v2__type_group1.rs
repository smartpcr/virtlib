// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Thread_V2_TypeGroup1 struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Thread_V2_TypeGroup1 {
    #[serde(flatten)]
    pub base: Thread_V2,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "StackBase")]
    pub stack_base: Option<u32>,

/// 
    #[serde(rename = "StackLimit")]
    pub stack_limit: Option<u32>,

/// 
    #[serde(rename = "StartAddr")]
    pub start_addr: Option<u32>,

/// 
    #[serde(rename = "SubProcessTag")]
    pub sub_process_tag: Option<u32>,

/// 
    #[serde(rename = "TebBase")]
    pub teb_base: Option<u32>,

/// 
    #[serde(rename = "TThreadId")]
    pub tthread_id: Option<u32>,

/// 
    #[serde(rename = "UserStackBase")]
    pub user_stack_base: Option<u32>,

/// 
    #[serde(rename = "UserStackLimit")]
    pub user_stack_limit: Option<u32>,

/// 
    #[serde(rename = "Win32StartAddr")]
    pub win32_start_addr: Option<u32>,
}

impl Thread_V2_TypeGroup1 {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Thread_V2::new(),
            process_id: None,
            stack_base: None,
            stack_limit: None,
            start_addr: None,
            sub_process_tag: None,
            teb_base: None,
            tthread_id: None,
            user_stack_base: None,
            user_stack_limit: None,
            win32_start_addr: None,
        }
    }


    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of StackBase
    pub fn set_stack_base(&mut self, value: u32) {
        self.stack_base = Some(value);
    }

    /// Gets the value of StackBase
    pub fn get_stack_base(&self) -> Option<&u32> {
        self.stack_base.as_ref()
    }

    /// Sets the value of StackLimit
    pub fn set_stack_limit(&mut self, value: u32) {
        self.stack_limit = Some(value);
    }

    /// Gets the value of StackLimit
    pub fn get_stack_limit(&self) -> Option<&u32> {
        self.stack_limit.as_ref()
    }

    /// Sets the value of StartAddr
    pub fn set_start_addr(&mut self, value: u32) {
        self.start_addr = Some(value);
    }

    /// Gets the value of StartAddr
    pub fn get_start_addr(&self) -> Option<&u32> {
        self.start_addr.as_ref()
    }

    /// Sets the value of SubProcessTag
    pub fn set_sub_process_tag(&mut self, value: u32) {
        self.sub_process_tag = Some(value);
    }

    /// Gets the value of SubProcessTag
    pub fn get_sub_process_tag(&self) -> Option<&u32> {
        self.sub_process_tag.as_ref()
    }

    /// Sets the value of TebBase
    pub fn set_teb_base(&mut self, value: u32) {
        self.teb_base = Some(value);
    }

    /// Gets the value of TebBase
    pub fn get_teb_base(&self) -> Option<&u32> {
        self.teb_base.as_ref()
    }

    /// Sets the value of TThreadId
    pub fn set_tthread_id(&mut self, value: u32) {
        self.tthread_id = Some(value);
    }

    /// Gets the value of TThreadId
    pub fn get_tthread_id(&self) -> Option<&u32> {
        self.tthread_id.as_ref()
    }

    /// Sets the value of UserStackBase
    pub fn set_user_stack_base(&mut self, value: u32) {
        self.user_stack_base = Some(value);
    }

    /// Gets the value of UserStackBase
    pub fn get_user_stack_base(&self) -> Option<&u32> {
        self.user_stack_base.as_ref()
    }

    /// Sets the value of UserStackLimit
    pub fn set_user_stack_limit(&mut self, value: u32) {
        self.user_stack_limit = Some(value);
    }

    /// Gets the value of UserStackLimit
    pub fn get_user_stack_limit(&self) -> Option<&u32> {
        self.user_stack_limit.as_ref()
    }

    /// Sets the value of Win32StartAddr
    pub fn set_win32_start_addr(&mut self, value: u32) {
        self.win32_start_addr = Some(value);
    }

    /// Gets the value of Win32StartAddr
    pub fn get_win32_start_addr(&self) -> Option<&u32> {
        self.win32_start_addr.as_ref()
    }
}

