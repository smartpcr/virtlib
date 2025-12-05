// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DASiteTableEntry struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DASiteTableEntry {
    #[serde(flatten)]
    pub base: MSFT_NetSettingData,

/// 
    #[serde(rename = "ADSite")]
    pub adsite: Option<String>,

/// 
    #[serde(rename = "EntryPointIPAddress")]
    pub entry_point_ipaddress: Option<String>,

/// 
    #[serde(rename = "EntryPointName")]
    pub entry_point_name: Option<String>,

/// 
    #[serde(rename = "EntryPointRange")]
    pub entry_point_range: Vec<String>,

/// 
    #[serde(rename = "GslbIP")]
    pub gslb_ip: Option<String>,

/// 
    #[serde(rename = "IPHttpsProfile")]
    pub iphttps_profile: Option<String>,

/// 
    #[serde(rename = "PolicyStore")]
    pub policy_store: Option<String>,

/// 
    #[serde(rename = "State")]
    pub state: Option<u32>,

/// 
    #[serde(rename = "TeredoServerIP")]
    pub teredo_server_ip: Option<String>,
}

impl MSFT_DASiteTableEntry {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetSettingData::new(),
            adsite: None,
            entry_point_ipaddress: None,
            entry_point_name: None,
            entry_point_range: Vec::new(),
            gslb_ip: None,
            iphttps_profile: None,
            policy_store: None,
            state: None,
            teredo_server_ip: None,
        }
    }


    /// Sets the value of ADSite
    pub fn set_adsite(&mut self, value: String) {
        self.adsite = Some(value);
    }

    /// Gets the value of ADSite
    pub fn get_adsite(&self) -> Option<&String> {
        self.adsite.as_ref()
    }

    /// Sets the value of EntryPointIPAddress
    pub fn set_entry_point_ipaddress(&mut self, value: String) {
        self.entry_point_ipaddress = Some(value);
    }

    /// Gets the value of EntryPointIPAddress
    pub fn get_entry_point_ipaddress(&self) -> Option<&String> {
        self.entry_point_ipaddress.as_ref()
    }

    /// Sets the value of EntryPointName
    pub fn set_entry_point_name(&mut self, value: String) {
        self.entry_point_name = Some(value);
    }

    /// Gets the value of EntryPointName
    pub fn get_entry_point_name(&self) -> Option<&String> {
        self.entry_point_name.as_ref()
    }

    /// Sets the value of EntryPointRange
    pub fn set_entry_point_range(&mut self, value: Vec<String>) {
        self.entry_point_range = value;
    }

    /// Gets the value of EntryPointRange
    pub fn get_entry_point_range(&self) -> &Vec<String> {
        &self.entry_point_range
    }

    /// Sets the value of GslbIP
    pub fn set_gslb_ip(&mut self, value: String) {
        self.gslb_ip = Some(value);
    }

    /// Gets the value of GslbIP
    pub fn get_gslb_ip(&self) -> Option<&String> {
        self.gslb_ip.as_ref()
    }

    /// Sets the value of IPHttpsProfile
    pub fn set_iphttps_profile(&mut self, value: String) {
        self.iphttps_profile = Some(value);
    }

    /// Gets the value of IPHttpsProfile
    pub fn get_iphttps_profile(&self) -> Option<&String> {
        self.iphttps_profile.as_ref()
    }

    /// Sets the value of PolicyStore
    pub fn set_policy_store(&mut self, value: String) {
        self.policy_store = Some(value);
    }

    /// Gets the value of PolicyStore
    pub fn get_policy_store(&self) -> Option<&String> {
        self.policy_store.as_ref()
    }

    /// Sets the value of State
    pub fn set_state(&mut self, value: u32) {
        self.state = Some(value);
    }

    /// Gets the value of State
    pub fn get_state(&self) -> Option<&u32> {
        self.state.as_ref()
    }

    /// Sets the value of TeredoServerIP
    pub fn set_teredo_server_ip(&mut self, value: String) {
        self.teredo_server_ip = Some(value);
    }

    /// Gets the value of TeredoServerIP
    pub fn get_teredo_server_ip(&self) -> Option<&String> {
        self.teredo_server_ip.as_ref()
    }

/// 

    /// * `entry_point_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn enable(&self, entry_point_name: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "EntryPointName".to_string(), value: entry_point_name.into() });
        self.invoke_method("Enable", &args)

    }


/// 

    /// * `return_value` -  (u32)
    pub fn disable(&self) -> Result<(), WmiError> {
        self.invoke_method("Disable", &[])

    }


/// 

    /// * `new_name` -  (String)
    /// * `pass_thru` -  (bool)

    /// * `output_object` -  (MSFT_DASiteTableEntry)
    /// * `return_value` -  (u32)
    pub fn rename(&self, new_name: &String, pass_thru: bool, output_object: &mut MSFT_DASiteTableEntry) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Rename", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }


/// 

    /// * `gslb_ip` -  (bool)
    /// * `iphttps_profile` -  (bool)
    /// * `pass_thru` -  (bool)
    /// * `teredo_server_ip` -  (bool)

    /// * `output_object` -  (MSFT_DASiteTableEntry)
    /// * `return_value` -  (u32)
    pub fn reset(&self, teredo_server_ip: bool, iphttps_profile: bool, gslb_ip: bool, pass_thru: bool, output_object: &mut MSFT_DASiteTableEntry) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TeredoServerIP".to_string(), value: teredo_server_ip.into() });
        args.push(MethodParameter { name: "IPHttpsProfile".to_string(), value: iphttps_profile.into() });
        args.push(MethodParameter { name: "GslbIP".to_string(), value: gslb_ip.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Reset", &args)?;
        let output_object = result.get_value("OutputObject")?;
        Ok(result.return_value)

    }

}

