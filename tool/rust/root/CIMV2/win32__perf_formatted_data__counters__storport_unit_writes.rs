// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfFormattedData_Counters_StorportUnitWrites struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfFormattedData_Counters_StorportUnitWrites {
    #[serde(flatten)]
    pub base: Win32_PerfFormattedData,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket014K")]
    pub successful_writes_persec_bucket014_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket028K")]
    pub successful_writes_persec_bucket028_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket0316K")]
    pub successful_writes_persec_bucket0316_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket0432K")]
    pub successful_writes_persec_bucket0432_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket0564K")]
    pub successful_writes_persec_bucket0564_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket06128K")]
    pub successful_writes_persec_bucket06128_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket07256K")]
    pub successful_writes_persec_bucket07256_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket081M")]
    pub successful_writes_persec_bucket081_m: Option<u64>,

/// 
    #[serde(rename = "SuccessfulWritesPersecBucket091M")]
    pub successful_writes_persec_bucket091_m: Option<u64>,

/// 
    #[serde(rename = "WriteBytesAverage")]
    pub write_bytes_average: Option<u64>,

/// 
    #[serde(rename = "WriteBytesPersec")]
    pub write_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "WriteLatency")]
    pub write_latency: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket01128us")]
    pub write_latency_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket02256us")]
    pub write_latency_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket03512us")]
    pub write_latency_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket041ms")]
    pub write_latency_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket054ms")]
    pub write_latency_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket0616ms")]
    pub write_latency_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket0764ms")]
    pub write_latency_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket08128ms")]
    pub write_latency_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket09256ms")]
    pub write_latency_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket10512ms")]
    pub write_latency_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket111s")]
    pub write_latency_bucket111s: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket122s")]
    pub write_latency_bucket122s: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket1310s")]
    pub write_latency_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "WriteLatencyBucket1410s")]
    pub write_latency_bucket1410s: Option<u64>,

/// 
    #[serde(rename = "WritesPersec")]
    pub writes_persec: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket01128us")]
    pub writes_persec_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket02256us")]
    pub writes_persec_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket03512us")]
    pub writes_persec_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket041ms")]
    pub writes_persec_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket054ms")]
    pub writes_persec_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket0616ms")]
    pub writes_persec_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket0764ms")]
    pub writes_persec_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket08128ms")]
    pub writes_persec_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket09256ms")]
    pub writes_persec_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket10512ms")]
    pub writes_persec_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket111s")]
    pub writes_persec_bucket111s: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket122s")]
    pub writes_persec_bucket122s: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket1310s")]
    pub writes_persec_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "WritesPersecBucket1410s")]
    pub writes_persec_bucket1410s: Option<u64>,
}

