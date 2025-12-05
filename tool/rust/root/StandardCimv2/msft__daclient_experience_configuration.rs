// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DAClientExperienceConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DAClientExperienceConfiguration {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "CorporateResources")]
    pub corporate_resources: Vec<String>,

/// 
    #[serde(rename = "CustomCommands")]
    pub custom_commands: Vec<String>,

/// 
    #[serde(rename = "ForceTunneling")]
    pub force_tunneling: Option<u32>,

/// 
    #[serde(rename = "FriendlyName")]
    pub friendly_name: Option<String>,

/// 
    #[serde(rename = "GslbFqdn")]
    pub gslb_fqdn: Option<String>,

/// 
    #[serde(rename = "IPsecTunnelEndpoints")]
    pub ipsec_tunnel_endpoints: Vec<String>,

/// 
    #[serde(rename = "ManualEntryPointSelectionAllowed")]
    pub manual_entry_point_selection_allowed: Option<bool>,

/// 
    #[serde(rename = "PassiveMode")]
    pub passive_mode: Option<bool>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "PreferLocalNamesAllowed")]
    pub prefer_local_names_allowed: Option<bool>,

/// 
    #[serde(rename = "SupportEmail")]
    pub support_email: Option<String>,

/// 
    #[serde(rename = "UserInterface")]
    pub user_interface: Option<bool>,
}

impl MSFT_DAClientExperienceConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            corporate_resources: Vec::new(),
            custom_commands: Vec::new(),
            force_tunneling: None,
            friendly_name: None,
            gslb_fqdn: None,
            ipsec_tunnel_endpoints: Vec::new(),
            manual_entry_point_selection_allowed: None,
            passive_mode: None,
            policy_store: None,
            prefer_local_names_allowed: None,
            support_email: None,
            user_interface: None,
        }
    }


    /// Sets the value of CorporateResources
    pub fn set_corporate_resources(&mut self, value: Vec<String>) {
        self.corporate_resources = value;
    }

    /// Gets the value of CorporateResources
    pub fn get_corporate_resources(&self) -> &Vec<String> {
        &self.corporate_resources
    }

    /// Sets the value of CustomCommands
    pub fn set_custom_commands(&mut self, value: Vec<String>) {
        self.custom_commands = value;
    }

    /// Gets the value of CustomCommands
    pub fn get_custom_commands(&self) -> &Vec<String> {
        &self.custom_commands
    }

    /// Sets the value of ForceTunneling
    pub fn set_force_tunneling(&mut self, value: u32) {
        self.force_tunneling = Some(value);
    }

    /// Gets the value of ForceTunneling
    pub fn get_force_tunneling(&self) -> Option<&u32> {
        self.force_tunneling.as_ref()
    }

    /// Sets the value of FriendlyName
    pub fn set_friendly_name(&mut self, value: String) {
        self.friendly_name = Some(value);
    }

    /// Gets the value of FriendlyName
    pub fn get_friendly_name(&self) -> Option<&String> {
        self.friendly_name.as_ref()
    }

    /// Sets the value of GslbFqdn
    pub fn set_gslb_fqdn(&mut self, value: String) {
        self.gslb_fqdn = Some(value);
    }

    /// Gets the value of GslbFqdn
    pub fn get_gslb_fqdn(&self) -> Option<&String> {
        self.gslb_fqdn.as_ref()
    }

    /// Sets the value of IPsecTunnelEndpoints
    pub fn set_ipsec_tunnel_endpoints(&mut self, value: Vec<String>) {
        self.ipsec_tunnel_endpoints = value;
    }

    /// Gets the value of IPsecTunnelEndpoints
    pub fn get_ipsec_tunnel_endpoints(&self) -> &Vec<String> {
        &self.ipsec_tunnel_endpoints
    }

    /// Sets the value of ManualEntryPointSelectionAllowed
    pub fn set_manual_entry_point_selection_allowed(&mut self, value: bool) {
        self.manual_entry_point_selection_allowed = Some(value);
    }

    /// Gets the value of ManualEntryPointSelectionAllowed
    pub fn get_manual_entry_point_selection_allowed(&self) -> Option<&bool> {
        self.manual_entry_point_selection_allowed.as_ref()
    }

    /// Sets the value of PassiveMode
    pub fn set_passive_mode(&mut self, value: bool) {
        self.passive_mode = Some(value);
    }

    /// Gets the value of PassiveMode
    pub fn get_passive_mode(&self) -> Option<&bool> {
        self.passive_mode.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of PreferLocalNamesAllowed
    pub fn set_prefer_local_names_allowed(&mut self, value: bool) {
        self.prefer_local_names_allowed = Some(value);
    }

    /// Gets the value of PreferLocalNamesAllowed
    pub fn get_prefer_local_names_allowed(&self) -> Option<&bool> {
        self.prefer_local_names_allowed.as_ref()
    }

    /// Sets the value of SupportEmail
    pub fn set_support_email(&mut self, value: String) {
        self.support_email = Some(value);
    }

    /// Gets the value of SupportEmail
    pub fn get_support_email(&self) -> Option<&String> {
        self.support_email.as_ref()
    }

    /// Sets the value of UserInterface
    pub fn set_user_interface(&mut self, value: bool) {
        self.user_interface = Some(value);
    }

    /// Gets the value of UserInterface
    pub fn get_user_interface(&self) -> Option<&bool> {
        self.user_interface.as_ref()
    }

/// 

    /// * `corporate_resources` -  (bool)
    /// * `custom_commands` -  (bool)
    /// * `force_tunneling` -  (bool)
    /// * `friendly_name` -  (bool)
    /// * `gslb_fqdn` -  (bool)
    /// * `ipsec_tunnel_endpoints` -  (bool)
    /// * `manual_entry_point_selection_allowed` -  (bool)
    /// * `passive_mode` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `prefer_local_names_allowed` -  (bool)
    /// * `support_email` -  (bool)
    /// * `user_interface` -  (bool)

    /// * `output_object` -  (MSFT_DAClientExperienceConfiguration)
    /// * `return_value` -  (u32)
    pub fn reset(&self, corporate_resources: bool, ipsec_tunnel_endpoints: bool, prefer_local_names_allowed: bool, user_interface: bool, support_email: bool, friendly_name: bool, passive_mode: bool, custom_commands: bool, manual_entry_point_selection_allowed: bool, gslb_fqdn: bool, force_tunneling: bool, pass_thru: bool, output_object: &mut MSFT_DAClientExperienceConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CorporateResources".to_string(), value: corporate_resources.into() });
        args.push(MethodParameter { name: "IPsecTunnelEndpoints".to_string(), value: ipsec_tunnel_endpoints.into() });
        args.push(MethodParameter { name: "PreferLocalNamesAllowed".to_string(), value: prefer_local_names_allowed.into() });
        args.push(MethodParameter { name: "UserInterface".to_string(), value: user_interface.into() });
        args.push(MethodParameter { name: "SupportEmail".to_string(), value: support_email.into() });
        args.push(MethodParameter { name: "FriendlyName".to_string(), value: friendly_name.into() });
        args.push(MethodParameter { name: "PassiveMode".to_string(), value: passive_mode.into() });
        args.push(MethodParameter { name: "CustomCommands".to_string(), value: custom_commands.into() });
        args.push(MethodParameter { name: "ManualEntryPointSelectionAllowed".to_string(), value: manual_entry_point_selection_allowed.into() });
        args.push(MethodParameter { name: "GslbFqdn".to_string(), value: gslb_fqdn.into() });
        args.push(MethodParameter { name: "ForceTunneling".to_string(), value: force_tunneling.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

