// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbServerAlternativePort struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbServerAlternativePort {

/// 
    #[serde(rename = "Instances")]
    pub instances: Option<SmbServerAlternativePort_Instances>,

/// 
    #[serde(rename = "Port")]
    pub port: Option<u16>,

/// 
    #[serde(rename = "TransportType")]
    pub transport_type: Option<SmbServerAlternativePort_TransportType>,
}

impl MSFT_SmbServerAlternativePort {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            instances: None,
            port: None,
            transport_type: None,
        }
    }


    /// Sets the value of Instances
    pub fn set_instances(&mut self, value: SmbServerAlternativePort_Instances) {
        self.instances = Some(value);
    }

    /// Gets the value of Instances
    pub fn get_instances(&self) -> Option<&SmbServerAlternativePort_Instances> {
        self.instances.as_ref()
    }

    /// Sets the value of Port
    pub fn set_port(&mut self, value: u16) {
        self.port = Some(value);
    }

    /// Gets the value of Port
    pub fn get_port(&self) -> Option<&u16> {
        self.port.as_ref()
    }

    /// Sets the value of TransportType
    pub fn set_transport_type(&mut self, value: SmbServerAlternativePort_TransportType) {
        self.transport_type = Some(value);
    }

    /// Gets the value of TransportType
    pub fn get_transport_type(&self) -> Option<&SmbServerAlternativePort_TransportType> {
        self.transport_type.as_ref()
    }

/// 

    /// * `enable_instances` -  (u32)
    /// * `port` -  (u16)
    /// * `transport_type` -  (u32)

    /// * `created_alternative_port` -  (MSFT_SmbServerAlternativePort)
    /// * `return_value` -  (u32)
    pub fn new_server_alternative_port(&self, transport_type: u32, port: u16, enable_instances: u32, created_alternative_port: &mut MSFT_SmbServerAlternativePort) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TransportType".to_string(), value: transport_type.into() });
        args.push(MethodParameter { name: "Port".to_string(), value: port.into() });
        args.push(MethodParameter { name: "EnableInstances".to_string(), value: enable_instances.into() });

        let result = self.invoke_method("NewServerAlternativePort", &args)?;
        let created_alternative_port = result.get_value("CreatedAlternativePort")?;
        Ok(result.return_value)

    }


/// 

    /// * `disable_instances` -  (u32)
    /// * `enable_instances` -  (u32)
    /// * `port` -  (u16)
    /// * `transport_type` -  (u32)

    /// * `created_alternative_port` -  (MSFT_SmbServerAlternativePort)
    /// * `return_value` -  (u32)
    pub fn set_server_alternative_port(&self, transport_type: u32, port: u16, enable_instances: u32, disable_instances: u32, created_alternative_port: &mut MSFT_SmbServerAlternativePort) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "TransportType".to_string(), value: transport_type.into() });
        args.push(MethodParameter { name: "Port".to_string(), value: port.into() });
        args.push(MethodParameter { name: "EnableInstances".to_string(), value: enable_instances.into() });
        args.push(MethodParameter { name: "DisableInstances".to_string(), value: disable_instances.into() });

        let result = self.invoke_method("SetServerAlternativePort", &args)?;
        let created_alternative_port = result.get_value("CreatedAlternativePort")?;
        Ok(result.return_value)

    }

}

