// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSStorageDriver_FailurePredictFunction struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSStorageDriver_FailurePredictFunction {
    #[serde(flatten)]
    pub base: MSStorageDriver,

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl MSStorageDriver_FailurePredictFunction {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: MSStorageDriver::new(),
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

    /// * `allow` -  (bool)
    pub fn allow_performance_hit(&self, allow: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Allow".to_string(), value: allow.into() });
        self.invoke_method("AllowPerformanceHit", &args)

    }


/// 

    /// * `enable` -  (bool)
    pub fn enable_disable_hardware_failure_prediction(&self, enable: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });
        self.invoke_method("EnableDisableHardwareFailurePrediction", &args)

    }


/// 

    /// * `enable` -  (bool)
    /// * `period` -  (u32)
    pub fn enable_disable_failure_prediction_polling(&self, period: u32, enable: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Period".to_string(), value: period.into() });
        args.push(MethodParameter { name: "Enable".to_string(), value: enable.into() });
        self.invoke_method("EnableDisableFailurePredictionPolling", &args)

    }


/// 

    /// * `capability` -  (u32)
    pub fn get_failure_prediction_capability(&self, capability: &mut u32) -> Result<(), WmiError> {

        let result = self.invoke_method("GetFailurePredictionCapability", &[])?;
        let capability = result.get_value("Capability")?;
        Ok(result.return_value)

    }


/// 

    /// * `success` -  (bool)
    pub fn enable_offline_diags(&self, success: &mut bool) -> Result<(), WmiError> {

        let result = self.invoke_method("EnableOfflineDiags", &[])?;
        let success = result.get_value("Success")?;
        Ok(result.return_value)

    }


/// 

    /// * `log_address` -  (u8)
    /// * `sector_count` -  (u8)

    /// * `length` -  (u32)
    /// * `log_sectors` -  (u8[])
    pub fn read_log_sectors(&self, log_address: u8, sector_count: u8, length: &mut u32, log_sectors: &mut Vec<u8>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogAddress".to_string(), value: log_address.into() });
        args.push(MethodParameter { name: "SectorCount".to_string(), value: sector_count.into() });

        let result = self.invoke_method("ReadLogSectors", &args)?;
        let length = result.get_value("Length")?;
        let log_sectors = result.get_value("LogSectors")?;
        Ok(result.return_value)

    }


/// 

    /// * `length` -  (u32)
    /// * `log_address` -  (u8)
    /// * `log_sectors` -  (u8[])
    /// * `sector_count` -  (u8)

    /// * `success` -  (bool)
    pub fn write_log_sectors(&self, log_address: u8, sector_count: u8, length: u32, log_sectors: &Vec<u8>, success: &mut bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LogAddress".to_string(), value: log_address.into() });
        args.push(MethodParameter { name: "SectorCount".to_string(), value: sector_count.into() });
        args.push(MethodParameter { name: "Length".to_string(), value: length.into() });
        args.push(MethodParameter { name: "LogSectors".to_string(), value: log_sectors.into() });

        let result = self.invoke_method("WriteLogSectors", &args)?;
        let success = result.get_value("Success")?;
        Ok(result.return_value)

    }


/// 

    /// * `subcommand` -  (u8)

    /// * `return_code` -  (u32)
    pub fn execute_self_test(&self, subcommand: u8, return_code: &mut u32) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Subcommand".to_string(), value: subcommand.into() });

        let result = self.invoke_method("ExecuteSelfTest", &args)?;
        let return_code = result.get_value("ReturnCode")?;
        Ok(result.return_value)

    }

}

