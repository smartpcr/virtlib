// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSiSCSI_ManagementOperations struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSiSCSI_ManagementOperations {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSiSCSI_ManagementOperations {
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

/// Perform an ICMP ping

    /// * `address` - IP address to ping (ISCSI_IP_Address)
    /// * `request_count` - Number of requests to send (u32)
    /// * `request_size` - Number of bytes in each request (u32)
    /// * `timeout` - Number of ms to wait for response (u32)

    /// * `responses_received` - Number of responses received (u32)
    /// * `status` - Status code resulting from operation (ManagementOperations_Status)
    pub fn ping_ipaddress(&self, request_count: u32, request_size: u32, timeout: u32, address: ISCSI_IP_Address, status: &mut ManagementOperations_Status, responses_received: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "RequestCount".to_string(), value: request_count.into() });
        args.push(MethodParameter { name: "RequestSize".to_string(), value: request_size.into() });
        args.push(MethodParameter { name: "Timeout".to_string(), value: timeout.into() });
        args.push(MethodParameter { name: "Address".to_string(), value: address.into() });

        let result = self.invoke_method("PingIPAddress", &args)?;
        let responses_received = result.get_value("ResponsesReceived")?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

