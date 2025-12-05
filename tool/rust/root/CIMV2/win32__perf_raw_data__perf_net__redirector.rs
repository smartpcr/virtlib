// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_PerfNet_Redirector struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_PerfNet_Redirector {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "BytesReceivedPersec")]
    pub bytes_received_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTotalPersec")]
    pub bytes_total_persec: Option<u64>,

/// 
    #[serde(rename = "BytesTransmittedPersec")]
    pub bytes_transmitted_persec: Option<u64>,

/// 
    #[serde(rename = "ConnectsCore")]
    pub connects_core: Option<u32>,

/// 
    #[serde(rename = "ConnectsLanManager20")]
    pub connects_lan_manager20: Option<u32>,

/// 
    #[serde(rename = "ConnectsLanManager21")]
    pub connects_lan_manager21: Option<u32>,

/// 
    #[serde(rename = "ConnectsWindowsNT")]
    pub connects_windows_nt: Option<u32>,

/// 
    #[serde(rename = "CurrentCommands")]
    pub current_commands: Option<u32>,

/// 
    #[serde(rename = "FileDataOperationsPersec")]
    pub file_data_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FileReadOperationsPersec")]
    pub file_read_operations_persec: Option<u32>,

/// 
    #[serde(rename = "FileWriteOperationsPersec")]
    pub file_write_operations_persec: Option<u32>,

/// 
    #[serde(rename = "NetworkErrorsPersec")]
    pub network_errors_persec: Option<u32>,

/// 
    #[serde(rename = "PacketsPersec")]
    pub packets_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsReceivedPersec")]
    pub packets_received_persec: Option<u64>,

/// 
    #[serde(rename = "PacketsTransmittedPersec")]
    pub packets_transmitted_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesCachePersec")]
    pub read_bytes_cache_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesNetworkPersec")]
    pub read_bytes_network_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesNonPagingPersec")]
    pub read_bytes_non_paging_persec: Option<u64>,

/// 
    #[serde(rename = "ReadBytesPagingPersec")]
    pub read_bytes_paging_persec: Option<u64>,

/// 
    #[serde(rename = "ReadOperationsRandomPersec")]
    pub read_operations_random_persec: Option<u32>,

/// 
    #[serde(rename = "ReadPacketsPersec")]
    pub read_packets_persec: Option<u32>,

/// 
    #[serde(rename = "ReadPacketsSmallPersec")]
    pub read_packets_small_persec: Option<u32>,

/// 
    #[serde(rename = "ReadsDeniedPersec")]
    pub reads_denied_persec: Option<u32>,

/// 
    #[serde(rename = "ReadsLargePersec")]
    pub reads_large_persec: Option<u32>,

/// 
    #[serde(rename = "ServerDisconnects")]
    pub server_disconnects: Option<u32>,

/// 
    #[serde(rename = "ServerReconnects")]
    pub server_reconnects: Option<u32>,

/// 
    #[serde(rename = "ServerSessions")]
    pub server_sessions: Option<u32>,

/// 
    #[serde(rename = "ServerSessionsHung")]
    pub server_sessions_hung: Option<u32>,

/// 
    #[serde(rename = "WriteBytesCachePersec")]
    pub write_bytes_cache_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesNetworkPersec")]
    pub write_bytes_network_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesNonPagingPersec")]
    pub write_bytes_non_paging_persec: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPagingPersec")]
    pub write_bytes_paging_persec: Option<u64>,

/// 
    #[serde(rename = "WriteOperationsRandomPersec")]
    pub write_operations_random_persec: Option<u32>,

/// 
    #[serde(rename = "WritePacketsPersec")]
    pub write_packets_persec: Option<u32>,

/// 
    #[serde(rename = "WritePacketsSmallPersec")]
    pub write_packets_small_persec: Option<u32>,

/// 
    #[serde(rename = "WritesDeniedPersec")]
    pub writes_denied_persec: Option<u32>,

/// 
    #[serde(rename = "WritesLargePersec")]
    pub writes_large_persec: Option<u32>,
}

