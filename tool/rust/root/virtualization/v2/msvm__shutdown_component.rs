// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_ShutdownComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_ShutdownComponent {
    #[serde(flatten)]
    pub base: CIM_LogicalDevice,
}

impl Msvm_ShutdownComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_LogicalDevice::new(),
        }
    }


/// 

    /// * `force` -  (bool)
    /// * `reason` -  (String)

    /// * `return_value` -  (u32)
    pub fn initiate_shutdown(&self, force: bool, reason: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });
        self.invoke_method("InitiateShutdown", &args)

    }


/// 

    /// * `force` -  (bool)
    /// * `reason` -  (String)

    /// * `return_value` -  (u32)
    pub fn initiate_reboot(&self, force: bool, reason: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "Reason".to_string(), value: reason.into() });
        self.invoke_method("InitiateReboot", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn initiate_hibernate(&self) -> Result<(), WmiError> {
        self.invoke_method("InitiateHibernate", &[])

    }

}

