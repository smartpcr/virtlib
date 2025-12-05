// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_SecuritySetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_SecuritySetting {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "ControlFlags")]
    pub control_flags: Option<u32>,
}

impl Win32_SecuritySetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            control_flags: None,
        }
    }


    /// Sets the value of ControlFlags
    pub fn set_control_flags(&mut self, value: u32) {
        self.control_flags = Some(value);
    }

    /// Gets the value of ControlFlags
    pub fn get_control_flags(&self) -> Option<&u32> {
        self.control_flags.as_ref()
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

