// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// ISCSI_Supported_LB_Policies struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ISCSI_Supported_LB_Policies {

/// Number of entries in MSiSCSI_Paths array
    #[serde(rename = "iSCSI_PathCount")]
    pub i_scsi__path_count: Option<u32>,

/// Describes iSCSI Initiator Paths
    #[serde(rename = "iSCSI_Paths")]
    pub i_scsi__paths: Vec<ISCSI_Path>,

/// Load Balance policy supported by the iSCSI Initiator
    #[serde(rename = "LoadBalancePolicy")]
    pub load_balance_policy: Option<Policies_LoadBalancePolicy>,

/// Id that is unique to this session within this adapter. 
    #[serde(rename = "UniqueSessionId")]
    pub unique_session_id: Option<u64>,
}

impl ISCSI_Supported_LB_Policies {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            i_scsi__path_count: None,
            i_scsi__paths: Vec::new(),
            load_balance_policy: None,
            unique_session_id: None,
        }
    }


    /// Sets the value of iSCSI_PathCount
    pub fn set_i_scsi__path_count(&mut self, value: u32) {
        self.i_scsi__path_count = Some(value);
    }

    /// Gets the value of iSCSI_PathCount
    pub fn get_i_scsi__path_count(&self) -> Option<&u32> {
        self.i_scsi__path_count.as_ref()
    }

    /// Sets the value of iSCSI_Paths
    pub fn set_i_scsi__paths(&mut self, value: Vec<ISCSI_Path>) {
        self.i_scsi__paths = value;
    }

    /// Gets the value of iSCSI_Paths
    pub fn get_i_scsi__paths(&self) -> &Vec<ISCSI_Path> {
        &self.i_scsi__paths
    }

    /// Sets the value of LoadBalancePolicy
    pub fn set_load_balance_policy(&mut self, value: Policies_LoadBalancePolicy) {
        self.load_balance_policy = Some(value);
    }

    /// Gets the value of LoadBalancePolicy
    pub fn get_load_balance_policy(&self) -> Option<&Policies_LoadBalancePolicy> {
        self.load_balance_policy.as_ref()
    }

    /// Sets the value of UniqueSessionId
    pub fn set_unique_session_id(&mut self, value: u64) {
        self.unique_session_id = Some(value);
    }

    /// Gets the value of UniqueSessionId
    pub fn get_unique_session_id(&self) -> Option<&u64> {
        self.unique_session_id.as_ref()
    }
}

