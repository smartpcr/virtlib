// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbComponent struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbComponent {
}

impl MSFT_SmbComponent {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `name` -  (String[])

    /// * `return_value` -  (u32)
    pub fn remove_smb_component(&self, name: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        self.invoke_method("RemoveSmbComponent", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn client_unload_smb_direct(&self) -> Result<(), WmiError> {
        self.invoke_method("ClientUnloadSmbDirect", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn client_reload_smb_direct(&self) -> Result<(), WmiError> {
        self.invoke_method("ClientReloadSmbDirect", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn server_unload_smb_direct(&self) -> Result<(), WmiError> {
        self.invoke_method("ServerUnloadSmbDirect", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn server_reload_smb_direct(&self) -> Result<(), WmiError> {
        self.invoke_method("ServerReloadSmbDirect", &[])

    }

}

