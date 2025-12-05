// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_QueryLBPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_QueryLBPolicy {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,

/// Load Balance Policy that is currently being used by iSCSI Initiator - one element for each session on the adapter
    #[serde(rename = "LoadBalancePolicies")]
    pub load_balance_policies: Vec<ISCSI_Supported_LB_Policies>,

/// 
    #[serde(rename = "Reserved")]
    pub reserved: Option<u32>,

/// Number of elements in LoadBalancePolicies array
    #[serde(rename = "SessionCount")]
    pub session_count: Option<u32>,

/// Id that is globally unique to each instance of each adapter. Using the address of the Adapter Extension is a good idea.
    #[serde(rename = "UniqueAdapterId")]
    pub unique_adapter_id: Option<u64>,
}

impl MSiSCSI_QueryLBPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            active: None,
            instance_name: None,
            load_balance_policies: Vec::new(),
            reserved: None,
            session_count: None,
            unique_adapter_id: None,
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

    /// Sets the value of LoadBalancePolicies
    pub fn set_load_balance_policies(&mut self, value: Vec<ISCSI_Supported_LB_Policies>) {
        self.load_balance_policies = value;
    }

    /// Gets the value of LoadBalancePolicies
    pub fn get_load_balance_policies(&self) -> &Vec<ISCSI_Supported_LB_Policies> {
        &self.load_balance_policies
    }

    /// Sets the value of Reserved
    pub fn set_reserved(&mut self, value: u32) {
        self.reserved = Some(value);
    }

    /// Gets the value of Reserved
    pub fn get_reserved(&self) -> Option<&u32> {
        self.reserved.as_ref()
    }

    /// Sets the value of SessionCount
    pub fn set_session_count(&mut self, value: u32) {
        self.session_count = Some(value);
    }

    /// Gets the value of SessionCount
    pub fn get_session_count(&self) -> Option<&u32> {
        self.session_count.as_ref()
    }

    /// Sets the value of UniqueAdapterId
    pub fn set_unique_adapter_id(&mut self, value: u64) {
        self.unique_adapter_id = Some(value);
    }

    /// Gets the value of UniqueAdapterId
    pub fn get_unique_adapter_id(&self) -> Option<&u64> {
        self.unique_adapter_id.as_ref()
    }
}

