// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_VpnConnectionRoute struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_VpnConnectionRoute {

/// 
    #[serde(rename = "AddressFamily")]
    pub address_family: Option<u16>,

/// 
    #[serde(rename = "DestinationPrefix")]
    pub destination_prefix: Option<String>,

/// 
    #[serde(rename = "RouteMetric")]
    pub route_metric: Option<u32>,
}

impl PS_VpnConnectionRoute {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            address_family: None,
            destination_prefix: None,
            route_metric: None,
        }
    }


    /// Sets the value of AddressFamily
    pub fn set_address_family(&mut self, value: u16) {
        self.address_family = Some(value);
    }

    /// Gets the value of AddressFamily
    pub fn get_address_family(&self) -> Option<&u16> {
        self.address_family.as_ref()
    }

    /// Sets the value of DestinationPrefix
    pub fn set_destination_prefix(&mut self, value: String) {
        self.destination_prefix = Some(value);
    }

    /// Gets the value of DestinationPrefix
    pub fn get_destination_prefix(&self) -> Option<&String> {
        self.destination_prefix.as_ref()
    }

    /// Sets the value of RouteMetric
    pub fn set_route_metric(&mut self, value: u32) {
        self.route_metric = Some(value);
    }

    /// Gets the value of RouteMetric
    pub fn get_route_metric(&self) -> Option<&u32> {
        self.route_metric.as_ref()
    }

/// 

    /// * `all_user_connection` -  (bool)
    /// * `connection_name` -  (String)
    /// * `destination_prefix` -  (String)
    /// * `pass_thru` -  (bool)
    /// * `route_metric` -  (u32)

    /// * `cmdlet_output` -  (MSFT_NetRoute)
    /// * `return_value` -  (u32)
    pub fn add(&self, connection_name: &String, destination_prefix: &String, route_metric: u32, pass_thru: bool, all_user_connection: bool, cmdlet_output: &mut MSFT_NetRoute) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DestinationPrefix".to_string(), value: destination_prefix.into() });
        args.push(MethodParameter { name: "RouteMetric".to_string(), value: route_metric.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });

        let result = self.invoke_method("Add", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `all_user_connection` -  (bool)
    /// * `connection_name` -  (String)
    /// * `destination_prefix` -  (String)
    /// * `pass_thru` -  (bool)

    /// * `cmdlet_output` -  (MSFT_NetRoute)
    /// * `return_value` -  (u32)
    pub fn remove(&self, connection_name: &String, destination_prefix: &String, all_user_connection: bool, pass_thru: bool, cmdlet_output: &mut MSFT_NetRoute) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ConnectionName".to_string(), value: connection_name.into() });
        args.push(MethodParameter { name: "DestinationPrefix".to_string(), value: destination_prefix.into() });
        args.push(MethodParameter { name: "AllUserConnection".to_string(), value: all_user_connection.into() });
        args.push(MethodParameter { name: "PassThru".to_string(), value: pass_thru.into() });

        let result = self.invoke_method("Remove", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

