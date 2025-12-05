// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.HardwareManagement
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_PCSVDevice struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_PCSVDevice {
    #[serde(flatten)]
    pub base: CIM_PhysicalComputerSystemView,

/// 
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,

/// 
    #[serde(rename = "IPv4AddressOrigin")]
    pub ipv4_address_origin: Option<u16>,

/// 
    #[serde(rename = "IPv4DefaultGateway")]
    pub ipv4_default_gateway: Option<String>,

/// 
    #[serde(rename = "IPv4SubnetMask")]
    pub ipv4_subnet_mask: Option<String>,

/// 
    #[serde(rename = "LogFreeSpace")]
    pub log_free_space: Option<u16>,

/// 
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

/// 
    #[serde(rename = "SMBIOSGuid")]
    pub smbiosguid: Option<String>,

/// 
    #[serde(rename = "TargetAddress")]
    pub target_address: Option<String>,
}

impl MSFT_PCSVDevice {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_PhysicalComputerSystemView::new(),
            ipv4_address: None,
            ipv4_address_origin: None,
            ipv4_default_gateway: None,
            ipv4_subnet_mask: None,
            log_free_space: None,
            mac_address: None,
            smbiosguid: None,
            target_address: None,
        }
    }


    /// Sets the value of IPv4Address
    pub fn set_ipv4_address(&mut self, value: String) {
        self.ipv4_address = Some(value);
    }

    /// Gets the value of IPv4Address
    pub fn get_ipv4_address(&self) -> Option<&String> {
        self.ipv4_address.as_ref()
    }

    /// Sets the value of IPv4AddressOrigin
    pub fn set_ipv4_address_origin(&mut self, value: u16) {
        self.ipv4_address_origin = Some(value);
    }

    /// Gets the value of IPv4AddressOrigin
    pub fn get_ipv4_address_origin(&self) -> Option<&u16> {
        self.ipv4_address_origin.as_ref()
    }

    /// Sets the value of IPv4DefaultGateway
    pub fn set_ipv4_default_gateway(&mut self, value: String) {
        self.ipv4_default_gateway = Some(value);
    }

    /// Gets the value of IPv4DefaultGateway
    pub fn get_ipv4_default_gateway(&self) -> Option<&String> {
        self.ipv4_default_gateway.as_ref()
    }

    /// Sets the value of IPv4SubnetMask
    pub fn set_ipv4_subnet_mask(&mut self, value: String) {
        self.ipv4_subnet_mask = Some(value);
    }

    /// Gets the value of IPv4SubnetMask
    pub fn get_ipv4_subnet_mask(&self) -> Option<&String> {
        self.ipv4_subnet_mask.as_ref()
    }

    /// Sets the value of LogFreeSpace
    pub fn set_log_free_space(&mut self, value: u16) {
        self.log_free_space = Some(value);
    }

    /// Gets the value of LogFreeSpace
    pub fn get_log_free_space(&self) -> Option<&u16> {
        self.log_free_space.as_ref()
    }

    /// Sets the value of MacAddress
    pub fn set_mac_address(&mut self, value: String) {
        self.mac_address = Some(value);
    }

    /// Gets the value of MacAddress
    pub fn get_mac_address(&self) -> Option<&String> {
        self.mac_address.as_ref()
    }

    /// Sets the value of SMBIOSGuid
    pub fn set_smbiosguid(&mut self, value: String) {
        self.smbiosguid = Some(value);
    }

    /// Gets the value of SMBIOSGuid
    pub fn get_smbiosguid(&self) -> Option<&String> {
        self.smbiosguid.as_ref()
    }

    /// Sets the value of TargetAddress
    pub fn set_target_address(&mut self, value: String) {
        self.target_address = Some(value);
    }

    /// Gets the value of TargetAddress
    pub fn get_target_address(&self) -> Option<&String> {
        self.target_address.as_ref()
    }

/// 

    /// * `job` -  (CIM_ConcreteJob)
    /// * `one_time_boot_source` -  (String)
    /// * `persistent_boot_source` -  (String[])

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn change_boot_configuration(&self, one_time_boot_source: &String, persistent_boot_source: &Vec<String>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "OneTimeBootSource".to_string(), value: one_time_boot_source.into() });
        args.push(MethodParameter { name: "PersistentBootSource".to_string(), value: persistent_boot_source.into() });

        let result = self.invoke_method_with_job("ChangeBootConfiguration", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `ipv4_address` -  (String)
    /// * `ipv4_address_origin` -  (u16)
    /// * `ipv4_default_gateway` -  (String)
    /// * `ipv4_subnet_mask` -  (String)
    /// * `job` -  (CIM_ConcreteJob)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn change_network_configuration(&self, ipv4_address_origin: u16, ipv4_address: &String, ipv4_subnet_mask: &String, ipv4_default_gateway: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "IPv4AddressOrigin".to_string(), value: ipv4_address_origin.into() });
        args.push(MethodParameter { name: "IPv4Address".to_string(), value: ipv4_address.into() });
        args.push(MethodParameter { name: "IPv4SubnetMask".to_string(), value: ipv4_subnet_mask.into() });
        args.push(MethodParameter { name: "IPv4DefaultGateway".to_string(), value: ipv4_default_gateway.into() });

        let result = self.invoke_method_with_job("ChangeNetworkConfiguration", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `current_credential` -  (String)
    /// * `job` -  (CIM_ConcreteJob)
    /// * `new_password` -  (String)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `return_value` -  (u32)
    pub fn change_user_password(&self, current_credential: &String, new_password: &String, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "CurrentCredential".to_string(), value: current_credential.into() });
        args.push(MethodParameter { name: "NewPassword".to_string(), value: new_password.into() });

        let result = self.invoke_method_with_job("ChangeUserPassword", &args)?;
        let job = result.get_value("Job")?;
        Ok(result.return_value)

    }


/// 

    /// * `job` -  (CIM_ConcreteJob)

    /// * `job` -  (CIM_ConcreteJob)
    /// * `log_records` -  (MSFT_PCSVLogRecord[])
    /// * `return_value` -  (u32)
    pub fn read_log(&self, log_records: &mut Vec<MSFT_PCSVLogRecord>, job: &mut CIM_ConcreteJob, action: UserAction, percent_complete: uint, timeout: int) -> Result<(), WmiError> {
        let mut args = Vec::new();

        let result = self.invoke_method_with_job("ReadLog", &args)?;
        let job = result.get_value("Job")?;
        let log_records = result.get_value("LogRecords")?;
        Ok(result.return_value)

    }

}

