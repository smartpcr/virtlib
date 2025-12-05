// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_LB_Operations struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_LB_Operations {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSiSCSI_LB_Operations {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
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

/// Sets Load Balance Policy for the iSCSI Initiator

    /// * `load_balance_policies` - New Load Balance policy to be set (ISCSI_Supported_LB_Policies)

    /// * `status` - Status of the operation (u32)
    pub fn set_load_balance_policy(&self, load_balance_policies: ISCSI_Supported_LB_Policies, status: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LoadBalancePolicies".to_string(), value: load_balance_policies.into() });

        let result = self.invoke_method("SetLoadBalancePolicy", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

