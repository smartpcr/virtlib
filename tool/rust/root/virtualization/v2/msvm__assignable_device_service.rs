// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_AssignableDeviceService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_AssignableDeviceService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_AssignableDeviceService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `dismount_setting_data` -  (String)

    /// * `dismounted_device_instance_path` -  (String)
    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn dismount_assignable_device(&self, dismount_setting_data: &String, dismounted_device_instance_path: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DismountSettingData".to_string(), value: dismount_setting_data.into() });

        let result = self.invoke_method_with_job("DismountAssignableDevice", &args)?;
        let dismounted_device_instance_path = result.get_value("DismountedDeviceInstancePath")?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `device_instance_path` -  (String)
    /// * `device_location_path` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `mounted_device_instance_path` -  (String)
    /// * `return_value` -  (u32)
    pub fn mount_assignable_device(&self, device_instance_path: &String, device_location_path: &String, mounted_device_instance_path: &mut String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DeviceInstancePath".to_string(), value: device_instance_path.into() });
        args.push(MethodParameter { name: "DeviceLocationPath".to_string(), value: device_location_path.into() });

        let result = self.invoke_method_with_job("MountAssignableDevice", &args)?;
        let job = result.get_value("Job")?;
        let mounted_device_instance_path = result.get_value("MountedDeviceInstancePath")?;
        Ok(result.return_value)

    }

}

impl Msvm_AssignableDeviceService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

}