impl Win32_PerfRawData_PerfNet_Redirector {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            bytes_received_persec: None,
            bytes_total_persec: None,
            bytes_transmitted_persec: None,
            connects_core: None,
            connects_lan_manager20: None,
            connects_lan_manager21: None,
            connects_windows_nt: None,
            current_commands: None,
            file_data_operations_persec: None,
            file_read_operations_persec: None,
            file_write_operations_persec: None,
            network_errors_persec: None,
            packets_persec: None,
            packets_received_persec: None,
            packets_transmitted_persec: None,
            read_bytes_cache_persec: None,
            read_bytes_network_persec: None,
            read_bytes_non_paging_persec: None,
            read_bytes_paging_persec: None,
            read_operations_random_persec: None,
            read_packets_persec: None,
            read_packets_small_persec: None,
            reads_denied_persec: None,
            reads_large_persec: None,
            server_disconnects: None,
            server_reconnects: None,
            server_sessions: None,
            server_sessions_hung: None,
            write_bytes_cache_persec: None,
            write_bytes_network_persec: None,
            write_bytes_non_paging_persec: None,
            write_bytes_paging_persec: None,
            write_operations_random_persec: None,
            write_packets_persec: None,
            write_packets_small_persec: None,
            writes_denied_persec: None,
            writes_large_persec: None,
        }
    }


    /// Sets the value of BytesReceivedPersec
    pub fn set_bytes_received_persec(&mut self, value: u64) {
        self.bytes_received_persec = Some(value);
    }

    /// Gets the value of BytesReceivedPersec
    pub fn get_bytes_received_persec(&self) -> Option<&u64> {
        self.bytes_received_persec.as_ref()
    }

    /// Sets the value of BytesTotalPersec
    pub fn set_bytes_total_persec(&mut self, value: u64) {
        self.bytes_total_persec = Some(value);
    }

    /// Gets the value of BytesTotalPersec
    pub fn get_bytes_total_persec(&self) -> Option<&u64> {
        self.bytes_total_persec.as_ref()
    }

    /// Sets the value of BytesTransmittedPersec
    pub fn set_bytes_transmitted_persec(&mut self, value: u64) {
        self.bytes_transmitted_persec = Some(value);
    }

    /// Gets the value of BytesTransmittedPersec
    pub fn get_bytes_transmitted_persec(&self) -> Option<&u64> {
        self.bytes_transmitted_persec.as_ref()
    }

    /// Sets the value of ConnectsCore
    pub fn set_connects_core(&mut self, value: u32) {
        self.connects_core = Some(value);
    }

    /// Gets the value of ConnectsCore
    pub fn get_connects_core(&self) -> Option<&u32> {
        self.connects_core.as_ref()
    }

    /// Sets the value of ConnectsLanManager20
    pub fn set_connects_lan_manager20(&mut self, value: u32) {
        self.connects_lan_manager20 = Some(value);
    }

    /// Gets the value of ConnectsLanManager20
    pub fn get_connects_lan_manager20(&self) -> Option<&u32> {
        self.connects_lan_manager20.as_ref()
    }

    /// Sets the value of ConnectsLanManager21
    pub fn set_connects_lan_manager21(&mut self, value: u32) {
        self.connects_lan_manager21 = Some(value);
    }

    /// Gets the value of ConnectsLanManager21
    pub fn get_connects_lan_manager21(&self) -> Option<&u32> {
        self.connects_lan_manager21.as_ref()
    }

    /// Sets the value of ConnectsWindowsNT
    pub fn set_connects_windows_nt(&mut self, value: u32) {
        self.connects_windows_nt = Some(value);
    }

    /// Gets the value of ConnectsWindowsNT
    pub fn get_connects_windows_nt(&self) -> Option<&u32> {
        self.connects_windows_nt.as_ref()
    }

    /// Sets the value of CurrentCommands
    pub fn set_current_commands(&mut self, value: u32) {
        self.current_commands = Some(value);
    }

    /// Gets the value of CurrentCommands
    pub fn get_current_commands(&self) -> Option<&u32> {
        self.current_commands.as_ref()
    }

    /// Sets the value of FileDataOperationsPersec
    pub fn set_file_data_operations_persec(&mut self, value: u32) {
        self.file_data_operations_persec = Some(value);
    }

    /// Gets the value of FileDataOperationsPersec
    pub fn get_file_data_operations_persec(&self) -> Option<&u32> {
        self.file_data_operations_persec.as_ref()
    }

    /// Sets the value of FileReadOperationsPersec
    pub fn set_file_read_operations_persec(&mut self, value: u32) {
        self.file_read_operations_persec = Some(value);
    }

    /// Gets the value of FileReadOperationsPersec
    pub fn get_file_read_operations_persec(&self) -> Option<&u32> {
        self.file_read_operations_persec.as_ref()
    }

    /// Sets the value of FileWriteOperationsPersec
    pub fn set_file_write_operations_persec(&mut self, value: u32) {
        self.file_write_operations_persec = Some(value);
    }

    /// Gets the value of FileWriteOperationsPersec
    pub fn get_file_write_operations_persec(&self) -> Option<&u32> {
        self.file_write_operations_persec.as_ref()
    }

    /// Sets the value of NetworkErrorsPersec
    pub fn set_network_errors_persec(&mut self, value: u32) {
        self.network_errors_persec = Some(value);
    }

    /// Gets the value of NetworkErrorsPersec
    pub fn get_network_errors_persec(&self) -> Option<&u32> {
        self.network_errors_persec.as_ref()
    }

    /// Sets the value of PacketsPersec
    pub fn set_packets_persec(&mut self, value: u64) {
        self.packets_persec = Some(value);
    }

    /// Gets the value of PacketsPersec
    pub fn get_packets_persec(&self) -> Option<&u64> {
        self.packets_persec.as_ref()
    }

    /// Sets the value of PacketsReceivedPersec
    pub fn set_packets_received_persec(&mut self, value: u64) {
        self.packets_received_persec = Some(value);
    }

    /// Gets the value of PacketsReceivedPersec
    pub fn get_packets_received_persec(&self) -> Option<&u64> {
        self.packets_received_persec.as_ref()
    }

    /// Sets the value of PacketsTransmittedPersec
    pub fn set_packets_transmitted_persec(&mut self, value: u64) {
        self.packets_transmitted_persec = Some(value);
    }

    /// Gets the value of PacketsTransmittedPersec
    pub fn get_packets_transmitted_persec(&self) -> Option<&u64> {
        self.packets_transmitted_persec.as_ref()
    }

    /// Sets the value of ReadBytesCachePersec
    pub fn set_read_bytes_cache_persec(&mut self, value: u64) {
        self.read_bytes_cache_persec = Some(value);
    }

    /// Gets the value of ReadBytesCachePersec
    pub fn get_read_bytes_cache_persec(&self) -> Option<&u64> {
        self.read_bytes_cache_persec.as_ref()
    }

    /// Sets the value of ReadBytesNetworkPersec
    pub fn set_read_bytes_network_persec(&mut self, value: u64) {
        self.read_bytes_network_persec = Some(value);
    }

    /// Gets the value of ReadBytesNetworkPersec
    pub fn get_read_bytes_network_persec(&self) -> Option<&u64> {
        self.read_bytes_network_persec.as_ref()
    }

    /// Sets the value of ReadBytesNonPagingPersec
    pub fn set_read_bytes_non_paging_persec(&mut self, value: u64) {
        self.read_bytes_non_paging_persec = Some(value);
    }

    /// Gets the value of ReadBytesNonPagingPersec
    pub fn get_read_bytes_non_paging_persec(&self) -> Option<&u64> {
        self.read_bytes_non_paging_persec.as_ref()
    }

    /// Sets the value of ReadBytesPagingPersec
    pub fn set_read_bytes_paging_persec(&mut self, value: u64) {
        self.read_bytes_paging_persec = Some(value);
    }

    /// Gets the value of ReadBytesPagingPersec
    pub fn get_read_bytes_paging_persec(&self) -> Option<&u64> {
        self.read_bytes_paging_persec.as_ref()
    }

    /// Sets the value of ReadOperationsRandomPersec
    pub fn set_read_operations_random_persec(&mut self, value: u32) {
        self.read_operations_random_persec = Some(value);
    }

    /// Gets the value of ReadOperationsRandomPersec
    pub fn get_read_operations_random_persec(&self) -> Option<&u32> {
        self.read_operations_random_persec.as_ref()
    }

    /// Sets the value of ReadPacketsPersec
    pub fn set_read_packets_persec(&mut self, value: u32) {
        self.read_packets_persec = Some(value);
    }

    /// Gets the value of ReadPacketsPersec
    pub fn get_read_packets_persec(&self) -> Option<&u32> {
        self.read_packets_persec.as_ref()
    }

    /// Sets the value of ReadPacketsSmallPersec
    pub fn set_read_packets_small_persec(&mut self, value: u32) {
        self.read_packets_small_persec = Some(value);
    }

    /// Gets the value of ReadPacketsSmallPersec
    pub fn get_read_packets_small_persec(&self) -> Option<&u32> {
        self.read_packets_small_persec.as_ref()
    }

    /// Sets the value of ReadsDeniedPersec
    pub fn set_reads_denied_persec(&mut self, value: u32) {
        self.reads_denied_persec = Some(value);
    }

    /// Gets the value of ReadsDeniedPersec
    pub fn get_reads_denied_persec(&self) -> Option<&u32> {
        self.reads_denied_persec.as_ref()
    }

    /// Sets the value of ReadsLargePersec
    pub fn set_reads_large_persec(&mut self, value: u32) {
        self.reads_large_persec = Some(value);
    }

    /// Gets the value of ReadsLargePersec
    pub fn get_reads_large_persec(&self) -> Option<&u32> {
        self.reads_large_persec.as_ref()
    }

    /// Sets the value of ServerDisconnects
    pub fn set_server_disconnects(&mut self, value: u32) {
        self.server_disconnects = Some(value);
    }

    /// Gets the value of ServerDisconnects
    pub fn get_server_disconnects(&self) -> Option<&u32> {
        self.server_disconnects.as_ref()
    }

    /// Sets the value of ServerReconnects
    pub fn set_server_reconnects(&mut self, value: u32) {
        self.server_reconnects = Some(value);
    }

    /// Gets the value of ServerReconnects
    pub fn get_server_reconnects(&self) -> Option<&u32> {
        self.server_reconnects.as_ref()
    }

    /// Sets the value of ServerSessions
    pub fn set_server_sessions(&mut self, value: u32) {
        self.server_sessions = Some(value);
    }

    /// Gets the value of ServerSessions
    pub fn get_server_sessions(&self) -> Option<&u32> {
        self.server_sessions.as_ref()
    }

    /// Sets the value of ServerSessionsHung
    pub fn set_server_sessions_hung(&mut self, value: u32) {
        self.server_sessions_hung = Some(value);
    }

    /// Gets the value of ServerSessionsHung
    pub fn get_server_sessions_hung(&self) -> Option<&u32> {
        self.server_sessions_hung.as_ref()
    }

    /// Sets the value of WriteBytesCachePersec
    pub fn set_write_bytes_cache_persec(&mut self, value: u64) {
        self.write_bytes_cache_persec = Some(value);
    }

    /// Gets the value of WriteBytesCachePersec
    pub fn get_write_bytes_cache_persec(&self) -> Option<&u64> {
        self.write_bytes_cache_persec.as_ref()
    }

    /// Sets the value of WriteBytesNetworkPersec
    pub fn set_write_bytes_network_persec(&mut self, value: u64) {
        self.write_bytes_network_persec = Some(value);
    }

    /// Gets the value of WriteBytesNetworkPersec
    pub fn get_write_bytes_network_persec(&self) -> Option<&u64> {
        self.write_bytes_network_persec.as_ref()
    }

    /// Sets the value of WriteBytesNonPagingPersec
    pub fn set_write_bytes_non_paging_persec(&mut self, value: u64) {
        self.write_bytes_non_paging_persec = Some(value);
    }

    /// Gets the value of WriteBytesNonPagingPersec
    pub fn get_write_bytes_non_paging_persec(&self) -> Option<&u64> {
        self.write_bytes_non_paging_persec.as_ref()
    }

    /// Sets the value of WriteBytesPagingPersec
    pub fn set_write_bytes_paging_persec(&mut self, value: u64) {
        self.write_bytes_paging_persec = Some(value);
    }

    /// Gets the value of WriteBytesPagingPersec
    pub fn get_write_bytes_paging_persec(&self) -> Option<&u64> {
        self.write_bytes_paging_persec.as_ref()
    }

    /// Sets the value of WriteOperationsRandomPersec
    pub fn set_write_operations_random_persec(&mut self, value: u32) {
        self.write_operations_random_persec = Some(value);
    }

    /// Gets the value of WriteOperationsRandomPersec
    pub fn get_write_operations_random_persec(&self) -> Option<&u32> {
        self.write_operations_random_persec.as_ref()
    }

    /// Sets the value of WritePacketsPersec
    pub fn set_write_packets_persec(&mut self, value: u32) {
        self.write_packets_persec = Some(value);
    }

    /// Gets the value of WritePacketsPersec
    pub fn get_write_packets_persec(&self) -> Option<&u32> {
        self.write_packets_persec.as_ref()
    }

    /// Sets the value of WritePacketsSmallPersec
    pub fn set_write_packets_small_persec(&mut self, value: u32) {
        self.write_packets_small_persec = Some(value);
    }

    /// Gets the value of WritePacketsSmallPersec
    pub fn get_write_packets_small_persec(&self) -> Option<&u32> {
        self.write_packets_small_persec.as_ref()
    }

    /// Sets the value of WritesDeniedPersec
    pub fn set_writes_denied_persec(&mut self, value: u32) {
        self.writes_denied_persec = Some(value);
    }

    /// Gets the value of WritesDeniedPersec
    pub fn get_writes_denied_persec(&self) -> Option<&u32> {
        self.writes_denied_persec.as_ref()
    }

    /// Sets the value of WritesLargePersec
    pub fn set_writes_large_persec(&mut self, value: u32) {
        self.writes_large_persec = Some(value);
    }

    /// Gets the value of WritesLargePersec
    pub fn get_writes_large_persec(&self) -> Option<&u32> {
        self.writes_large_persec.as_ref()
    }
}

