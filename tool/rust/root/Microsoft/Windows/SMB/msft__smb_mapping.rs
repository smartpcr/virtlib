// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbMapping {

/// 
    #[serde(rename = "BlockNTLM")]
    pub block_ntlm: Option<bool>,

/// 
    #[serde(rename = "CompressNetworkTraffic")]
    pub compress_network_traffic: Option<i32>,

/// 
    #[serde(rename = "GlobalMapping")]
    pub global_mapping: Option<bool>,

/// 
    #[serde(rename = "LocalPath")]
    pub local_path: Option<String>,

/// 
    #[serde(rename = "QuicPort")]
    pub quic_port: Option<u16>,

/// 
    #[serde(rename = "RdmaPort")]
    pub rdma_port: Option<u16>,

/// 
    #[serde(rename = "RemotePath")]
    pub remote_path: Option<String>,

/// 
    #[serde(rename = "RequireIntegrity")]
    pub require_integrity: Option<bool>,

/// 
    #[serde(rename = "RequirePrivacy")]
    pub require_privacy: Option<bool>,

/// 
    #[serde(rename = "SkipCertificateCheck")]
    pub skip_certificate_check: Option<bool>,

/// 
    #[serde(rename = "Status")]
    pub status: Option<SmbMapping_Status>,

/// 
    #[serde(rename = "TcpPort")]
    pub tcp_port: Option<u16>,

/// 
    #[serde(rename = "TransportType")]
    pub transport_type: Option<SmbMapping_TransportType>,

/// 
    #[serde(rename = "UseWriteThrough")]
    pub use_write_through: Option<bool>,
}

impl MSFT_SmbMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            block_ntlm: None,
            compress_network_traffic: None,
            global_mapping: None,
            local_path: None,
            quic_port: None,
            rdma_port: None,
            remote_path: None,
            require_integrity: None,
            require_privacy: None,
            skip_certificate_check: None,
            status: None,
            tcp_port: None,
            transport_type: None,
            use_write_through: None,
        }
    }


    /// Sets the value of BlockNTLM
    pub fn set_block_ntlm(&mut self, value: bool) {
        self.block_ntlm = Some(value);
    }

    /// Gets the value of BlockNTLM
    pub fn get_block_ntlm(&self) -> Option<&bool> {
        self.block_ntlm.as_ref()
    }

    /// Sets the value of CompressNetworkTraffic
    pub fn set_compress_network_traffic(&mut self, value: i32) {
        self.compress_network_traffic = Some(value);
    }

    /// Gets the value of CompressNetworkTraffic
    pub fn get_compress_network_traffic(&self) -> Option<&i32> {
        self.compress_network_traffic.as_ref()
    }

    /// Sets the value of GlobalMapping
    pub fn set_global_mapping(&mut self, value: bool) {
        self.global_mapping = Some(value);
    }

    /// Gets the value of GlobalMapping
    pub fn get_global_mapping(&self) -> Option<&bool> {
        self.global_mapping.as_ref()
    }

    /// Sets the value of LocalPath
    pub fn set_local_path(&mut self, value: String) {
        self.local_path = Some(value);
    }

    /// Gets the value of LocalPath
    pub fn get_local_path(&self) -> Option<&String> {
        self.local_path.as_ref()
    }

    /// Sets the value of QuicPort
    pub fn set_quic_port(&mut self, value: u16) {
        self.quic_port = Some(value);
    }

    /// Gets the value of QuicPort
    pub fn get_quic_port(&self) -> Option<&u16> {
        self.quic_port.as_ref()
    }

    /// Sets the value of RdmaPort
    pub fn set_rdma_port(&mut self, value: u16) {
        self.rdma_port = Some(value);
    }

    /// Gets the value of RdmaPort
    pub fn get_rdma_port(&self) -> Option<&u16> {
        self.rdma_port.as_ref()
    }

    /// Sets the value of RemotePath
    pub fn set_remote_path(&mut self, value: String) {
        self.remote_path = Some(value);
    }

    /// Gets the value of RemotePath
    pub fn get_remote_path(&self) -> Option<&String> {
        self.remote_path.as_ref()
    }

    /// Sets the value of RequireIntegrity
    pub fn set_require_integrity(&mut self, value: bool) {
        self.require_integrity = Some(value);
    }

    /// Gets the value of RequireIntegrity
    pub fn get_require_integrity(&self) -> Option<&bool> {
        self.require_integrity.as_ref()
    }

    /// Sets the value of RequirePrivacy
    pub fn set_require_privacy(&mut self, value: bool) {
        self.require_privacy = Some(value);
    }

    /// Gets the value of RequirePrivacy
    pub fn get_require_privacy(&self) -> Option<&bool> {
        self.require_privacy.as_ref()
    }

    /// Sets the value of SkipCertificateCheck
    pub fn set_skip_certificate_check(&mut self, value: bool) {
        self.skip_certificate_check = Some(value);
    }

    /// Gets the value of SkipCertificateCheck
    pub fn get_skip_certificate_check(&self) -> Option<&bool> {
        self.skip_certificate_check.as_ref()
    }

    /// Sets the value of Status
    pub fn set_status(&mut self, value: SmbMapping_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&SmbMapping_Status> {
        self.status.as_ref()
    }

    /// Sets the value of TcpPort
    pub fn set_tcp_port(&mut self, value: u16) {
        self.tcp_port = Some(value);
    }

    /// Gets the value of TcpPort
    pub fn get_tcp_port(&self) -> Option<&u16> {
        self.tcp_port.as_ref()
    }

    /// Sets the value of TransportType
    pub fn set_transport_type(&mut self, value: SmbMapping_TransportType) {
        self.transport_type = Some(value);
    }

    /// Gets the value of TransportType
    pub fn get_transport_type(&self) -> Option<&SmbMapping_TransportType> {
        self.transport_type.as_ref()
    }

    /// Sets the value of UseWriteThrough
    pub fn set_use_write_through(&mut self, value: bool) {
        self.use_write_through = Some(value);
    }

    /// Gets the value of UseWriteThrough
    pub fn get_use_write_through(&self) -> Option<&bool> {
        self.use_write_through.as_ref()
    }

/// 

    /// * `force` -  (bool)
    /// * `global_mapping` -  (bool)
    /// * `update_profile` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove(&self, update_profile: bool, force: bool, global_mapping: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "UpdateProfile".to_string(), value: update_profile.into() });
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        args.push(MethodParameter { name: "GlobalMapping".to_string(), value: global_mapping.into() });
        self.invoke_method("Remove", &args)

    }


