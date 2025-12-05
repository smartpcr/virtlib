// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.virtualization.v2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// CIM_VirtualSystemMigrationSettingData struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CIM_VirtualSystemMigrationSettingData {
    #[serde(flatten)]
    pub base: CIM_SettingData,

/// Bandwidth indicates the bandwidth assigned to or requested for a virtual system migration operation. The special value 0 indicates:
/// - in migration requests the default bandwidth
/// - otherwise that bandwidths are not supported.
/// Bandwidth and Priority may be used in conjunction. Migration processes that have the highest equal priority value share the available bandwidth based on their requested bandwidth. If not all bandwidth is consumed by this set of processes, migration processes with the next lower equal priority share the remaining bandwidth. If still more bandwidth remains, migration processes with the next lower equal priority are considered, and so forth.
/// The unit applicable for the Bandwidth property is conveyed by the value of the BandwidthUnit property. If the value of the BandwidthUnit property matches "percent", the following restrictions apply: 
/// The value of the Bandwidth property shall be between 0 and 100, with higher values indicating a higher bandwidth. A value of 100 indicates the total available bandwidth for performing virtual system migration operations. Values between 1 and 100 should linearly correlate with the available bandwidth range. For exampe, a value of 50 should request half of the available bandwidth, a value of 33 should request one third of the bandwidth, etc. .
    #[serde(rename = "Bandwidth")]
    pub bandwidth: Option<u16>,

/// This property specifies the unit used by the Bandwidth property. The value of this property shall be a legal value of the Programmatic Units qualifier as defined in Appendix C.1 of DSP0004 V2.4 or later.
/// NOTE: Profiles like DMTF DSP1081 define means by that clients are enabled to discover the set of units supported by an implementation, along with ranges and increments for admissable values of the Bandwidth property.
    #[serde(rename = "BandwidthUnit")]
    pub bandwidth_unit: Option<String>,

/// MigrationType describes a type of migration operation to be performed.
/// A value of 2 - Virtual System is to be migrated in a 'live' manner such that the running of the Virtual System is minimally impacted during the move.
/// A value of 3 - Virtual System will be temporarily paused prior to migration and then resume running after it is moved.
/// A value of 4 - The Virtual System will be quiesced to a stopped state prior to migration and then restarted after it is moved.
    #[serde(rename = "MigrationType")]
    pub migration_type: Option<VirtualSystemMigrationSettingData_MigrationType>,

/// OtherTransportType indicates the type of transport to be applied if the value of TransportType is 1 (Other).
    #[serde(rename = "OtherTransportType")]
    pub other_transport_type: Option<String>,

/// Priority specifies a relative migration importance which the virtual system migration implementation may use to order or otherwise give preference among multiple pending migration requests. The lower the value, the higher the priority. A value of 0 indicates:
/// - in migration requests the default priority
/// - otherwise that priorities are not supported
    #[serde(rename = "Priority")]
    pub priority: Option<u16>,

/// TransportType indicates the type of transport to be applied for a virtual system migration operation.
/// - 0(Unknown) indicates that the transport type is not exposed.- 1(Other) indicates that the transport type is specified as a textual value of the OtherTransportType property.
/// - 2(SSH) indicates the secure shell transport type.
/// - 3(TLS) indicats the transport layer security transport type.
/// - 4(TLS strict) indicats the transport layer security transport type with mutual authentication.
/// - 5(TCP) indicates the TCP transport type.
/// - 6(IPC) indicates the inter-process communication socket transport type. This transport type includes Unix domain sockets.
    #[serde(rename = "TransportType")]
    pub transport_type: Option<VirtualSystemMigrationSettingData_TransportType>,
}

impl CIM_VirtualSystemMigrationSettingData {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: CIM_SettingData::new(),
            bandwidth: None,
            bandwidth_unit: None,
            migration_type: None,
            other_transport_type: None,
            priority: None,
            transport_type: None,
        }
    }


    /// Sets the value of Bandwidth
    pub fn set_bandwidth(&mut self, value: u16) {
        self.bandwidth = Some(value);
    }

    /// Gets the value of Bandwidth
    pub fn get_bandwidth(&self) -> Option<&u16> {
        self.bandwidth.as_ref()
    }

    /// Sets the value of BandwidthUnit
    pub fn set_bandwidth_unit(&mut self, value: String) {
        self.bandwidth_unit = Some(value);
    }

    /// Gets the value of BandwidthUnit
    pub fn get_bandwidth_unit(&self) -> Option<&String> {
        self.bandwidth_unit.as_ref()
    }

    /// Sets the value of MigrationType
    pub fn set_migration_type(&mut self, value: VirtualSystemMigrationSettingData_MigrationType) {
        self.migration_type = Some(value);
    }

    /// Gets the value of MigrationType
    pub fn get_migration_type(&self) -> Option<&VirtualSystemMigrationSettingData_MigrationType> {
        self.migration_type.as_ref()
    }

    /// Sets the value of OtherTransportType
    pub fn set_other_transport_type(&mut self, value: String) {
        self.other_transport_type = Some(value);
    }

    /// Gets the value of OtherTransportType
    pub fn get_other_transport_type(&self) -> Option<&String> {
        self.other_transport_type.as_ref()
    }

    /// Sets the value of Priority
    pub fn set_priority(&mut self, value: u16) {
        self.priority = Some(value);
    }

    /// Gets the value of Priority
    pub fn get_priority(&self) -> Option<&u16> {
        self.priority.as_ref()
    }

    /// Sets the value of TransportType
    pub fn set_transport_type(&mut self, value: VirtualSystemMigrationSettingData_TransportType) {
        self.transport_type = Some(value);
    }

    /// Gets the value of TransportType
    pub fn get_transport_type(&self) -> Option<&VirtualSystemMigrationSettingData_TransportType> {
        self.transport_type.as_ref()
    }
}

