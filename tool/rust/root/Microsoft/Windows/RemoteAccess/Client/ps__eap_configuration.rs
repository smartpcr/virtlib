// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.RemoteAccess.Client
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// PS_EapConfiguration struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PS_EapConfiguration {
}

impl PS_EapConfiguration {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
        }
    }


/// 

    /// * `use_winlogon_credential` -  (bool)

    /// * `cmdlet_output` -  (EapConfiguration)
    /// * `return_value` -  (u32)
    pub fn new_by_eap_ms_chapv2_auth(&self, use_winlogon_credential: bool, cmdlet_output: &mut EapConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UseWinlogonCredential".to_string(), value: use_winlogon_credential.into() });

        let result = self.invoke_method("NewByEapMsChapv2Auth", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `tls` -  (bool)
    /// * `user_certificate` -  (bool)
    /// * `verify_server_identity` -  (bool)

    /// * `cmdlet_output` -  (EapConfiguration)
    /// * `return_value` -  (u32)
    pub fn new_by_eap_tls_auth(&self, tls: bool, user_certificate: bool, verify_server_identity: bool, cmdlet_output: &mut EapConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Tls".to_string(), value: tls.into() });
        args.push(MethodParameter { name: "UserCertificate".to_string(), value: user_certificate.into() });
        args.push(MethodParameter { name: "VerifyServerIdentity".to_string(), value: verify_server_identity.into() });

        let result = self.invoke_method("NewByEapTlsAuth", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `ttls` -  (bool)
    /// * `tunnled_eap_auth_method` -  (String)
    /// * `tunnled_non_eap_auth_method` -  (String)
    /// * `use_winlogon_credential` -  (bool)

    /// * `cmdlet_output` -  (EapConfiguration)
    /// * `return_value` -  (u32)
    pub fn new_by_eap_ttls_auth(&self, ttls: bool, use_winlogon_credential: bool, tunnled_non_eap_auth_method: &String, tunnled_eap_auth_method: &String, cmdlet_output: &mut EapConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Ttls".to_string(), value: ttls.into() });
        args.push(MethodParameter { name: "UseWinlogonCredential".to_string(), value: use_winlogon_credential.into() });
        args.push(MethodParameter { name: "TunnledNonEapAuthMethod".to_string(), value: tunnled_non_eap_auth_method.into() });
        args.push(MethodParameter { name: "TunnledEapAuthMethod".to_string(), value: tunnled_eap_auth_method.into() });

        let result = self.invoke_method("NewByEapTtlsAuth", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }


/// 

    /// * `enable_nap` -  (bool)
    /// * `fast_reconnect` -  (bool)
    /// * `peap` -  (bool)
    /// * `tunnled_eap_auth_method` -  (String)
    /// * `verify_server_identity` -  (bool)

    /// * `cmdlet_output` -  (EapConfiguration)
    /// * `return_value` -  (u32)
    pub fn new_by_peap_auth(&self, peap: bool, verify_server_identity: bool, enable_nap: bool, fast_reconnect: bool, tunnled_eap_auth_method: &String, cmdlet_output: &mut EapConfiguration) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Peap".to_string(), value: peap.into() });
        args.push(MethodParameter { name: "VerifyServerIdentity".to_string(), value: verify_server_identity.into() });
        args.push(MethodParameter { name: "EnableNap".to_string(), value: enable_nap.into() });
        args.push(MethodParameter { name: "FastReconnect".to_string(), value: fast_reconnect.into() });
        args.push(MethodParameter { name: "TunnledEapAuthMethod".to_string(), value: tunnled_eap_auth_method.into() });

        let result = self.invoke_method("NewByPeapAuth", &args)?;
        let cmdlet_output = result.get_value("cmdletOutput")?;
        Ok(result.return_value)

    }

}

