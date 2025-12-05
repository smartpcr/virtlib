// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.Storage
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_StorageQoSPolicy struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_StorageQoSPolicy {

/// 
    #[serde(rename = "BandwidthLimit")]
    pub bandwidth_limit: Option<u64>,

/// 
    #[serde(rename = "Name")]
    pub name: Option<String>,

/// 
    #[serde(rename = "ParentPolicy")]
    pub parent_policy: Option<String>,

/// 
    #[serde(rename = "PolicyId")]
    pub policy_id: Option<String>,

/// 
    #[serde(rename = "PolicyType")]
    pub policy_type: Option<u16>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<u16>,

/// 
    #[serde(rename = "ThroughputLimit")]
    pub throughput_limit: Option<u64>,

/// 
    #[serde(rename = "ThroughputReservation")]
    pub throughput_reservation: Option<u64>,
}

impl MSFT_StorageQoSPolicy {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bandwidth_limit: None,
            name: None,
            parent_policy: None,
            policy_id: None,
            policy_type: None,
            status: None,
            throughput_limit: None,
            throughput_reservation: None,
        }
    }


    /// Sets the value of BandwidthLimit
    pub fn set_bandwidth_limit(&mut self, value: u64) {
        self.bandwidth_limit = Some(value);
    }

    /// Gets the value of BandwidthLimit
    pub fn get_bandwidth_limit(&self) -> Option<&u64> {
        self.bandwidth_limit.as_ref()
    }

    /// Sets the value of Name
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    /// Gets the value of Name
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// Sets the value of ParentPolicy
    pub fn set_parent_policy(&mut self, value: String) {
        self.parent_policy = Some(value);
    }

    /// Gets the value of ParentPolicy
    pub fn get_parent_policy(&self) -> Option<&String> {
        self.parent_policy.as_ref()
    }

    /// Sets the value of PolicyId
    pub fn set_policy_id(&mut self, value: String) {
        self.policy_id = Some(value);
    }

    /// Gets the value of PolicyId
    pub fn get_policy_id(&self) -> Option<&String> {
        self.policy_id.as_ref()
    }

    /// Sets the value of PolicyType
    pub fn set_policy_type(&mut self, value: u16) {
        self.policy_type = Some(value);
    }

    /// Gets the value of PolicyType
    pub fn get_policy_type(&self) -> Option<&u16> {
        self.policy_type.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: u16) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&u16> {
        self.status.as_ref()
    }

    /// Sets the value of ThroughputLimit
    pub fn set_throughput_limit(&mut self, value: u64) {
        self.throughput_limit = Some(value);
    }

    /// Gets the value of ThroughputLimit
    pub fn get_throughput_limit(&self) -> Option<&u64> {
        self.throughput_limit.as_ref()
    }

    /// Sets the value of ThroughputReservation
    pub fn set_throughput_reservation(&mut self, value: u64) {
        self.throughput_reservation = Some(value);
    }

    /// Gets the value of ThroughputReservation
    pub fn get_throughput_reservation(&self) -> Option<&u64> {
        self.throughput_reservation.as_ref()
    }

/// 

    /// * `bandwidth_limit` -  (u64)
    /// * `limit` -  (u64)
    /// * `new_name` -  (String)
    /// * `reservation` -  (u64)

    /// * `return_value` -  (i32)
    pub fn set_attributes(&self, new_name: &String, limit: u64, reservation: u64, bandwidth_limit: u64) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "NewName".to_string(), value: new_name.into() });
        args.push(MethodParameter { name: "Limit".to_string(), value: limit.into() });
        args.push(MethodParameter { name: "Reservation".to_string(), value: reservation.into() });
        args.push(MethodParameter { name: "BandwidthLimit".to_string(), value: bandwidth_limit.into() });
        self.invoke_method("SetAttributes", &args)

    }


/// 

    /// * `return_value` -  (i32)
    pub fn delete_policy(&self) -> Result<(), WmiError> {
        self.invoke_method("DeletePolicy", &[])

    }

}