impl Win32_PerfFormattedData_Counters_StorportUnitWrites {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfFormattedData::new(),
            successful_writes_persec_bucket014_k: None,
            successful_writes_persec_bucket028_k: None,
            successful_writes_persec_bucket0316_k: None,
            successful_writes_persec_bucket0432_k: None,
            successful_writes_persec_bucket0564_k: None,
            successful_writes_persec_bucket06128_k: None,
            successful_writes_persec_bucket07256_k: None,
            successful_writes_persec_bucket081_m: None,
            successful_writes_persec_bucket091_m: None,
            write_bytes_average: None,
            write_bytes_persec: None,
            write_latency: None,
            write_latency_bucket01128us: None,
            write_latency_bucket02256us: None,
            write_latency_bucket03512us: None,
            write_latency_bucket041ms: None,
            write_latency_bucket054ms: None,
            write_latency_bucket0616ms: None,
            write_latency_bucket0764ms: None,
            write_latency_bucket08128ms: None,
            write_latency_bucket09256ms: None,
            write_latency_bucket10512ms: None,
            write_latency_bucket111s: None,
            write_latency_bucket122s: None,
            write_latency_bucket1310s: None,
            write_latency_bucket1410s: None,
            writes_persec: None,
            writes_persec_bucket01128us: None,
            writes_persec_bucket02256us: None,
            writes_persec_bucket03512us: None,
            writes_persec_bucket041ms: None,
            writes_persec_bucket054ms: None,
            writes_persec_bucket0616ms: None,
            writes_persec_bucket0764ms: None,
            writes_persec_bucket08128ms: None,
            writes_persec_bucket09256ms: None,
            writes_persec_bucket10512ms: None,
            writes_persec_bucket111s: None,
            writes_persec_bucket122s: None,
            writes_persec_bucket1310s: None,
            writes_persec_bucket1410s: None,
        }
    }


    /// Sets the value of SuccessfulWritesPersecBucket014K
    pub fn set_successful_writes_persec_bucket014_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket014_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket014K
    pub fn get_successful_writes_persec_bucket014_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket014_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket028K
    pub fn set_successful_writes_persec_bucket028_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket028_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket028K
    pub fn get_successful_writes_persec_bucket028_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket028_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket0316K
    pub fn set_successful_writes_persec_bucket0316_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket0316_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket0316K
    pub fn get_successful_writes_persec_bucket0316_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket0316_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket0432K
    pub fn set_successful_writes_persec_bucket0432_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket0432_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket0432K
    pub fn get_successful_writes_persec_bucket0432_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket0432_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket0564K
    pub fn set_successful_writes_persec_bucket0564_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket0564_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket0564K
    pub fn get_successful_writes_persec_bucket0564_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket0564_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket06128K
    pub fn set_successful_writes_persec_bucket06128_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket06128_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket06128K
    pub fn get_successful_writes_persec_bucket06128_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket06128_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket07256K
    pub fn set_successful_writes_persec_bucket07256_k(&mut self, value: u64) {
        self.successful_writes_persec_bucket07256_k = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket07256K
    pub fn get_successful_writes_persec_bucket07256_k(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket07256_k.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket081M
    pub fn set_successful_writes_persec_bucket081_m(&mut self, value: u64) {
        self.successful_writes_persec_bucket081_m = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket081M
    pub fn get_successful_writes_persec_bucket081_m(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket081_m.as_ref()
    }

    /// Sets the value of SuccessfulWritesPersecBucket091M
    pub fn set_successful_writes_persec_bucket091_m(&mut self, value: u64) {
        self.successful_writes_persec_bucket091_m = Some(value);
    }

    /// Gets the value of SuccessfulWritesPersecBucket091M
    pub fn get_successful_writes_persec_bucket091_m(&self) -> Option<&u64> {
        self.successful_writes_persec_bucket091_m.as_ref()
    }

    /// Sets the value of WriteBytesAverage
    pub fn set_write_bytes_average(&mut self, value: u64) {
        self.write_bytes_average = Some(value);
    }

    /// Gets the value of WriteBytesAverage
    pub fn get_write_bytes_average(&self) -> Option<&u64> {
        self.write_bytes_average.as_ref()
    }

    /// Sets the value of WriteBytesPersec
    pub fn set_write_bytes_persec(&mut self, value: u64) {
        self.write_bytes_persec = Some(value);
    }

    /// Gets the value of WriteBytesPersec
    pub fn get_write_bytes_persec(&self) -> Option<&u64> {
        self.write_bytes_persec.as_ref()
    }

    /// Sets the value of WriteLatency
    pub fn set_write_latency(&mut self, value: u64) {
        self.write_latency = Some(value);
    }

    /// Gets the value of WriteLatency
    pub fn get_write_latency(&self) -> Option<&u64> {
        self.write_latency.as_ref()
    }

    /// Sets the value of WriteLatencyBucket01128us
    pub fn set_write_latency_bucket01128us(&mut self, value: u64) {
        self.write_latency_bucket01128us = Some(value);
    }

    /// Gets the value of WriteLatencyBucket01128us
    pub fn get_write_latency_bucket01128us(&self) -> Option<&u64> {
        self.write_latency_bucket01128us.as_ref()
    }

    /// Sets the value of WriteLatencyBucket02256us
    pub fn set_write_latency_bucket02256us(&mut self, value: u64) {
        self.write_latency_bucket02256us = Some(value);
    }

    /// Gets the value of WriteLatencyBucket02256us
    pub fn get_write_latency_bucket02256us(&self) -> Option<&u64> {
        self.write_latency_bucket02256us.as_ref()
    }

    /// Sets the value of WriteLatencyBucket03512us
    pub fn set_write_latency_bucket03512us(&mut self, value: u64) {
        self.write_latency_bucket03512us = Some(value);
    }

    /// Gets the value of WriteLatencyBucket03512us
    pub fn get_write_latency_bucket03512us(&self) -> Option<&u64> {
        self.write_latency_bucket03512us.as_ref()
    }

    /// Sets the value of WriteLatencyBucket041ms
    pub fn set_write_latency_bucket041ms(&mut self, value: u64) {
        self.write_latency_bucket041ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket041ms
    pub fn get_write_latency_bucket041ms(&self) -> Option<&u64> {
        self.write_latency_bucket041ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket054ms
    pub fn set_write_latency_bucket054ms(&mut self, value: u64) {
        self.write_latency_bucket054ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket054ms
    pub fn get_write_latency_bucket054ms(&self) -> Option<&u64> {
        self.write_latency_bucket054ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket0616ms
    pub fn set_write_latency_bucket0616ms(&mut self, value: u64) {
        self.write_latency_bucket0616ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket0616ms
    pub fn get_write_latency_bucket0616ms(&self) -> Option<&u64> {
        self.write_latency_bucket0616ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket0764ms
    pub fn set_write_latency_bucket0764ms(&mut self, value: u64) {
        self.write_latency_bucket0764ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket0764ms
    pub fn get_write_latency_bucket0764ms(&self) -> Option<&u64> {
        self.write_latency_bucket0764ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket08128ms
    pub fn set_write_latency_bucket08128ms(&mut self, value: u64) {
        self.write_latency_bucket08128ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket08128ms
    pub fn get_write_latency_bucket08128ms(&self) -> Option<&u64> {
        self.write_latency_bucket08128ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket09256ms
    pub fn set_write_latency_bucket09256ms(&mut self, value: u64) {
        self.write_latency_bucket09256ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket09256ms
    pub fn get_write_latency_bucket09256ms(&self) -> Option<&u64> {
        self.write_latency_bucket09256ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket10512ms
    pub fn set_write_latency_bucket10512ms(&mut self, value: u64) {
        self.write_latency_bucket10512ms = Some(value);
    }

    /// Gets the value of WriteLatencyBucket10512ms
    pub fn get_write_latency_bucket10512ms(&self) -> Option<&u64> {
        self.write_latency_bucket10512ms.as_ref()
    }

    /// Sets the value of WriteLatencyBucket111s
    pub fn set_write_latency_bucket111s(&mut self, value: u64) {
        self.write_latency_bucket111s = Some(value);
    }

    /// Gets the value of WriteLatencyBucket111s
    pub fn get_write_latency_bucket111s(&self) -> Option<&u64> {
        self.write_latency_bucket111s.as_ref()
    }

    /// Sets the value of WriteLatencyBucket122s
    pub fn set_write_latency_bucket122s(&mut self, value: u64) {
        self.write_latency_bucket122s = Some(value);
    }

    /// Gets the value of WriteLatencyBucket122s
    pub fn get_write_latency_bucket122s(&self) -> Option<&u64> {
        self.write_latency_bucket122s.as_ref()
    }

    /// Sets the value of WriteLatencyBucket1310s
    pub fn set_write_latency_bucket1310s(&mut self, value: u64) {
        self.write_latency_bucket1310s = Some(value);
    }

    /// Gets the value of WriteLatencyBucket1310s
    pub fn get_write_latency_bucket1310s(&self) -> Option<&u64> {
        self.write_latency_bucket1310s.as_ref()
    }

    /// Sets the value of WriteLatencyBucket1410s
    pub fn set_write_latency_bucket1410s(&mut self, value: u64) {
        self.write_latency_bucket1410s = Some(value);
    }

    /// Gets the value of WriteLatencyBucket1410s
    pub fn get_write_latency_bucket1410s(&self) -> Option<&u64> {
        self.write_latency_bucket1410s.as_ref()
    }

    /// Sets the value of WritesPersec
    pub fn set_writes_persec(&mut self, value: u64) {
        self.writes_persec = Some(value);
    }

    /// Gets the value of WritesPersec
    pub fn get_writes_persec(&self) -> Option<&u64> {
        self.writes_persec.as_ref()
    }

    /// Sets the value of WritesPersecBucket01128us
    pub fn set_writes_persec_bucket01128us(&mut self, value: u64) {
        self.writes_persec_bucket01128us = Some(value);
    }

    /// Gets the value of WritesPersecBucket01128us
    pub fn get_writes_persec_bucket01128us(&self) -> Option<&u64> {
        self.writes_persec_bucket01128us.as_ref()
    }

    /// Sets the value of WritesPersecBucket02256us
    pub fn set_writes_persec_bucket02256us(&mut self, value: u64) {
        self.writes_persec_bucket02256us = Some(value);
    }

    /// Gets the value of WritesPersecBucket02256us
    pub fn get_writes_persec_bucket02256us(&self) -> Option<&u64> {
        self.writes_persec_bucket02256us.as_ref()
    }

    /// Sets the value of WritesPersecBucket03512us
    pub fn set_writes_persec_bucket03512us(&mut self, value: u64) {
        self.writes_persec_bucket03512us = Some(value);
    }

    /// Gets the value of WritesPersecBucket03512us
    pub fn get_writes_persec_bucket03512us(&self) -> Option<&u64> {
        self.writes_persec_bucket03512us.as_ref()
    }

    /// Sets the value of WritesPersecBucket041ms
    pub fn set_writes_persec_bucket041ms(&mut self, value: u64) {
        self.writes_persec_bucket041ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket041ms
    pub fn get_writes_persec_bucket041ms(&self) -> Option<&u64> {
        self.writes_persec_bucket041ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket054ms
    pub fn set_writes_persec_bucket054ms(&mut self, value: u64) {
        self.writes_persec_bucket054ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket054ms
    pub fn get_writes_persec_bucket054ms(&self) -> Option<&u64> {
        self.writes_persec_bucket054ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket0616ms
    pub fn set_writes_persec_bucket0616ms(&mut self, value: u64) {
        self.writes_persec_bucket0616ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket0616ms
    pub fn get_writes_persec_bucket0616ms(&self) -> Option<&u64> {
        self.writes_persec_bucket0616ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket0764ms
    pub fn set_writes_persec_bucket0764ms(&mut self, value: u64) {
        self.writes_persec_bucket0764ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket0764ms
    pub fn get_writes_persec_bucket0764ms(&self) -> Option<&u64> {
        self.writes_persec_bucket0764ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket08128ms
    pub fn set_writes_persec_bucket08128ms(&mut self, value: u64) {
        self.writes_persec_bucket08128ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket08128ms
    pub fn get_writes_persec_bucket08128ms(&self) -> Option<&u64> {
        self.writes_persec_bucket08128ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket09256ms
    pub fn set_writes_persec_bucket09256ms(&mut self, value: u64) {
        self.writes_persec_bucket09256ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket09256ms
    pub fn get_writes_persec_bucket09256ms(&self) -> Option<&u64> {
        self.writes_persec_bucket09256ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket10512ms
    pub fn set_writes_persec_bucket10512ms(&mut self, value: u64) {
        self.writes_persec_bucket10512ms = Some(value);
    }

    /// Gets the value of WritesPersecBucket10512ms
    pub fn get_writes_persec_bucket10512ms(&self) -> Option<&u64> {
        self.writes_persec_bucket10512ms.as_ref()
    }

    /// Sets the value of WritesPersecBucket111s
    pub fn set_writes_persec_bucket111s(&mut self, value: u64) {
        self.writes_persec_bucket111s = Some(value);
    }

    /// Gets the value of WritesPersecBucket111s
    pub fn get_writes_persec_bucket111s(&self) -> Option<&u64> {
        self.writes_persec_bucket111s.as_ref()
    }

    /// Sets the value of WritesPersecBucket122s
    pub fn set_writes_persec_bucket122s(&mut self, value: u64) {
        self.writes_persec_bucket122s = Some(value);
    }

    /// Gets the value of WritesPersecBucket122s
    pub fn get_writes_persec_bucket122s(&self) -> Option<&u64> {
        self.writes_persec_bucket122s.as_ref()
    }

    /// Sets the value of WritesPersecBucket1310s
    pub fn set_writes_persec_bucket1310s(&mut self, value: u64) {
        self.writes_persec_bucket1310s = Some(value);
    }

    /// Gets the value of WritesPersecBucket1310s
    pub fn get_writes_persec_bucket1310s(&self) -> Option<&u64> {
        self.writes_persec_bucket1310s.as_ref()
    }

    /// Sets the value of WritesPersecBucket1410s
    pub fn set_writes_persec_bucket1410s(&mut self, value: u64) {
        self.writes_persec_bucket1410s = Some(value);
    }

    /// Gets the value of WritesPersecBucket1410s
    pub fn get_writes_persec_bucket1410s(&self) -> Option<&u64> {
        self.writes_persec_bucket1410s.as_ref()
    }
}

