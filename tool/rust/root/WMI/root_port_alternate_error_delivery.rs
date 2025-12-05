// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.WMI
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// RootPortAlternateErrorDelivery struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RootPortAlternateErrorDelivery {

/// 
    #[serde(rename = "Active")]
    pub active: Option<bool>,

/// 
    #[serde(rename = "InstanceName")]
    pub instance_name: Option<String>,
}

impl RootPortAlternateErrorDelivery {
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

/// Change root port delivery from NMI to SCI

    /// * `bus` - The Bus number of the root port. (u8)
    /// * `device` - The Device number of the root port. (u8)
    /// * `function` - The Function number of the root port. (u8)
    /// * `segment` - The segment number of the root port. (u16)

    /// * `status` - Status of the method (u8)
    pub fn enable_alternate_error_delivery(&self, segment: u16, bus: u8, device: u8, function: u8, status: &mut u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Segment".to_string(), value: segment.into() });
        args.push(MethodParameter { name: "Bus".to_string(), value: bus.into() });
        args.push(MethodParameter { name: "Device".to_string(), value: device.into() });
        args.push(MethodParameter { name: "Function".to_string(), value: function.into() });

        let result = self.invoke_method("EnableAlternateErrorDelivery", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// Change root port delivery from SCI to NMI

    /// * `bus` - The Bus number of the root port. (u8)
    /// * `device` - The Device number of the root port. (u8)
    /// * `function` - The Function number of the root port. (u8)
    /// * `segment` - The segment number of the root port. (u16)

    /// * `status` - Status of the method (u8)
    pub fn disable_alternate_error_delivery(&self, segment: u16, bus: u8, device: u8, function: u8, status: &mut u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Segment".to_string(), value: segment.into() });
        args.push(MethodParameter { name: "Bus".to_string(), value: bus.into() });
        args.push(MethodParameter { name: "Device".to_string(), value: device.into() });
        args.push(MethodParameter { name: "Function".to_string(), value: function.into() });

        let result = self.invoke_method("DisableAlternateErrorDelivery", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }


/// Reenable error delivery after an error occurs

    /// * `bus` - The Bus number of the root port. (u8)
    /// * `device` - The Device number of the root port. (u8)
    /// * `function` - The Function number of the root port. (u8)
    /// * `segment` - The segment number of the root port. (u16)

    /// * `status` - Status of the method (u8)
    pub fn reenable_error_delivery(&self, segment: u16, bus: u8, device: u8, function: u8, status: &mut u8) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Segment".to_string(), value: segment.into() });
        args.push(MethodParameter { name: "Bus".to_string(), value: bus.into() });
        args.push(MethodParameter { name: "Device".to_string(), value: device.into() });
        args.push(MethodParameter { name: "Function".to_string(), value: function.into() });

        let result = self.invoke_method("ReenableErrorDelivery", &args)?;
        let status = result.get_value("Status")?;
        Ok(result.return_value)

    }

}

