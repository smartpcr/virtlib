// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ServerManager
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_ServerManagerDeploymentTasks struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_ServerManagerDeploymentTasks {
}

impl MSFT_ServerManagerDeploymentTasks {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)

    /// * `enumeration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    /// * `server_components` -  (MSFT_ServerManagerServerComponent[])
    pub fn get_server_components_async(&self, request_guid: MSFT_ServerManagerRequestGuid, enumeration_state: &mut MSFT_ServerManagerRequestState, server_components: &mut Vec<MSFT_ServerManagerServerComponent>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });

        let result = self.invoke_method("GetServerComponentsAsync", &args)?;
        let enumeration_state = result.get_value("EnumerationState")?;
        let server_components = result.get_value("ServerComponents")?;
        Ok(result.return_value)

    }


/// 

    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)
    /// * `vhd_path` -  (String)

    /// * `enumeration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    /// * `server_components` -  (MSFT_ServerManagerServerComponent[])
    pub fn get_server_components_vhd_async(&self, request_guid: MSFT_ServerManagerRequestGuid, vhd_path: &String, enumeration_state: &mut MSFT_ServerManagerRequestState, server_components: &mut Vec<MSFT_ServerManagerServerComponent>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "VhdPath".to_string(), value: vhd_path.into() });

        let result = self.invoke_method("GetServerComponentsVhdAsync", &args)?;
        let enumeration_state = result.get_value("EnumerationState")?;
        let server_components = result.get_value("ServerComponents")?;
        Ok(result.return_value)

    }


/// 

    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)

    /// * `enumeration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    /// * `server_components` -  (MSFT_ServerManagerServerComponent[])
    pub fn get_enumeration_request_state(&self, request_guid: MSFT_ServerManagerRequestGuid, enumeration_state: &mut MSFT_ServerManagerRequestState, server_components: &mut Vec<MSFT_ServerManagerServerComponent>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });

        let result = self.invoke_method("GetEnumerationRequestState", &args)?;
        let enumeration_state = result.get_value("EnumerationState")?;
        let server_components = result.get_value("ServerComponents")?;
        Ok(result.return_value)

    }


/// 

    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)
    /// * `scan_for_updates` -  (bool)
    /// * `server_component_descriptors` -  (MSFT_ServerManagerServerComponentDescriptor[])
    /// * `source` -  (String[])

    /// * `alteration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    pub fn add_server_component_async(&self, request_guid: MSFT_ServerManagerRequestGuid, source: &Vec<String>, scan_for_updates: bool, server_component_descriptors: &Vec<MSFT_ServerManagerServerComponentDescriptor>, alteration_state: &mut MSFT_ServerManagerRequestState) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "Source".to_string(), value: source.into() });
        args.push(MethodParameter { name: "ScanForUpdates".to_string(), value: scan_for_updates.into() });
        args.push(MethodParameter { name: "ServerComponentDescriptors".to_string(), value: server_component_descriptors.into() });

        let result = self.invoke_method("AddServerComponentAsync", &args)?;
        let alteration_state = result.get_value("AlterationState")?;
        Ok(result.return_value)

    }


/// 

    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)
    /// * `scan_for_updates` -  (bool)
    /// * `server_component_descriptors` -  (MSFT_ServerManagerServerComponentDescriptor[])
    /// * `source` -  (String[])
    /// * `vhd_path` -  (String)

    /// * `alteration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    pub fn add_server_component_vhd_async(&self, request_guid: MSFT_ServerManagerRequestGuid, source: &Vec<String>, scan_for_updates: bool, server_component_descriptors: &Vec<MSFT_ServerManagerServerComponentDescriptor>, vhd_path: &String, alteration_state: &mut MSFT_ServerManagerRequestState) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "Source".to_string(), value: source.into() });
        args.push(MethodParameter { name: "ScanForUpdates".to_string(), value: scan_for_updates.into() });
        args.push(MethodParameter { name: "ServerComponentDescriptors".to_string(), value: server_component_descriptors.into() });
        args.push(MethodParameter { name: "VhdPath".to_string(), value: vhd_path.into() });

        let result = self.invoke_method("AddServerComponentVhdAsync", &args)?;
        let alteration_state = result.get_value("AlterationState")?;
        Ok(result.return_value)

    }


/// 

    /// * `delete_components` -  (bool)
    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)
    /// * `server_component_descriptors` -  (MSFT_ServerManagerServerComponentDescriptor[])

    /// * `alteration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    pub fn remove_server_component_async(&self, request_guid: MSFT_ServerManagerRequestGuid, delete_components: bool, server_component_descriptors: &Vec<MSFT_ServerManagerServerComponentDescriptor>, alteration_state: &mut MSFT_ServerManagerRequestState) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "DeleteComponents".to_string(), value: delete_components.into() });
        args.push(MethodParameter { name: "ServerComponentDescriptors".to_string(), value: server_component_descriptors.into() });

        let result = self.invoke_method("RemoveServerComponentAsync", &args)?;
        let alteration_state = result.get_value("AlterationState")?;
        Ok(result.return_value)

    }


/// 

    /// * `delete_components` -  (bool)
    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)
    /// * `server_component_descriptors` -  (MSFT_ServerManagerServerComponentDescriptor[])
    /// * `vhd_path` -  (String)

    /// * `alteration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    pub fn remove_server_component_vhd_async(&self, request_guid: MSFT_ServerManagerRequestGuid, delete_components: bool, server_component_descriptors: &Vec<MSFT_ServerManagerServerComponentDescriptor>, vhd_path: &String, alteration_state: &mut MSFT_ServerManagerRequestState) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "DeleteComponents".to_string(), value: delete_components.into() });
        args.push(MethodParameter { name: "ServerComponentDescriptors".to_string(), value: server_component_descriptors.into() });
        args.push(MethodParameter { name: "VhdPath".to_string(), value: vhd_path.into() });

        let result = self.invoke_method("RemoveServerComponentVhdAsync", &args)?;
        let alteration_state = result.get_value("AlterationState")?;
        Ok(result.return_value)

    }


/// 

    /// * `keep_alteration_state_on_restart_required` -  (bool)
    /// * `request_guid` -  (MSFT_ServerManagerRequestGuid)

    /// * `alteration_state` -  (MSFT_ServerManagerRequestState)
    /// * `return_value` -  (u32)
    /// * `server_components` -  (MSFT_ServerManagerServerComponent[])
    pub fn get_alteration_request_state(&self, request_guid: MSFT_ServerManagerRequestGuid, keep_alteration_state_on_restart_required: bool, alteration_state: &mut MSFT_ServerManagerRequestState, server_components: &mut Vec<MSFT_ServerManagerServerComponent>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestGuid".to_string(), value: request_guid.into() });
        args.push(MethodParameter { name: "KeepAlterationStateOnRestartRequired".to_string(), value: keep_alteration_state_on_restart_required.into() });

        let result = self.invoke_method("GetAlterationRequestState", &args)?;
        let alteration_state = result.get_value("AlterationState")?;
        let server_components = result.get_value("ServerComponents")?;
        Ok(result.return_value)

    }

}

