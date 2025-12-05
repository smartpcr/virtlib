// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.msdtc
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DtcNetworkSettingTask struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DtcNetworkSettingTask {
}

impl MSFT_DtcNetworkSettingTask {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `dtc_name` -  (String)

    /// * `cmdlet_output` -  (DtcNetworkSettings)
    /// * `return_value` -  (u32)
    pub fn get(&self, dtc_name: &String, cmdlet_output: &mut DtcNetworkSettings) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });

        let result = self.invoke_method("Get", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `disable_network_access` -  (bool)
    /// * `dtc_name` -  (String)

    /// * `return_value` -  (u32)
    pub fn set_by_disable_network(&self, dtc_name: &String, disable_network_access: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "DisableNetworkAccess".to_string(), value: disable_network_access.into() });
        self.invoke_method("SetByDisableNetwork", &args)

    }


/// 

    /// * `authentication_level` -  (String)
    /// * `dtc_name` -  (String)
    /// * `inbound_transactions_enabled` -  (bool)
    /// * `lutransactions_enabled` -  (bool)
    /// * `outbound_transactions_enabled` -  (bool)
    /// * `remote_administration_access_enabled` -  (bool)
    /// * `remote_client_access_enabled` -  (bool)
    /// * `xatransactions_enabled` -  (bool)

    /// * `return_value` -  (u32)
    pub fn set_by_network_settings(&self, dtc_name: &String, inbound_transactions_enabled: bool, outbound_transactions_enabled: bool, remote_client_access_enabled: bool, remote_administration_access_enabled: bool, xatransactions_enabled: bool, lutransactions_enabled: bool, authentication_level: &String) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "DtcName".to_string(), value: dtc_name.into() });
        args.push(MethodParameter { name: "InboundTransactionsEnabled".to_string(), value: inbound_transactions_enabled.into() });
        args.push(MethodParameter { name: "OutboundTransactionsEnabled".to_string(), value: outbound_transactions_enabled.into() });
        args.push(MethodParameter { name: "RemoteClientAccessEnabled".to_string(), value: remote_client_access_enabled.into() });
        args.push(MethodParameter { name: "RemoteAdministrationAccessEnabled".to_string(), value: remote_administration_access_enabled.into() });
        args.push(MethodParameter { name: "XATransactionsEnabled".to_string(), value: xatransactions_enabled.into() });
        args.push(MethodParameter { name: "LUTransactionsEnabled".to_string(), value: lutransactions_enabled.into() });
        args.push(MethodParameter { name: "AuthenticationLevel".to_string(), value: authentication_level.into() });
        self.invoke_method("SetByNetworkSettings", &args)

    }

}