/// 

    /// * `block_ntlm` -  (bool)
    /// * `compress_network_traffic` -  (bool)
    /// * `credential` -  (String)
    /// * `global_mapping` -  (bool)
    /// * `home_folder` -  (bool)
    /// * `local_path` -  (String)
    /// * `password` -  (String)
    /// * `persistent` -  (bool)
    /// * `quic_port` -  (u16)
    /// * `rdma_port` -  (u16)
    /// * `remote_path` -  (String)
    /// * `require_integrity` -  (bool)
    /// * `require_privacy` -  (bool)
    /// * `save_credentials` -  (bool)
    /// * `skip_certificate_check` -  (bool)
    /// * `tcp_port` -  (u16)
    /// * `transport_type` -  (u32)
    /// * `user_name` -  (String)
    /// * `use_write_through` -  (bool)

    /// * `created_mapping` -  (MSFT_SmbMapping)
    /// * `return_value` -  (u32)
    pub fn create(&self, local_path: &String, remote_path: &String, user_name: &String, password: &String, persistent: bool, save_credentials: bool, home_folder: bool, created_mapping: &mut MSFT_SmbMapping, require_integrity: Option<bool>, require_privacy: Option<bool>, use_write_through: Option<bool>, transport_type: Option<u32>, skip_certificate_check: Option<bool>, compress_network_traffic: Option<bool>, global_mapping: Option<bool>, block_ntlm: Option<bool>, tcp_port: Option<u16>, quic_port: Option<u16>, rdma_port: Option<u16>, credential: &Option<String>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LocalPath".to_string(), value: local_path.into() });
        args.push(MethodParameter { name: "RemotePath".to_string(), value: remote_path.into() });
        args.push(MethodParameter { name: "UserName".to_string(), value: user_name.into() });
        args.push(MethodParameter { name: "Password".to_string(), value: password.into() });
        args.push(MethodParameter { name: "Persistent".to_string(), value: persistent.into() });
        args.push(MethodParameter { name: "SaveCredentials".to_string(), value: save_credentials.into() });
        args.push(MethodParameter { name: "HomeFolder".to_string(), value: home_folder.into() });
        if let Some(val) = require_integrity {
            args.push(MethodParameter { name: "RequireIntegrity".to_string(), value: val.into() });
        }
        if let Some(val) = require_privacy {
            args.push(MethodParameter { name: "RequirePrivacy".to_string(), value: val.into() });
        }
        if let Some(val) = use_write_through {
            args.push(MethodParameter { name: "UseWriteThrough".to_string(), value: val.into() });
        }
        if let Some(val) = transport_type {
            args.push(MethodParameter { name: "TransportType".to_string(), value: val.into() });
        }
        if let Some(val) = skip_certificate_check {
            args.push(MethodParameter { name: "SkipCertificateCheck".to_string(), value: val.into() });
        }
        if let Some(val) = compress_network_traffic {
            args.push(MethodParameter { name: "CompressNetworkTraffic".to_string(), value: val.into() });
        }
        if let Some(val) = global_mapping {
            args.push(MethodParameter { name: "GlobalMapping".to_string(), value: val.into() });
        }
        if let Some(val) = block_ntlm {
            args.push(MethodParameter { name: "BlockNTLM".to_string(), value: val.into() });
        }
        if let Some(val) = tcp_port {
            args.push(MethodParameter { name: "TcpPort".to_string(), value: val.into() });
        }
        if let Some(val) = quic_port {
            args.push(MethodParameter { name: "QuicPort".to_string(), value: val.into() });
        }
        if let Some(val) = rdma_port {
            args.push(MethodParameter { name: "RdmaPort".to_string(), value: val.into() });
        }
        if let Some(val) = credential {
            args.push(MethodParameter { name: "Credential".to_string(), value: val.into() });
        }

        let result = self.invoke_method("Create", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }

}

