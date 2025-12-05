// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_NFS struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_NFS {
    #[serde(flatten)]
    pub base: CIM_RemoteFileSystem,

/// 
    #[serde(rename = "AttributeCaching")]
    pub attribute_caching: Option<bool>,

/// 
    #[serde(rename = "AttributeCachingForDirectoriesMax")]
    pub attribute_caching_for_directories_max: Option<u16>,

/// 
    #[serde(rename = "AttributeCachingForDirectoriesMin")]
    pub attribute_caching_for_directories_min: Option<u16>,

/// 
    #[serde(rename = "AttributeCachingForRegularFilesMax")]
    pub attribute_caching_for_regular_files_max: Option<u16>,

/// 
    #[serde(rename = "AttributeCachingForRegularFilesMin")]
    pub attribute_caching_for_regular_files_min: Option<u16>,

/// 
    #[serde(rename = "ForegroundMount")]
    pub foreground_mount: Option<bool>,

/// 
    #[serde(rename = "HardMount")]
    pub hard_mount: Option<bool>,

/// 
    #[serde(rename = "Interrupt")]
    pub interrupt: Option<bool>,

/// 
    #[serde(rename = "MountFailureRetries")]
    pub mount_failure_retries: Option<u16>,

/// 
    #[serde(rename = "ReadBufferSize")]
    pub read_buffer_size: Option<u64>,

/// 
    #[serde(rename = "RetransmissionAttempts")]
    pub retransmission_attempts: Option<u16>,

/// 
    #[serde(rename = "RetransmissionTimeout")]
    pub retransmission_timeout: Option<u32>,

/// 
    #[serde(rename = "ServerCommunicationPort")]
    pub server_communication_port: Option<u32>,

/// 
    #[serde(rename = "WriteBufferSize")]
    pub write_buffer_size: Option<u64>,
}

impl CIM_NFS {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_RemoteFileSystem::new(),
            attribute_caching: None,
            attribute_caching_for_directories_max: None,
            attribute_caching_for_directories_min: None,
            attribute_caching_for_regular_files_max: None,
            attribute_caching_for_regular_files_min: None,
            foreground_mount: None,
            hard_mount: None,
            interrupt: None,
            mount_failure_retries: None,
            read_buffer_size: None,
            retransmission_attempts: None,
            retransmission_timeout: None,
            server_communication_port: None,
            write_buffer_size: None,
        }
    }


    /// Sets the value of AttributeCaching
    pub fn set_attribute_caching(&mut self, value: bool) {
        self.attribute_caching = Some(value);
    }

    /// Gets the value of AttributeCaching
    pub fn get_attribute_caching(&self) -> Option<&bool> {
        self.attribute_caching.as_ref()
    }

    /// Sets the value of AttributeCachingForDirectoriesMax
    pub fn set_attribute_caching_for_directories_max(&mut self, value: u16) {
        self.attribute_caching_for_directories_max = Some(value);
    }

    /// Gets the value of AttributeCachingForDirectoriesMax
    pub fn get_attribute_caching_for_directories_max(&self) -> Option<&u16> {
        self.attribute_caching_for_directories_max.as_ref()
    }

    /// Sets the value of AttributeCachingForDirectoriesMin
    pub fn set_attribute_caching_for_directories_min(&mut self, value: u16) {
        self.attribute_caching_for_directories_min = Some(value);
    }

    /// Gets the value of AttributeCachingForDirectoriesMin
    pub fn get_attribute_caching_for_directories_min(&self) -> Option<&u16> {
        self.attribute_caching_for_directories_min.as_ref()
    }

    /// Sets the value of AttributeCachingForRegularFilesMax
    pub fn set_attribute_caching_for_regular_files_max(&mut self, value: u16) {
        self.attribute_caching_for_regular_files_max = Some(value);
    }

    /// Gets the value of AttributeCachingForRegularFilesMax
    pub fn get_attribute_caching_for_regular_files_max(&self) -> Option<&u16> {
        self.attribute_caching_for_regular_files_max.as_ref()
    }

    /// Sets the value of AttributeCachingForRegularFilesMin
    pub fn set_attribute_caching_for_regular_files_min(&mut self, value: u16) {
        self.attribute_caching_for_regular_files_min = Some(value);
    }

    /// Gets the value of AttributeCachingForRegularFilesMin
    pub fn get_attribute_caching_for_regular_files_min(&self) -> Option<&u16> {
        self.attribute_caching_for_regular_files_min.as_ref()
    }

    /// Sets the value of ForegroundMount
    pub fn set_foreground_mount(&mut self, value: bool) {
        self.foreground_mount = Some(value);
    }

    /// Gets the value of ForegroundMount
    pub fn get_foreground_mount(&self) -> Option<&bool> {
        self.foreground_mount.as_ref()
    }

    /// Sets the value of HardMount
    pub fn set_hard_mount(&mut self, value: bool) {
        self.hard_mount = Some(value);
    }

    /// Gets the value of HardMount
    pub fn get_hard_mount(&self) -> Option<&bool> {
        self.hard_mount.as_ref()
    }

    /// Sets the value of Interrupt
    pub fn set_interrupt(&mut self, value: bool) {
        self.interrupt = Some(value);
    }

    /// Gets the value of Interrupt
    pub fn get_interrupt(&self) -> Option<&bool> {
        self.interrupt.as_ref()
    }

    /// Sets the value of MountFailureRetries
    pub fn set_mount_failure_retries(&mut self, value: u16) {
        self.mount_failure_retries = Some(value);
    }

    /// Gets the value of MountFailureRetries
    pub fn get_mount_failure_retries(&self) -> Option<&u16> {
        self.mount_failure_retries.as_ref()
    }

    /// Sets the value of ReadBufferSize
    pub fn set_read_buffer_size(&mut self, value: u64) {
        self.read_buffer_size = Some(value);
    }

    /// Gets the value of ReadBufferSize
    pub fn get_read_buffer_size(&self) -> Option<&u64> {
        self.read_buffer_size.as_ref()
    }

    /// Sets the value of RetransmissionAttempts
    pub fn set_retransmission_attempts(&mut self, value: u16) {
        self.retransmission_attempts = Some(value);
    }

    /// Gets the value of RetransmissionAttempts
    pub fn get_retransmission_attempts(&self) -> Option<&u16> {
        self.retransmission_attempts.as_ref()
    }

    /// Sets the value of RetransmissionTimeout
    pub fn set_retransmission_timeout(&mut self, value: u32) {
        self.retransmission_timeout = Some(value);
    }

    /// Gets the value of RetransmissionTimeout
    pub fn get_retransmission_timeout(&self) -> Option<&u32> {
        self.retransmission_timeout.as_ref()
    }

    /// Sets the value of ServerCommunicationPort
    pub fn set_server_communication_port(&mut self, value: u32) {
        self.server_communication_port = Some(value);
    }

    /// Gets the value of ServerCommunicationPort
    pub fn get_server_communication_port(&self) -> Option<&u32> {
        self.server_communication_port.as_ref()
    }

    /// Sets the value of WriteBufferSize
    pub fn set_write_buffer_size(&mut self, value: u64) {
        self.write_buffer_size = Some(value);
    }

    /// Gets the value of WriteBufferSize
    pub fn get_write_buffer_size(&self) -> Option<&u64> {
        self.write_buffer_size.as_ref()
    }
}

