// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.StandardCimv2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_NetLbfoTeam struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_NetLbfoTeam {
    #[serde(flatten)]
    pub base: MSFT_NetImPlatTeam,

/// 416
    #[serde(rename = "LacpTimer")]
    pub lacp_timer: Option<u32>,

/// 12
    #[serde(rename = "LoadBalancingAlgorithm")]
    pub load_balancing_algorithm: Option<u32>,

/// 13
    #[serde(rename = "Status")]
    pub status: Option<u32>,

/// 11
    #[serde(rename = "TeamingMode")]
    pub teaming_mode: Option<u32>,
}

impl MSFT_NetLbfoTeam {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSFT_NetImPlatTeam::new(),
            lacp_timer: None,
            load_balancing_algorithm: None,
            status: None,
            teaming_mode: None,
        }
    }


    /// Sets the value of LacpTimer
    pub fn set_lacp_timer(&mut self, value: u32) {
        self.lacp_timer = Some(value);
    }

    /// Gets the value of LacpTimer
    pub fn get_lacp_timer(&self) -> Option<&u32> {
        self.lacp_timer.as_ref()
    }

    /// Sets the value of LoadBalancingAlgorithm
    pub fn set_load_balancing_algorithm(&mut self, value: u32) {
        self.load_balancing_algorithm = Some(value);
    }

    /// Gets the value of LoadBalancingAlgorithm
    pub fn get_load_balancing_algorithm(&self) -> Option<&u32> {
        self.load_balancing_algorithm.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u32) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u32> {
        self.status.as_ref()
    }

    /// Sets the value of TeamingMode
    pub fn set_teaming_mode(&mut self, value: u32) {
        self.teaming_mode = Some(value);
    }

    /// Gets the value of TeamingMode
    pub fn get_teaming_mode(&self) -> Option<&u32> {
        self.teaming_mode.as_ref()
    }

/// 14

    /// * `name` -  (String)
    /// * `new_name` -  (String)

    /// * `cmdlet_output` -  (MSFT_NetLbfoTeam)
    /// * `return_value` -  (u32)
    pub fn rename(&self, name: &String, new_name: &String, cmdlet_output: &mut MSFT_NetLbfoTeam) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Name".to_string(), value: name.into() });
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });

        let result = self.invoke_method("Rename", &args)?;
        let cmdlet_output = result.get_value("CmdletOutput")?;
        Ok(result.return_value)

    }

}

