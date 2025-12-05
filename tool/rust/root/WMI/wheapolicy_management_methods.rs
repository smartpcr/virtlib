// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// WHEAPolicyManagementMethods struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WHEAPolicyManagementMethods {
    #[serde(flatten)]
    pub base: WHEA,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl WHEAPolicyManagementMethods {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: WHEA::new(),
            active: None,
            instance_name: None,
        }
    }


    /// Sets the value of Active
    pub fn set_active(&mut self, value: bool) {
        self.active = Some(value);
    }

    /// Gets the value of Active
    pub fn get_active(&self) -> Option<&bool> {
        self.active.as_ref()
    }

    /// Sets the value of InstanceName
    pub fn set_instance_name(&mut self, value: String) {
        self.instance_name = Some(value);
    }

    /// Gets the value of InstanceName
    pub fn get_instance_name(&self) -> Option<&String> {
        self.instance_name.as_ref()
    }

/// 

    /// * `count` -  (u32)
    /// * `length` -  (u32)
    /// * `status` -  (u32)
    /// * `values` -  (u8[])
    pub fn whea_get_all_policy_rtn(&self, status: &mut u32, count: &mut u32, length: &mut u32, values: &mut Vec<u8>) -> Result<(), WmiError> {

        let result = self.invoke_method("WheaGetAllPolicyRtn", &[])?;
        let count = result.get_value("Count")?;
        let length = result.get_value("Length")?;
        let status = result.get_value("Status")?;
        let values = result.get_value("Values")?;
        Ok(result.return_value)

    }


/// 

    /// * `type` -  (u32)

    /// * `status` -  (u32)
    /// * `value` -  (u32)
    pub fn whea_get_policy_rtn(&self, status: &mut u32, type: Option<u32>, value: &mut Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = type {
            args.push(MethodParameter { name: "Type".to_string(), value: val.into() });
        }

        let result = self.invoke_method("WheaGetPolicyRtn", &args)?;
        let status = result.get_value("Status")?;
        let value = result.get_value("Value")?;
        Ok(result.return_value)

    }


/// 

    /// * `type` -  (u32)
    /// * `value` -  (u32)

    /// * `status` -  (u32)
    pub fn whea_set_policy_rtn(&self, status: &mut u32, type: Option<u32>, value: Option<u32>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        if let Some(val) = type {
            args.push(MethodParameter { name: "Type".to_string(), value: val.into() });
        }
        if let Some(val) = value {
            args.push(MethodParameter { name: "Value".to_string(), value: val.into() });
        }

        let result = self.invoke_method("WheaSetPolicyRtn", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `status` -  (u32)
    pub fn whea_commit_policy_rtn(&self, status: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("WheaCommitPolicyRtn", &[])?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// 

    /// * `status` -  (u32)
    pub fn whea_reset_policy_rtn(&self, status: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("WheaResetPolicyRtn", &[])?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

