// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2.TerminalServices
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_TSSessionDirectory struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_TSSessionDirectory {
    #[serde(flatten)]
    pub base: CIM_Setting,

/// 
    #[serde(rename = "GetLoadBalancingState")]
    pub get_load_balancing_state: Option<u32>,

/// 
    #[serde(rename = "GetServerWeight")]
    pub get_server_weight: Option<u32>,

/// 
    #[serde(rename = "GetTSRedirectorMode")]
    pub get_tsredirector_mode: Option<u32>,

/// 
    #[serde(rename = "PolicySourceLoadBalancing")]
    pub policy_source_load_balancing: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSessionDirectoryActive")]
    pub policy_source_session_directory_active: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSessionDirectoryClusterName")]
    pub policy_source_session_directory_cluster_name: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSessionDirectoryExposeServerIP")]
    pub policy_source_session_directory_expose_server_ip: Option<u32>,

/// 
    #[serde(rename = "PolicySourceSessionDirectoryLocation")]
    pub policy_source_session_directory_location: Option<u32>,

/// 
    #[serde(rename = "SessionDirectoryActive")]
    pub session_directory_active: Option<u32>,

/// 
    #[serde(rename = "SessionDirectoryClusterName")]
    pub session_directory_cluster_name: Option<String>,

/// 
    #[serde(rename = "SessionDirectoryExposeServerIP")]
    pub session_directory_expose_server_ip: Option<u32>,

/// 
    #[serde(rename = "SessionDirectoryIPAddress")]
    pub session_directory_ipaddress: Option<String>,

/// 
    #[serde(rename = "SessionDirectoryLocation")]
    pub session_directory_location: Option<String>,
}

