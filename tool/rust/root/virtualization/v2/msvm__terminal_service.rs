// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_TerminalService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_TerminalService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_TerminalService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `service_setting_data` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_service_settings(&self, service_setting_data: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServiceSettingData".to_string(), value: service_setting_data.into() });

        let result = self.invoke_method_with_job("ModifyServiceSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `trustees` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn grant_interactive_session_access(&self, computer_system: CIM_ComputerSystem, trustees: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "Trustees".to_string(), value: trustees.into() });

        let result = self.invoke_method_with_job("GrantInteractiveSessionAccess", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)
    /// * `trustees` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn revoke_interactive_session_access(&self, computer_system: CIM_ComputerSystem, trustees: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });
        args.push(MethodParameter { name: "Trustees".to_string(), value: trustees.into() });

        let result = self.invoke_method_with_job("RevokeInteractiveSessionAccess", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `computer_system` -  (CIM_ComputerSystem)

    /// * `access_control_list` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_interactive_session_acl(&self, computer_system: CIM_ComputerSystem, access_control_list: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ComputerSystem".to_string(), value: computer_system.into() });

        let result = self.invoke_method("GetInteractiveSessionACL", &args)?;
        let access_control_list = result.get_value("AccessControlList")?;
        Ok(result.return_value)

    }

}

impl Msvm_TerminalService {
    /// Gets the related Msvm_ComputerSystem object(s)
    pub fn get_related__computer_system(&self) -> Result<Msvm_ComputerSystem, WmiError> {
        self.get_related("Msvm_ComputerSystem")
    }

    /// Gets the related Msvm_TerminalServiceSettingData object(s)
    pub fn get_related__terminal_service_setting_data(&self) -> Result<Msvm_TerminalServiceSettingData, WmiError> {
        self.get_related("Msvm_TerminalServiceSettingData")
    }

}

