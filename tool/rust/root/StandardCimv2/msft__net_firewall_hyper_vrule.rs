// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetFirewallHyperVRule struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetFirewallHyperVRule {
    #[serde(flatten)]
    pub base: CIM_PolicyRule,

/// 
    #[serde(rename = "Action")]
    pub action: Option<u16>,

/// 
    #[serde(rename = "Direction")]
    pub direction: Option<u16>,

/// 
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,

/// 
    #[serde(rename = "EnforcementStatus")]
    pub enforcement_status: Option<u16>,

/// 
    #[serde(rename = "LocalAddresses")]
    pub local_addresses: Vec<String>,

/// 
    #[serde(rename = "LocalPorts")]
    pub local_ports: Vec<String>,

/// 
    #[serde(rename = "PolicyStoreSourceType")]
    pub policy_store_source_type: Option<u16>,

/// 
    #[serde(rename = "PortStatuses")]
    pub port_statuses: Vec<MSFT_NetFirewallHyperVRulePortStatus>,

/// 
    #[serde(rename = "Profiles")]
    pub profiles: Option<u16>,

/// 
    #[serde(rename = "Protocol")]
    pub protocol: Option<String>,

/// 
    #[serde(rename = "RemoteAddresses")]
    pub remote_addresses: Vec<String>,

/// 
    #[serde(rename = "RemotePorts")]
    pub remote_ports: Vec<String>,

/// 
    #[serde(rename = "RulePriority")]
    pub rule_priority: Option<u16>,

/// 
    #[serde(rename = "VMCreatorId")]
    pub vmcreator_id: Option<String>,
}

impl MSFT_NetFirewallHyperVRule {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PolicyRule::new(),
            action: None,
            direction: None,
            display_name: None,
            enforcement_status: None,
            local_addresses: Vec::new(),
            local_ports: Vec::new(),
            policy_store_source_type: None,
            port_statuses: Vec::new(),
            profiles: None,
            protocol: None,
            remote_addresses: Vec::new(),
            remote_ports: Vec::new(),
            rule_priority: None,
            vmcreator_id: None,
        }
    }


    /// Sets the value of Action
    pub fn set_action(&mut self, value: u16) {
        self.action = Some(value);
    }

    /// Gets the value of Action
    pub fn get_action(&self) -> Option<&u16> {
        self.action.as_ref()
    }

    /// Sets the value of Direction
    pub fn set_direction(&mut self, value: u16) {
        self.direction = Some(value);
    }

    /// Gets the value of Direction
    pub fn get_direction(&self) -> Option<&u16> {
        self.direction.as_ref()
    }

    /// Sets the value of DisplayName
    pub fn set_display_name(&mut self, value: String) {
        self.display_name = Some(value);
    }

    /// Gets the value of DisplayName
    pub fn get_display_name(&self) -> Option<&String> {
        self.display_name.as_ref()
    }

    /// Sets the value of EnforcementStatus
    pub fn set_enforcement_status(&mut self, value: u16) {
        self.enforcement_status = Some(value);
    }

    /// Gets the value of EnforcementStatus
    pub fn get_enforcement_status(&self) -> Option<&u16> {
        self.enforcement_status.as_ref()
    }

    /// Sets the value of LocalAddresses
    pub fn set_local_addresses(&mut self, value: Vec<String>) {
        self.local_addresses = value;
    }

    /// Gets the value of LocalAddresses
    pub fn get_local_addresses(&self) -> &Vec<String> {
        &self.local_addresses
    }

    /// Sets the value of LocalPorts
    pub fn set_local_ports(&mut self, value: Vec<String>) {
        self.local_ports = value;
    }

    /// Gets the value of LocalPorts
    pub fn get_local_ports(&self) -> &Vec<String> {
        &self.local_ports
    }

    /// Sets the value of PolicyStoreSourceType
    pub fn set_policy_store_source_type(&mut self, value: u16) {
        self.policy_store_source_type = Some(value);
    }

    /// Gets the value of PolicyStoreSourceType
    pub fn get_policy_store_source_type(&self) -> Option<&u16> {
        self.policy_store_source_type.as_ref()
    }

    /// Sets the value of PortStatuses
    pub fn set_port_statuses(&mut self, value: Vec<MSFT_NetFirewallHyperVRulePortStatus>) {
        self.port_statuses = value;
    }

    /// Gets the value of PortStatuses
    pub fn get_port_statuses(&self) -> &Vec<MSFT_NetFirewallHyperVRulePortStatus> {
        &self.port_statuses
    }

    /// Sets the value of Profiles
    pub fn set_profiles(&mut self, value: u16) {
        self.profiles = Some(value);
    }

    /// Gets the value of Profiles
    pub fn get_profiles(&self) -> Option<&u16> {
        self.profiles.as_ref()
    }

    /// Sets the value of Protocol
    pub fn set_protocol(&mut self, value: String) {
        self.protocol = Some(value);
    }

    /// Gets the value of Protocol
    pub fn get_protocol(&self) -> Option<&String> {
        self.protocol.as_ref()
    }

    /// Sets the value of RemoteAddresses
    pub fn set_remote_addresses(&mut self, value: Vec<String>) {
        self.remote_addresses = value;
    }

    /// Gets the value of RemoteAddresses
    pub fn get_remote_addresses(&self) -> &Vec<String> {
        &self.remote_addresses
    }

    /// Sets the value of RemotePorts
    pub fn set_remote_ports(&mut self, value: Vec<String>) {
        self.remote_ports = value;
    }

    /// Gets the value of RemotePorts
    pub fn get_remote_ports(&self) -> &Vec<String> {
        &self.remote_ports
    }

    /// Sets the value of RulePriority
    pub fn set_rule_priority(&mut self, value: u16) {
        self.rule_priority = Some(value);
    }

    /// Gets the value of RulePriority
    pub fn get_rule_priority(&self) -> Option<&u16> {
        self.rule_priority.as_ref()
    }

    /// Sets the value of VMCreatorId
    pub fn set_vmcreator_id(&mut self, value: String) {
        self.vmcreator_id = Some(value);
    }

    /// Gets the value of VMCreatorId
    pub fn get_vmcreator_id(&self) -> Option<&String> {
        self.vmcreator_id.as_ref()
    }

/// 

    /// * `return_value` -  (u32)
    pub fn enable(&self) -> Result<(), WmiError> {
        self.invoke_method("Enable", &[])

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `new_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        self.invoke_method("Rename", &args)

    }


/// 

    /// * `dependents` -  (CIM_ManagedSystemElement[])
    /// * `return_value` -  (u32)
    pub fn enumerate_full(&self, dependents: &mut Vec<CIM_ManagedSystemElement>) -> Result<(), WmiError> {

        let result = self.invoke_method("EnumerateFull", &[])?;
        let dependents = result.get_value("Dependents")?;
        Ok(result.return_value)

    }

}