impl Win32_TSSessionDirectory {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_Setting::new(),
            get_load_balancing_state: None,
            get_server_weight: None,
            get_tsredirector_mode: None,
            policy_source_load_balancing: None,
            policy_source_session_directory_active: None,
            policy_source_session_directory_cluster_name: None,
            policy_source_session_directory_expose_server_ip: None,
            policy_source_session_directory_location: None,
            session_directory_active: None,
            session_directory_cluster_name: None,
            session_directory_expose_server_ip: None,
            session_directory_ipaddress: None,
            session_directory_location: None,
        }
    }


    /// Sets the value of GetLoadBalancingState
    pub fn set_get_load_balancing_state(&mut self, value: u32) {
        self.get_load_balancing_state = Some(value);
    }

    /// Gets the value of GetLoadBalancingState
    pub fn get_get_load_balancing_state(&self) -> Option<&u32> {
        self.get_load_balancing_state.as_ref()
    }

    /// Sets the value of GetServerWeight
    pub fn set_get_server_weight(&mut self, value: u32) {
        self.get_server_weight = Some(value);
    }

    /// Gets the value of GetServerWeight
    pub fn get_get_server_weight(&self) -> Option<&u32> {
        self.get_server_weight.as_ref()
    }

    /// Sets the value of GetTSRedirectorMode
    pub fn set_get_tsredirector_mode(&mut self, value: u32) {
        self.get_tsredirector_mode = Some(value);
    }

    /// Gets the value of GetTSRedirectorMode
    pub fn get_get_tsredirector_mode(&self) -> Option<&u32> {
        self.get_tsredirector_mode.as_ref()
    }

    /// Sets the value of PolicySourceLoadBalancing
    pub fn set_policy_source_load_balancing(&mut self, value: u32) {
        self.policy_source_load_balancing = Some(value);
    }

    /// Gets the value of PolicySourceLoadBalancing
    pub fn get_policy_source_load_balancing(&self) -> Option<&u32> {
        self.policy_source_load_balancing.as_ref()
    }

    /// Sets the value of PolicySourceSessionDirectoryActive
    pub fn set_policy_source_session_directory_active(&mut self, value: u32) {
        self.policy_source_session_directory_active = Some(value);
    }

    /// Gets the value of PolicySourceSessionDirectoryActive
    pub fn get_policy_source_session_directory_active(&self) -> Option<&u32> {
        self.policy_source_session_directory_active.as_ref()
    }

    /// Sets the value of PolicySourceSessionDirectoryClusterName
    pub fn set_policy_source_session_directory_cluster_name(&mut self, value: u32) {
        self.policy_source_session_directory_cluster_name = Some(value);
    }

    /// Gets the value of PolicySourceSessionDirectoryClusterName
    pub fn get_policy_source_session_directory_cluster_name(&self) -> Option<&u32> {
        self.policy_source_session_directory_cluster_name.as_ref()
    }

    /// Sets the value of PolicySourceSessionDirectoryExposeServerIP
    pub fn set_policy_source_session_directory_expose_server_ip(&mut self, value: u32) {
        self.policy_source_session_directory_expose_server_ip = Some(value);
    }

    /// Gets the value of PolicySourceSessionDirectoryExposeServerIP
    pub fn get_policy_source_session_directory_expose_server_ip(&self) -> Option<&u32> {
        self.policy_source_session_directory_expose_server_ip.as_ref()
    }

    /// Sets the value of PolicySourceSessionDirectoryLocation
    pub fn set_policy_source_session_directory_location(&mut self, value: u32) {
        self.policy_source_session_directory_location = Some(value);
    }

    /// Gets the value of PolicySourceSessionDirectoryLocation
    pub fn get_policy_source_session_directory_location(&self) -> Option<&u32> {
        self.policy_source_session_directory_location.as_ref()
    }

    /// Sets the value of SessionDirectoryActive
    pub fn set_session_directory_active(&mut self, value: u32) {
        self.session_directory_active = Some(value);
    }

    /// Gets the value of SessionDirectoryActive
    pub fn get_session_directory_active(&self) -> Option<&u32> {
        self.session_directory_active.as_ref()
    }

    /// Sets the value of SessionDirectoryClusterName
    pub fn set_session_directory_cluster_name(&mut self, value: String) {
        self.session_directory_cluster_name = Some(value);
    }

    /// Gets the value of SessionDirectoryClusterName
    pub fn get_session_directory_cluster_name(&self) -> Option<&String> {
        self.session_directory_cluster_name.as_ref()
    }

    /// Sets the value of SessionDirectoryExposeServerIP
    pub fn set_session_directory_expose_server_ip(&mut self, value: u32) {
        self.session_directory_expose_server_ip = Some(value);
    }

    /// Gets the value of SessionDirectoryExposeServerIP
    pub fn get_session_directory_expose_server_ip(&self) -> Option<&u32> {
        self.session_directory_expose_server_ip.as_ref()
    }

    /// Sets the value of SessionDirectoryIPAddress
    pub fn set_session_directory_ipaddress(&mut self, value: String) {
        self.session_directory_ipaddress = Some(value);
    }

    /// Gets the value of SessionDirectoryIPAddress
    pub fn get_session_directory_ipaddress(&self) -> Option<&String> {
        self.session_directory_ipaddress.as_ref()
    }

    /// Sets the value of SessionDirectoryLocation
    pub fn set_session_directory_location(&mut self, value: String) {
        self.session_directory_location = Some(value);
    }

    /// Gets the value of SessionDirectoryLocation
    pub fn get_session_directory_location(&self) -> Option<&String> {
        self.session_directory_location.as_ref()
    }

/// 

    /// * `property_name` -  (String)
    /// * `value` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_session_directory_property(&self, property_name: &String, value: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "PropertyName".to_string(), value: property_name.into() });
        args.push(MethodParameter { name: "Value".to_string(), value: value.into() });
        self.invoke_method("SetSessionDirectoryProperty", &args)

    }


/// 

    /// * `session_directory_active` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_session_directory_active(&self, session_directory_active: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SessionDirectoryActive".to_string(), value: session_directory_active.into() });
        self.invoke_method("SetSessionDirectoryActive", &args)

    }


