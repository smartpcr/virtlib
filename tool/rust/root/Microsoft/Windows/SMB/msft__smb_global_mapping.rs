// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.SMB
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_SmbGlobalMapping struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_SmbGlobalMapping {

/// 
    #[serde(rename = "BlockNTLM")]
    pub block_ntlm: Option<bool>,

/// 
    #[serde(rename = "CompressNetworkTraffic")]
    pub compress_network_traffic: Option<i32>,

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
    pub status: Option<SmbGlobalMapping_Status>,

/// 
    #[serde(rename = "TcpPort")]
    pub tcp_port: Option<u16>,

/// 
    #[serde(rename = "TransportType")]
    pub transport_type: Option<SmbGlobalMapping_TransportType>,

/// 
    #[serde(rename = "UseWriteThrough")]
    pub use_write_through: Option<bool>,
}

impl MSFT_SmbGlobalMapping {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            block_ntlm: None,
            compress_network_traffic: None,
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
    pub fn set_status(&mut self, value: SmbGlobalMapping_Status) {
        self.status = Some(value);
    }

    /// Gets the value of Status
    pub fn get_status(&self) -> Option<&SmbGlobalMapping_Status> {
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
    pub fn set_transport_type(&mut self, value: SmbGlobalMapping_TransportType) {
        self.transport_type = Some(value);
    }

    /// Gets the value of TransportType
    pub fn get_transport_type(&self) -> Option<&SmbGlobalMapping_TransportType> {
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

    /// * `block_ntlm` -  (bool)
    /// * `compress_network_traffic` -  (bool)
    /// * `credential` -  (String)
    /// * `deny_access` -  (String[])
    /// * `full_access` -  (String[])
    /// * `local_path` -  (String)
    /// * `persistent` -  (bool)
    /// * `quic_port` -  (u16)
    /// * `rdma_port` -  (u16)
    /// * `remote_path` -  (String)
    /// * `require_integrity` -  (bool)
    /// * `require_privacy` -  (bool)
    /// * `skip_certificate_check` -  (bool)
    /// * `tcp_port` -  (u16)
    /// * `transport_type` -  (u32)
    /// * `use_write_through` -  (bool)

    /// * `created_mapping` -  (MSFT_SmbGlobalMapping)
    /// * `return_value` -  (u32)
    pub fn create(&self, local_path: &String, remote_path: &String, credential: &String, persistent: bool, created_mapping: &mut MSFT_SmbGlobalMapping, require_integrity: Option<bool>, require_privacy: Option<bool>, full_access: &Option<Vec<String>>, deny_access: &Option<Vec<String>>, use_write_through: Option<bool>, transport_type: Option<u32>, skip_certificate_check: Option<bool>, compress_network_traffic: Option<bool>, block_ntlm: Option<bool>, tcp_port: Option<u16>, quic_port: Option<u16>, rdma_port: Option<u16>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "LocalPath".to_string(), value: local_path.into() });
        args.push(MethodParameter { name: "RemotePath".to_string(), value: remote_path.into() });
        args.push(MethodParameter { name: "Credential".to_string(), value: credential.into() });
        args.push(MethodParameter { name: "Persistent".to_string(), value: persistent.into() });
        if let Some(val) = require_integrity {
            args.push(MethodParameter { name: "RequireIntegrity".to_string(), value: val.into() });
        }
        if let Some(val) = require_privacy {
            args.push(MethodParameter { name: "RequirePrivacy".to_string(), value: val.into() });
        }
        if let Some(val) = full_access {
            args.push(MethodParameter { name: "FullAccess".to_string(), value: val.into() });
        }
        if let Some(val) = deny_access {
            args.push(MethodParameter { name: "DenyAccess".to_string(), value: val.into() });
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

        let result = self.invoke_method("Create", &args)?;
        let created_mapping = result.get_value("CreatedMapping")?;
        Ok(result.return_value)

    }


/// 

    /// * `force` -  (bool)

    /// * `return_value` -  (u32)
    pub fn remove(&self, force: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "Force".to_string(), value: force.into() });
        self.invoke_method("Remove", &args)

    }

}

