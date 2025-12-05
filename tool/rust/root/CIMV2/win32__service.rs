// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_Service struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_Service {
    #[serde(flatten)]
    pub base: Win32_BaseService,

/// 
    #[serde(rename = "CheckPoint")]
    pub check_point: Option<u32>,

/// 
    #[serde(rename = "DelayedAutoStart")]
    pub delayed_auto_start: Option<bool>,

/// 
    #[serde(rename = "ProcessId")]
    pub process_id: Option<u32>,

/// 
    #[serde(rename = "WaitHint")]
    pub wait_hint: Option<u32>,
}

impl Win32_Service {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_BaseService::new(),
            check_point: None,
            delayed_auto_start: None,
            process_id: None,
            wait_hint: None,
        }
    }


    /// Sets the value of CheckPoint
    pub fn set_check_point(&mut self, value: u32) {
        self.check_point = Some(value);
    }

    /// Gets the value of CheckPoint
    pub fn get_check_point(&self) -> Option<&u32> {
        self.check_point.as_ref()
    }

    /// Sets the value of DelayedAutoStart
    pub fn set_delayed_auto_start(&mut self, value: bool) {
        self.delayed_auto_start = Some(value);
    }

    /// Gets the value of DelayedAutoStart
    pub fn get_delayed_auto_start(&self) -> Option<&bool> {
        self.delayed_auto_start.as_ref()
    }

    /// Sets the value of ProcessId
    pub fn set_process_id(&mut self, value: u32) {
        self.process_id = Some(value);
    }

    /// Gets the value of ProcessId
    pub fn get_process_id(&self) -> Option<&u32> {
        self.process_id.as_ref()
    }

    /// Sets the value of WaitHint
    pub fn set_wait_hint(&mut self, value: u32) {
        self.wait_hint = Some(value);
    }

    /// Gets the value of WaitHint
    pub fn get_wait_hint(&self) -> Option<&u32> {
        self.wait_hint.as_ref()
    }

/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)
    /// * `return_value` -  (u32)
    pub fn get_security_descriptor(&self, descriptor: &mut Win32_SecurityDescriptor) -> Result<(), WmiError> {

        let result = self.invoke_method("GetSecurityDescriptor", &[])?;
        let descriptor = result.get_value("Descriptor")?;
        Ok(result.return_value)

    }


/// 

    /// * `descriptor` -  (Win32_SecurityDescriptor)

    /// * `return_value` -  (u32)
    pub fn set_security_descriptor(&self, descriptor: Win32_SecurityDescriptor) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Descriptor".to_string(), value: descriptor.into() });
        self.invoke_method("SetSecurityDescriptor", &args)

    }

}

