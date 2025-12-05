// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.DeliveryOptimization
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_DeliveryOptimizationFilePeerInfo struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_DeliveryOptimizationFilePeerInfo {

/// 
    #[serde(rename = "BytesReceived")]
    pub bytes_received: Vec<u64>,

/// 
    #[serde(rename = "BytesSent")]
    pub bytes_sent: Vec<u64>,

/// 
    #[serde(rename = "ConnectionEstablished")]
    pub connection_established: Vec<bool>,

/// 
    #[serde(rename = "DownloadRates")]
    pub download_rates: Vec<u32>,

/// 
    #[serde(rename = "FileId")]
    pub file_id: Option<String>,

/// 
    #[serde(rename = "IPs")]
    pub ips: Vec<String>,

/// 
    #[serde(rename = "PeerTypes")]
    pub peer_types: Vec<u8>,

/// 
    #[serde(rename = "UploadRates")]
    pub upload_rates: Vec<u32>,
}

impl MSFT_DeliveryOptimizationFilePeerInfo {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            bytes_received: Vec::new(),
            bytes_sent: Vec::new(),
            connection_established: Vec::new(),
            download_rates: Vec::new(),
            file_id: None,
            ips: Vec::new(),
            peer_types: Vec::new(),
            upload_rates: Vec::new(),
        }
    }


    /// Sets the value of BytesReceived
    pub fn set_bytes_received(&mut self, value: Vec<u64>) {
        self.bytes_received = value;
    }

    /// Gets the value of BytesReceived
    pub fn get_bytes_received(&self) -> &Vec<u64> {
        &self.bytes_received
    }

    /// Sets the value of BytesSent
    pub fn set_bytes_sent(&mut self, value: Vec<u64>) {
        self.bytes_sent = value;
    }

    /// Gets the value of BytesSent
    pub fn get_bytes_sent(&self) -> &Vec<u64> {
        &self.bytes_sent
    }

    /// Sets the value of ConnectionEstablished
    pub fn set_connection_established(&mut self, value: Vec<bool>) {
        self.connection_established = value;
    }

    /// Gets the value of ConnectionEstablished
    pub fn get_connection_established(&self) -> &Vec<bool> {
        &self.connection_established
    }

    /// Sets the value of DownloadRates
    pub fn set_download_rates(&mut self, value: Vec<u32>) {
        self.download_rates = value;
    }

    /// Gets the value of DownloadRates
    pub fn get_download_rates(&self) -> &Vec<u32> {
        &self.download_rates
    }

    /// Sets the value of FileId
    pub fn set_file_id(&mut self, value: String) {
        self.file_id = Some(value);
    }

    /// Gets the value of FileId
    pub fn get_file_id(&self) -> Option<&String> {
        self.file_id.as_ref()
    }

    /// Sets the value of IPs
    pub fn set_ips(&mut self, value: Vec<String>) {
        self.ips = value;
    }

    /// Gets the value of IPs
    pub fn get_ips(&self) -> &Vec<String> {
        &self.ips
    }

    /// Sets the value of PeerTypes
    pub fn set_peer_types(&mut self, value: Vec<u8>) {
        self.peer_types = value;
    }

    /// Gets the value of PeerTypes
    pub fn get_peer_types(&self) -> &Vec<u8> {
        &self.peer_types
    }

    /// Sets the value of UploadRates
    pub fn set_upload_rates(&mut self, value: Vec<u32>) {
        self.upload_rates = value;
    }

    /// Gets the value of UploadRates
    pub fn get_upload_rates(&self) -> &Vec<u32> {
        &self.upload_rates
    }
}