/// 

    /// * `session_directory_expose_server_ip` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_session_directory_expose_server_ip(&self, session_directory_expose_server_ip: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "SessionDirectoryExposeServerIP".to_string(), value: session_directory_expose_server_ip.into() });
        self.invoke_method("SetSessionDirectoryExposeServerIP", &args)

    }


/// 

    /// * `f_token_redirection` -  (u32)

    /// * `adapter_names` -  (String[])
    /// * `ipaddresses` -  (String[])
    /// * `net_con_name` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_redirectable_addresses(&self, f_token_redirection: u32, ipaddresses: &mut Vec<String>, adapter_names: &mut Vec<String>, net_con_name: &mut Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fTokenRedirection".to_string(), value: f_token_redirection.into() });

        let result = self.invoke_method("GetRedirectableAddresses", &args)?;
        let adapter_names = result.get_value("AdapterNames")?;
        let ipaddresses = result.get_value("IPAddresses")?;
        let net_con_name = result.get_value("NetConName")?;
        Ok(result.return_value)

    }


/// 

    /// * `f_token_redirection` -  (u32)
    /// * `ipaddresses` -  (String[])
    /// * `return_value` -  (u32)
    pub fn get_current_redirectable_addresses(&self, f_token_redirection: &mut u32, ipaddresses: &mut Vec<String>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetCurrentRedirectableAddresses", &[])?;
        let f_token_redirection = result.get_value("fTokenRedirection")?;
        let ipaddresses = result.get_value("IPAddresses")?;
        Ok(result.return_value)

    }


/// 

    /// * `f_token_redirection` -  (u32)
    /// * `ipaddresses` -  (String[])

    /// * `return_value` -  (u32)
    pub fn set_current_redirectable_addresses(&self, f_token_redirection: u32, ipaddresses: &Vec<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "fTokenRedirection".to_string(), value: f_token_redirection.into() });
        args.push(MethodParameter { name: "IPAddresses".to_string(), value: ipaddresses.into() });
        self.invoke_method("SetCurrentRedirectableAddresses", &args)

    }


/// 

    /// * `server_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn ping_session_directory(&self, server_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerName".to_string(), value: server_name.into() });
        self.invoke_method("PingSessionDirectory", &args)

    }


/// 

    /// * `state_value` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_load_balancing_state(&self, state_value: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "StateValue".to_string(), value: state_value.into() });
        self.invoke_method("SetLoadBalancingState", &args)

    }


/// 

    /// * `server_weight_value` -  (u32)

    /// * `return_value` -  (u32)
    pub fn set_server_weight(&self, server_weight_value: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ServerWeightValue".to_string(), value: server_weight_value.into() });
        self.invoke_method("SetServerWeight", &args)

    }


/// 

    /// * `user_disk_max_size_in_gb` -  (u32)
    /// * `user_disks_storage_url` -  (String)

    /// * `return_value` -  (u32)
    pub fn create_user_disk_template(&self, user_disks_storage_url: &String, user_disk_max_size_in_gb: u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UserDisksStorageUrl".to_string(), value: user_disks_storage_url.into() });
        args.push(MethodParameter { name: "UserDiskMaxSizeInGB".to_string(), value: user_disk_max_size_in_gb.into() });
        self.invoke_method("CreateUserDiskTemplate", &args)

    }


/// 

    /// * `uvhd_roaming_policy_xml` -  (String)
    /// * `uvhd_share_url` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable_user_vhd(&self, uvhd_share_url: &String, uvhd_roaming_policy_xml: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UvhdShareUrl".to_string(), value: uvhd_share_url.into() });
        args.push(MethodParameter { name: "UvhdRoamingPolicyXml".to_string(), value: uvhd_roaming_policy_xml.into() });
        self.invoke_method("EnableUserVhd", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable_user_vhd(&self) -> Result<(), WmiError> {
        self.invoke_method("DisableUserVhd", &[])

    }

}

