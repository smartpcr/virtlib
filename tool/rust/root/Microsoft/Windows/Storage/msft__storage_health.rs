// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageHealth struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageHealth {
    #[serde(flatten)]
    pub base: MSFT_StorageObject,
}

impl MSFT_StorageHealth {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageObject::new(),
        }
    }


/// 

    /// * `name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    /// * `storage_health_setting` -  (MSFT_StorageHealthSetting[])
    pub fn get_setting(&self, name: &String, storage_health_setting: &mut Vec<MSFT_StorageHealthSetting>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("GetSetting", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let storage_health_setting = result.get_value("StorageHealthSetting")?;
        Ok(result.return_value)

    }


/// 

    /// * `name` -  (String)
    /// * `value` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn set_setting(&self, name: &String, value: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });

        let result = self.invoke_method("SetSetting", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `name` -  (String)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_setting(&self, name: &String, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });

        let result = self.invoke_method("RemoveSetting", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `count` -  (u32)
    /// * `target_object` -  (MSFT_StorageObject)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `reports` -  (MSFT_StorageHealthReport[])
    /// * `return_value` -  (u32)
    pub fn get_report(&self, target_object: MSFT_StorageObject, count: u32, reports: &mut Vec<MSFT_StorageHealthReport>, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetObject".to_string(), value: target_object.into() });
        args.push(MethodParameter { name: "Count".to_string(), value: count.into() });

        let result = self.invoke_method("GetReport", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        let reports = result.get_value("Reports")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_maintenance_mode` -  (bool)
    /// * `ignore_detached_virtual_disks` -  (bool)
    /// * `manufacturer` -  (String)
    /// * `model` -  (String)
    /// * `target_object` -  (MSFT_StorageFaultDomain)
    /// * `timeout` -  (u32)
    /// * `validate_maintenance_mode` -  (bool)
    /// * `validation_flags` -  (u16)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn maintenance(&self, target_object: MSFT_StorageFaultDomain, validate_maintenance_mode: bool, enable_maintenance_mode: bool, ignore_detached_virtual_disks: bool, timeout: u32, model: &String, manufacturer: &String, validation_flags: u16, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetObject".to_string(), value: target_object.into() });
        args.push(MethodParameter { name: "ValidateMaintenanceMode".to_string(), value: validate_maintenance_mode.into() });
        args.push(MethodParameter { name: "EnableMaintenanceMode".to_string(), value: enable_maintenance_mode.into() });
        args.push(MethodParameter { name: "IgnoreDetachedVirtualDisks".to_string(), value: ignore_detached_virtual_disks.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Model".to_string(), value: model.into() });
        args.push(MethodParameter { name: "Manufacturer".to_string(), value: manufacturer.into() });
        args.push(MethodParameter { name: "ValidationFlags".to_string(), value: validation_flags.into() });

        let result = self.invoke_method("Maintenance", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }


/// 

    /// * `target_object` -  (MSFT_StorageObject)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn remove_intent(&self, target_object: MSFT_StorageObject, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TargetObject".to_string(), value: target_object.into() });

        let result = self.invoke_method("RemoveIntent", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

