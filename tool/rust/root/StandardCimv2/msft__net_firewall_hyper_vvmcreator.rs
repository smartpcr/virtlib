// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVVMCreator struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVVMCreator {
    #[serde(flatten)]
    pub base: CIM_ManagedElement,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "VMCreatorId")]
    pub vmcreator_id: Option<String>,
}

impl MSFT_NetFirewallHyperVVMCreator {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_ManagedElement::new(),
            friendly_name: None,
            vmcreator_id: None,
        }
    }


    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of VMCreatorId
    pub fn set_vmcreator_id(&mut self, value: String) {
        self.vmcreator_id = Some(value);
    }

    /// Gets the value of VMCreatorId
    pub fn get_vmcreator_id(&self) -> Option<&String> {
        self.vmcreator_id.as_ref()
    }

/// 

    /// * `friendly_name` -  (String)
    /// * `vmcreator_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn register_hyper_vvmcreator(&self, vmcreator_id: &String, friendly_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VMCreatorId".to_string(), value: vmcreator_id.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        self.invoke_method("RegisterHyperVVMCreator", &args)

    }


/// 

    /// * `vmcreator_id` -  (String)

    /// * `return_value` -  (u32)
    pub fn unregister_hyper_vvmcreator(&self, vmcreator_id: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VMCreatorId".to_string(), value: vmcreator_id.into() });
        self.invoke_method("UnregisterHyperVVMCreator", &args)

    }

}

