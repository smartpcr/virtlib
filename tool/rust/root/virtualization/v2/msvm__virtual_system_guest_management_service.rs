// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Msvm_VirtualSystemGuestManagementService struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Msvm_VirtualSystemGuestManagementService {
    #[serde(flatten)]
    pub base: CIM_Service,
}

impl Msvm_VirtualSystemGuestManagementService {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Service::new(),
        }
    }


/// 

    /// * `vm_id` -  (String)

    /// * `current_update_id` - The current updated ID. (u64)
    /// * `return_value` -  (u32)
    /// * `settings` - String represents VTL2 settings in JSON format. (u8[])
    pub fn get_vtl2_settings(&self, vm_id: &String, settings: &mut Vec<u8>, current_update_id: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });

        let result = self.invoke_method("GetVtl2Settings", &args)?;
        let current_update_id = result.get_value("CurrentUpdateId")?;
        let settings = result.get_value("Settings")?;
        Ok(result.return_value)

    }


/// 

    /// * `current_update_id` -  (u64)
    /// * `settings` -  (u8[])
    /// * `vm_id` -  (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn modify_vtl2_settings(&self, vm_id: &String, settings: &Vec<u8>, current_update_id: u64, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "CurrentUpdateId".to_string(), value: current_update_id.into() });

        let result = self.invoke_method_with_job("ModifyVtl2Settings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `vm_id` -  (String)

    /// * `current_update_id` - The current updated ID. (u64)
    /// * `return_value` -  (u32)
    /// * `settings` - String represents VTL2 settings in JSON format. (String)
    pub fn query_vtl2_settings(&self, vm_id: &String, settings: &mut String, current_update_id: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });

        let result = self.invoke_method("QueryVtl2Settings", &args)?;
        let current_update_id = result.get_value("CurrentUpdateId")?;
        let settings = result.get_value("Settings")?;
        Ok(result.return_value)

    }


/// 

    /// * `current_update_id` -  (u64)
    /// * `settings` -  (String)
    /// * `vm_id` -  (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn update_vtl2_settings(&self, vm_id: &String, settings: &String, current_update_id: u64, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "CurrentUpdateId".to_string(), value: current_update_id.into() });

        let result = self.invoke_method_with_job("UpdateVtl2Settings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `namespace` -  (String)
    /// * `vm_id` -  (String)

    /// * `current_update_id` - The current updated ID. (u64)
    /// * `return_value` -  (u32)
    /// * `settings` - Bytes representing the settings. (u8[])
    pub fn get_management_vtl_settings(&self, vm_id: &String, namespace: &String, settings: &mut Vec<u8>, current_update_id: &mut u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });

        let result = self.invoke_method("GetManagementVtlSettings", &args)?;
        let current_update_id = result.get_value("CurrentUpdateId")?;
        let settings = result.get_value("Settings")?;
        Ok(result.return_value)

    }


/// 

    /// * `current_update_id` -  (u64)
    /// * `namespace` -  (String)
    /// * `settings` -  (u8[])
    /// * `vm_id` -  (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn set_management_vtl_settings(&self, vm_id: &String, namespace: &String, settings: &Vec<u8>, current_update_id: u64, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });
        args.push(MethodParameter { name: "Settings".to_string(), value: settings.into() });
        args.push(MethodParameter { name: "CurrentUpdateId".to_string(), value: current_update_id.into() });

        let result = self.invoke_method_with_job("SetManagementVtlSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `namespace` -  (String)
    /// * `vm_id` -  (String)

    /// * `job` - May contain a reference to the ConcreteJob created to track the state transition initiated by the method invocation. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn remove_management_vtl_settings(&self, vm_id: &String, namespace: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Namespace".to_string(), value: namespace.into() });

        let result = self.invoke_method_with_job("RemoveManagementVtlSettings", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `options` -  (u32)
    /// * `timeout_hint_secs` -  (u16)
    /// * `vm_id` -  (String)

    /// * `job` - If the operation is long running, then optionally a job may be returned. (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn reload_management_vtl(&self, vm_id: &String, options: u32, timeout_hint_secs: u16, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "VmId".to_string(), value: vm_id.into() });
        args.push(MethodParameter { name: "Options".to_string(), value: options.into() });
        args.push(MethodParameter { name: "TimeoutHintSecs".to_string(), value: timeout_hint_secs.into() });

        let result = self.invoke_method_with_job("ReloadManagementVtl", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }

}

