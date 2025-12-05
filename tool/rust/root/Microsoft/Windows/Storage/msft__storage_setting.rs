// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageSetting struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageSetting {

/// 
    #[serde(rename = "NewDiskPolicy")]
    pub new_disk_policy: Option<u16>,

/// 
    #[serde(rename = "ScrubPolicy")]
    pub scrub_policy: Option<u32>,
}

impl MSFT_StorageSetting {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            new_disk_policy: None,
            scrub_policy: None,
        }
    }


    /// Sets the value of NewDiskPolicy
    pub fn set_new_disk_policy(&mut self, value: u16) {
        self.new_disk_policy = Some(value);
    }

    /// Gets the value of NewDiskPolicy
    pub fn get_new_disk_policy(&self) -> Option<&u16> {
        self.new_disk_policy.as_ref()
    }

    /// Sets the value of ScrubPolicy
    pub fn set_scrub_policy(&mut self, value: u32) {
        self.scrub_policy = Some(value);
    }

    /// Gets the value of ScrubPolicy
    pub fn get_scrub_policy(&self) -> Option<&u32> {
        self.scrub_policy.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    /// * `storage_setting` -  (MSFT_StorageSetting)
    pub fn get(&self, storage_setting: &mut MSFT_StorageSetting) -> Result<(), WmiError> {

        let result = self.invoke_method("Get", &[])?;
        let storage_setting = result.get_value("StorageSetting")?;
        Ok(result.return_value)

    }


/// 

    /// * `new_disk_policy` -  (u16)
    /// * `scrub_policy` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set(&self, new_disk_policy: u16, scrub_policy: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewDiskPolicy".to_string(), value: new_disk_policy.into() });
        args.push(MethodParameter { name: "ScrubPolicy".to_string(), value: scrub_policy.into() });
        self.invoke_method("Set", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn update_host_storage_cache(&self) -> Result<(), WmiError> {
        self.invoke_method("UpdateHostStorageCache", &[])

    }

}

