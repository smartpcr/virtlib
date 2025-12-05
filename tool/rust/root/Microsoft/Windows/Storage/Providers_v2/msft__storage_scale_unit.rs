// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage.Providers_v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageScaleUnit struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageScaleUnit {
    #[serde(flatten)]
    pub base: MSFT_StorageFaultDomain,
}

impl MSFT_StorageScaleUnit {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_StorageFaultDomain::new(),
        }
    }


/// 

    /// * `enable_maintenance_mode` -  (bool)
    /// * `ignore_detached_virtual_disks` -  (bool)
    /// * `manufacturer` -  (String)
    /// * `model` -  (String)
    /// * `timeout` -  (u32)
    /// * `validate_maintenance_mode` -  (bool)

    /// * `extended_status` -  (MSFT_StorageExtendedStatus)
    /// * `return_value` -  (u32)
    pub fn maintenance(&self, validate_maintenance_mode: bool, enable_maintenance_mode: bool, timeout: u32, model: &String, manufacturer: &String, ignore_detached_virtual_disks: bool, extended_status: &mut MSFT_StorageExtendedStatus) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ValidateMaintenanceMode".to_string(), value: validate_maintenance_mode.into() });
        args.push(MethodParameter { name: "EnableMaintenanceMode".to_string(), value: enable_maintenance_mode.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Model".to_string(), value: model.into() });
        args.push(MethodParameter { name: "Manufacturer".to_string(), value: manufacturer.into() });
        args.push(MethodParameter { name: "IgnoreDetachedVirtualDisks".to_string(), value: ignore_detached_virtual_disks.into() });

        let result = self.invoke_method("Maintenance", &args)?;
        let extended_status = result.get_value("ExtendedStatus")?;
        Ok(result.return_value)

    }

}

