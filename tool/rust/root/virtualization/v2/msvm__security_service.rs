// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_SecurityService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_SecurityService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_SecurityService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `security_setting_data` - An embedded instance of class Msvm_SecuritySettingData that describes modifications to the current security settings of an existing virtual machine. (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_security_settings(&self, security_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecuritySettingData".to_string(), value: security_setting_data.into() });

        let result = self.invoke_method_with_job("ModifySecuritySettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `security_policy` -  (u8[])
    /// * `security_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_security_policy(&self, security_setting_data: &String, security_policy: &Vec<u8>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecuritySettingData".to_string(), value: security_setting_data.into() });
        args.push(MethodParameter { name: "SecurityPolicy".to_string(), value: security_policy.into() });

        let result = self.invoke_method_with_job("SetSecurityPolicy", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `key_protector` -  (u8[])
    /// * `security_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_key_protector(&self, security_setting_data: &String, key_protector: &Vec<u8>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecuritySettingData".to_string(), value: security_setting_data.into() });
        args.push(MethodParameter { name: "KeyProtector".to_string(), value: key_protector.into() });

        let result = self.invoke_method_with_job("SetKeyProtector", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `security_setting_data` -  (String)

    /// * `key_protector` -  (u8[])
    /// * `return_value` -  (u32)
    pub fn get_key_protector(&self, security_setting_data: &String, key_protector: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecuritySettingData".to_string(), value: security_setting_data.into() });

        let result = self.invoke_method("GetKeyProtector", &args)?;
        let key_protector = result.get_value("KeyProtector")?;
        Ok(result.return_value)

    }


/// 

    /// * `security_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn restore_last_known_good_key_protector(&self, security_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SecuritySettingData".to_string(), value: security_setting_data.into() });

        let result = self.invoke_method_with_job("RestoreLastKnownGoodKeyProtector", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

impl Msvm_SecurityService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Vec<Msvm_ComputerSystem>, WmiError> {
        self.get_all_related("Msvm_ComputerSystem")
    }

}

