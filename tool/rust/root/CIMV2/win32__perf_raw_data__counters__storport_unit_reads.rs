// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorportUnitReads struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorportUnitReads {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "ReadBytesAverage")]
    pub read_bytes_average: Option<u64>,

/// 
    #[serde(rename = "ReadBytesAverage_Base")]
    pub read_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "ReadBytesPersec")]
    pub read_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "ReadLatency")]
    pub read_latency: Option<u64>,

/// 
    #[serde(rename = "ReadLatency_Base")]
    pub read_latency__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket01128us")]
    pub read_latency_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket01128us_Base")]
    pub read_latency_bucket01128us__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket02256us")]
    pub read_latency_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket02256us_Base")]
    pub read_latency_bucket02256us__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket03512us")]
    pub read_latency_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket03512us_Base")]
    pub read_latency_bucket03512us__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket041ms")]
    pub read_latency_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket041ms_Base")]
    pub read_latency_bucket041ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket054ms")]
    pub read_latency_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket054ms_Base")]
    pub read_latency_bucket054ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket0616ms")]
    pub read_latency_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket0616ms_Base")]
    pub read_latency_bucket0616ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket0764ms")]
    pub read_latency_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket0764ms_Base")]
    pub read_latency_bucket0764ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket08128ms")]
    pub read_latency_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket08128ms_Base")]
    pub read_latency_bucket08128ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket09256ms")]
    pub read_latency_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket09256ms_Base")]
    pub read_latency_bucket09256ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket10512ms")]
    pub read_latency_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket10512ms_Base")]
    pub read_latency_bucket10512ms__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket111s")]
    pub read_latency_bucket111s: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket111s_Base")]
    pub read_latency_bucket111s__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket122s")]
    pub read_latency_bucket122s: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket122s_Base")]
    pub read_latency_bucket122s__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket1310s")]
    pub read_latency_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket1310s_Base")]
    pub read_latency_bucket1310s__base: Option<u32>,

/// 
    #[serde(rename = "ReadLatencyBucket1410s")]
    pub read_latency_bucket1410s: Option<u64>,

/// 
    #[serde(rename = "ReadLatencyBucket1410s_Base")]
    pub read_latency_bucket1410s__base: Option<u32>,

/// 
    #[serde(rename = "ReadsPersec")]
    pub reads_persec: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket01128us")]
    pub reads_persec_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket02256us")]
    pub reads_persec_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket03512us")]
    pub reads_persec_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket041ms")]
    pub reads_persec_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket054ms")]
    pub reads_persec_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket0616ms")]
    pub reads_persec_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket0764ms")]
    pub reads_persec_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket08128ms")]
    pub reads_persec_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket09256ms")]
    pub reads_persec_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket10512ms")]
    pub reads_persec_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket111s")]
    pub reads_persec_bucket111s: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket122s")]
    pub reads_persec_bucket122s: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket1310s")]
    pub reads_persec_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "ReadsPersecBucket1410s")]
    pub reads_persec_bucket1410s: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket014K")]
    pub successful_reads_persec_bucket014_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket028K")]
    pub successful_reads_persec_bucket028_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket0316K")]
    pub successful_reads_persec_bucket0316_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket0432K")]
    pub successful_reads_persec_bucket0432_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket0564K")]
    pub successful_reads_persec_bucket0564_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket06128K")]
    pub successful_reads_persec_bucket06128_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket07256K")]
    pub successful_reads_persec_bucket07256_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket081M")]
    pub successful_reads_persec_bucket081_m: Option<u64>,

/// 
    #[serde(rename = "SuccessfulReadsPersecBucket091M")]
    pub successful_reads_persec_bucket091_m: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorportUnitReads {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            read_bytes_average: None,
            read_bytes_average__base: None,
            read_bytes_persec: None,
            read_latency: None,
            read_latency__base: None,
            read_latency_bucket01128us: None,
            read_latency_bucket01128us__base: None,
            read_latency_bucket02256us: None,
            read_latency_bucket02256us__base: None,
            read_latency_bucket03512us: None,
            read_latency_bucket03512us__base: None,
            read_latency_bucket041ms: None,
            read_latency_bucket041ms__base: None,
            read_latency_bucket054ms: None,
            read_latency_bucket054ms__base: None,
            read_latency_bucket0616ms: None,
            read_latency_bucket0616ms__base: None,
            read_latency_bucket0764ms: None,
            read_latency_bucket0764ms__base: None,
            read_latency_bucket08128ms: None,
            read_latency_bucket08128ms__base: None,
            read_latency_bucket09256ms: None,
            read_latency_bucket09256ms__base: None,
            read_latency_bucket10512ms: None,
            read_latency_bucket10512ms__base: None,
            read_latency_bucket111s: None,
            read_latency_bucket111s__base: None,
            read_latency_bucket122s: None,
            read_latency_bucket122s__base: None,
            read_latency_bucket1310s: None,
            read_latency_bucket1310s__base: None,
            read_latency_bucket1410s: None,
            read_latency_bucket1410s__base: None,
            reads_persec: None,
            reads_persec_bucket01128us: None,
            reads_persec_bucket02256us: None,
            reads_persec_bucket03512us: None,
            reads_persec_bucket041ms: None,
            reads_persec_bucket054ms: None,
            reads_persec_bucket0616ms: None,
            reads_persec_bucket0764ms: None,
            reads_persec_bucket08128ms: None,
            reads_persec_bucket09256ms: None,
            reads_persec_bucket10512ms: None,
            reads_persec_bucket111s: None,
            reads_persec_bucket122s: None,
            reads_persec_bucket1310s: None,
            reads_persec_bucket1410s: None,
            successful_reads_persec_bucket014_k: None,
            successful_reads_persec_bucket028_k: None,
            successful_reads_persec_bucket0316_k: None,
            successful_reads_persec_bucket0432_k: None,
            successful_reads_persec_bucket0564_k: None,
            successful_reads_persec_bucket06128_k: None,
            successful_reads_persec_bucket07256_k: None,
            successful_reads_persec_bucket081_m: None,
            successful_reads_persec_bucket091_m: None,
        }
    }


    /// Sets the value of ReadBytesAverage
    pub fn set_read_bytes_average(&mut self, value: u64) {
        self.read_bytes_average = Some(value);
    }

    /// Gets the value of ReadBytesAverage
    pub fn get_read_bytes_average(&self) -> Option<&u64> {
        self.read_bytes_average.as_ref()
    }

    /// Sets the value of ReadBytesAverage_Base
    pub fn set_read_bytes_average__base(&mut self, value: u32) {
        self.read_bytes_average__base = Some(value);
    }

    /// Gets the value of ReadBytesAverage_Base
    pub fn get_read_bytes_average__base(&self) -> Option<&u32> {
        self.read_bytes_average__base.as_ref()
    }

    /// Sets the value of ReadBytesPersec
    pub fn set_read_bytes_persec(&mut self, value: u64) {
        self.read_bytes_persec = Some(value);
    }

    /// Gets the value of ReadBytesPersec
    pub fn get_read_bytes_persec(&self) -> Option<&u64> {
        self.read_bytes_persec.as_ref()
    }

    /// Sets the value of ReadLatency
    pub fn set_read_latency(&mut self, value: u64) {
        self.read_latency = Some(value);
    }

    /// Gets the value of ReadLatency
    pub fn get_read_latency(&self) -> Option<&u64> {
        self.read_latency.as_ref()
    }

    /// Sets the value of ReadLatency_Base
    pub fn set_read_latency__base(&mut self, value: u32) {
        self.read_latency__base = Some(value);
    }

    /// Gets the value of ReadLatency_Base
    pub fn get_read_latency__base(&self) -> Option<&u32> {
        self.read_latency__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket01128us
    pub fn set_read_latency_bucket01128us(&mut self, value: u64) {
        self.read_latency_bucket01128us = Some(value);
    }

    /// Gets the value of ReadLatencyBucket01128us
    pub fn get_read_latency_bucket01128us(&self) -> Option<&u64> {
        self.read_latency_bucket01128us.as_ref()
    }

    /// Sets the value of ReadLatencyBucket01128us_Base
    pub fn set_read_latency_bucket01128us__base(&mut self, value: u32) {
        self.read_latency_bucket01128us__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket01128us_Base
    pub fn get_read_latency_bucket01128us__base(&self) -> Option<&u32> {
        self.read_latency_bucket01128us__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket02256us
    pub fn set_read_latency_bucket02256us(&mut self, value: u64) {
        self.read_latency_bucket02256us = Some(value);
    }

    /// Gets the value of ReadLatencyBucket02256us
    pub fn get_read_latency_bucket02256us(&self) -> Option<&u64> {
        self.read_latency_bucket02256us.as_ref()
    }

    /// Sets the value of ReadLatencyBucket02256us_Base
    pub fn set_read_latency_bucket02256us__base(&mut self, value: u32) {
        self.read_latency_bucket02256us__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket02256us_Base
    pub fn get_read_latency_bucket02256us__base(&self) -> Option<&u32> {
        self.read_latency_bucket02256us__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket03512us
    pub fn set_read_latency_bucket03512us(&mut self, value: u64) {
        self.read_latency_bucket03512us = Some(value);
    }

    /// Gets the value of ReadLatencyBucket03512us
    pub fn get_read_latency_bucket03512us(&self) -> Option<&u64> {
        self.read_latency_bucket03512us.as_ref()
    }

    /// Sets the value of ReadLatencyBucket03512us_Base
    pub fn set_read_latency_bucket03512us__base(&mut self, value: u32) {
        self.read_latency_bucket03512us__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket03512us_Base
    pub fn get_read_latency_bucket03512us__base(&self) -> Option<&u32> {
        self.read_latency_bucket03512us__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket041ms
    pub fn set_read_latency_bucket041ms(&mut self, value: u64) {
        self.read_latency_bucket041ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket041ms
    pub fn get_read_latency_bucket041ms(&self) -> Option<&u64> {
        self.read_latency_bucket041ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket041ms_Base
    pub fn set_read_latency_bucket041ms__base(&mut self, value: u32) {
        self.read_latency_bucket041ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket041ms_Base
    pub fn get_read_latency_bucket041ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket041ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket054ms
    pub fn set_read_latency_bucket054ms(&mut self, value: u64) {
        self.read_latency_bucket054ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket054ms
    pub fn get_read_latency_bucket054ms(&self) -> Option<&u64> {
        self.read_latency_bucket054ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket054ms_Base
    pub fn set_read_latency_bucket054ms__base(&mut self, value: u32) {
        self.read_latency_bucket054ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket054ms_Base
    pub fn get_read_latency_bucket054ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket054ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket0616ms
    pub fn set_read_latency_bucket0616ms(&mut self, value: u64) {
        self.read_latency_bucket0616ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket0616ms
    pub fn get_read_latency_bucket0616ms(&self) -> Option<&u64> {
        self.read_latency_bucket0616ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket0616ms_Base
    pub fn set_read_latency_bucket0616ms__base(&mut self, value: u32) {
        self.read_latency_bucket0616ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket0616ms_Base
    pub fn get_read_latency_bucket0616ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket0616ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket0764ms
    pub fn set_read_latency_bucket0764ms(&mut self, value: u64) {
        self.read_latency_bucket0764ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket0764ms
    pub fn get_read_latency_bucket0764ms(&self) -> Option<&u64> {
        self.read_latency_bucket0764ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket0764ms_Base
    pub fn set_read_latency_bucket0764ms__base(&mut self, value: u32) {
        self.read_latency_bucket0764ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket0764ms_Base
    pub fn get_read_latency_bucket0764ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket0764ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket08128ms
    pub fn set_read_latency_bucket08128ms(&mut self, value: u64) {
        self.read_latency_bucket08128ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket08128ms
    pub fn get_read_latency_bucket08128ms(&self) -> Option<&u64> {
        self.read_latency_bucket08128ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket08128ms_Base
    pub fn set_read_latency_bucket08128ms__base(&mut self, value: u32) {
        self.read_latency_bucket08128ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket08128ms_Base
    pub fn get_read_latency_bucket08128ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket08128ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket09256ms
    pub fn set_read_latency_bucket09256ms(&mut self, value: u64) {
        self.read_latency_bucket09256ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket09256ms
    pub fn get_read_latency_bucket09256ms(&self) -> Option<&u64> {
        self.read_latency_bucket09256ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket09256ms_Base
    pub fn set_read_latency_bucket09256ms__base(&mut self, value: u32) {
        self.read_latency_bucket09256ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket09256ms_Base
    pub fn get_read_latency_bucket09256ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket09256ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket10512ms
    pub fn set_read_latency_bucket10512ms(&mut self, value: u64) {
        self.read_latency_bucket10512ms = Some(value);
    }

    /// Gets the value of ReadLatencyBucket10512ms
    pub fn get_read_latency_bucket10512ms(&self) -> Option<&u64> {
        self.read_latency_bucket10512ms.as_ref()
    }

    /// Sets the value of ReadLatencyBucket10512ms_Base
    pub fn set_read_latency_bucket10512ms__base(&mut self, value: u32) {
        self.read_latency_bucket10512ms__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket10512ms_Base
    pub fn get_read_latency_bucket10512ms__base(&self) -> Option<&u32> {
        self.read_latency_bucket10512ms__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket111s
    pub fn set_read_latency_bucket111s(&mut self, value: u64) {
        self.read_latency_bucket111s = Some(value);
    }

    /// Gets the value of ReadLatencyBucket111s
    pub fn get_read_latency_bucket111s(&self) -> Option<&u64> {
        self.read_latency_bucket111s.as_ref()
    }

    /// Sets the value of ReadLatencyBucket111s_Base
    pub fn set_read_latency_bucket111s__base(&mut self, value: u32) {
        self.read_latency_bucket111s__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket111s_Base
    pub fn get_read_latency_bucket111s__base(&self) -> Option<&u32> {
        self.read_latency_bucket111s__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket122s
    pub fn set_read_latency_bucket122s(&mut self, value: u64) {
        self.read_latency_bucket122s = Some(value);
    }

    /// Gets the value of ReadLatencyBucket122s
    pub fn get_read_latency_bucket122s(&self) -> Option<&u64> {
        self.read_latency_bucket122s.as_ref()
    }

    /// Sets the value of ReadLatencyBucket122s_Base
    pub fn set_read_latency_bucket122s__base(&mut self, value: u32) {
        self.read_latency_bucket122s__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket122s_Base
    pub fn get_read_latency_bucket122s__base(&self) -> Option<&u32> {
        self.read_latency_bucket122s__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket1310s
    pub fn set_read_latency_bucket1310s(&mut self, value: u64) {
        self.read_latency_bucket1310s = Some(value);
    }

    /// Gets the value of ReadLatencyBucket1310s
    pub fn get_read_latency_bucket1310s(&self) -> Option<&u64> {
        self.read_latency_bucket1310s.as_ref()
    }

    /// Sets the value of ReadLatencyBucket1310s_Base
    pub fn set_read_latency_bucket1310s__base(&mut self, value: u32) {
        self.read_latency_bucket1310s__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket1310s_Base
    pub fn get_read_latency_bucket1310s__base(&self) -> Option<&u32> {
        self.read_latency_bucket1310s__base.as_ref()
    }

    /// Sets the value of ReadLatencyBucket1410s
    pub fn set_read_latency_bucket1410s(&mut self, value: u64) {
        self.read_latency_bucket1410s = Some(value);
    }

    /// Gets the value of ReadLatencyBucket1410s
    pub fn get_read_latency_bucket1410s(&self) -> Option<&u64> {
        self.read_latency_bucket1410s.as_ref()
    }

    /// Sets the value of ReadLatencyBucket1410s_Base
    pub fn set_read_latency_bucket1410s__base(&mut self, value: u32) {
        self.read_latency_bucket1410s__base = Some(value);
    }

    /// Gets the value of ReadLatencyBucket1410s_Base
    pub fn get_read_latency_bucket1410s__base(&self) -> Option<&u32> {
        self.read_latency_bucket1410s__base.as_ref()
    }

    /// Sets the value of ReadsPersec
    pub fn set_reads_persec(&mut self, value: u64) {
        self.reads_persec = Some(value);
    }

    /// Gets the value of ReadsPersec
    pub fn get_reads_persec(&self) -> Option<&u64> {
        self.reads_persec.as_ref()
    }

    /// Sets the value of ReadsPersecBucket01128us
    pub fn set_reads_persec_bucket01128us(&mut self, value: u64) {
        self.reads_persec_bucket01128us = Some(value);
    }

    /// Gets the value of ReadsPersecBucket01128us
    pub fn get_reads_persec_bucket01128us(&self) -> Option<&u64> {
        self.reads_persec_bucket01128us.as_ref()
    }

    /// Sets the value of ReadsPersecBucket02256us
    pub fn set_reads_persec_bucket02256us(&mut self, value: u64) {
        self.reads_persec_bucket02256us = Some(value);
    }

    /// Gets the value of ReadsPersecBucket02256us
    pub fn get_reads_persec_bucket02256us(&self) -> Option<&u64> {
        self.reads_persec_bucket02256us.as_ref()
    }

    /// Sets the value of ReadsPersecBucket03512us
    pub fn set_reads_persec_bucket03512us(&mut self, value: u64) {
        self.reads_persec_bucket03512us = Some(value);
    }

    /// Gets the value of ReadsPersecBucket03512us
    pub fn get_reads_persec_bucket03512us(&self) -> Option<&u64> {
        self.reads_persec_bucket03512us.as_ref()
    }

    /// Sets the value of ReadsPersecBucket041ms
    pub fn set_reads_persec_bucket041ms(&mut self, value: u64) {
        self.reads_persec_bucket041ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket041ms
    pub fn get_reads_persec_bucket041ms(&self) -> Option<&u64> {
        self.reads_persec_bucket041ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket054ms
    pub fn set_reads_persec_bucket054ms(&mut self, value: u64) {
        self.reads_persec_bucket054ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket054ms
    pub fn get_reads_persec_bucket054ms(&self) -> Option<&u64> {
        self.reads_persec_bucket054ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket0616ms
    pub fn set_reads_persec_bucket0616ms(&mut self, value: u64) {
        self.reads_persec_bucket0616ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket0616ms
    pub fn get_reads_persec_bucket0616ms(&self) -> Option<&u64> {
        self.reads_persec_bucket0616ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket0764ms
    pub fn set_reads_persec_bucket0764ms(&mut self, value: u64) {
        self.reads_persec_bucket0764ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket0764ms
    pub fn get_reads_persec_bucket0764ms(&self) -> Option<&u64> {
        self.reads_persec_bucket0764ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket08128ms
    pub fn set_reads_persec_bucket08128ms(&mut self, value: u64) {
        self.reads_persec_bucket08128ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket08128ms
    pub fn get_reads_persec_bucket08128ms(&self) -> Option<&u64> {
        self.reads_persec_bucket08128ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket09256ms
    pub fn set_reads_persec_bucket09256ms(&mut self, value: u64) {
        self.reads_persec_bucket09256ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket09256ms
    pub fn get_reads_persec_bucket09256ms(&self) -> Option<&u64> {
        self.reads_persec_bucket09256ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket10512ms
    pub fn set_reads_persec_bucket10512ms(&mut self, value: u64) {
        self.reads_persec_bucket10512ms = Some(value);
    }

    /// Gets the value of ReadsPersecBucket10512ms
    pub fn get_reads_persec_bucket10512ms(&self) -> Option<&u64> {
        self.reads_persec_bucket10512ms.as_ref()
    }

    /// Sets the value of ReadsPersecBucket111s
    pub fn set_reads_persec_bucket111s(&mut self, value: u64) {
        self.reads_persec_bucket111s = Some(value);
    }

    /// Gets the value of ReadsPersecBucket111s
    pub fn get_reads_persec_bucket111s(&self) -> Option<&u64> {
        self.reads_persec_bucket111s.as_ref()
    }

    /// Sets the value of ReadsPersecBucket122s
    pub fn set_reads_persec_bucket122s(&mut self, value: u64) {
        self.reads_persec_bucket122s = Some(value);
    }

    /// Gets the value of ReadsPersecBucket122s
    pub fn get_reads_persec_bucket122s(&self) -> Option<&u64> {
        self.reads_persec_bucket122s.as_ref()
    }

    /// Sets the value of ReadsPersecBucket1310s
    pub fn set_reads_persec_bucket1310s(&mut self, value: u64) {
        self.reads_persec_bucket1310s = Some(value);
    }

    /// Gets the value of ReadsPersecBucket1310s
    pub fn get_reads_persec_bucket1310s(&self) -> Option<&u64> {
        self.reads_persec_bucket1310s.as_ref()
    }

    /// Sets the value of ReadsPersecBucket1410s
    pub fn set_reads_persec_bucket1410s(&mut self, value: u64) {
        self.reads_persec_bucket1410s = Some(value);
    }

    /// Gets the value of ReadsPersecBucket1410s
    pub fn get_reads_persec_bucket1410s(&self) -> Option<&u64> {
        self.reads_persec_bucket1410s.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket014K
    pub fn set_successful_reads_persec_bucket014_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket014_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket014K
    pub fn get_successful_reads_persec_bucket014_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket014_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket028K
    pub fn set_successful_reads_persec_bucket028_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket028_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket028K
    pub fn get_successful_reads_persec_bucket028_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket028_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket0316K
    pub fn set_successful_reads_persec_bucket0316_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket0316_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket0316K
    pub fn get_successful_reads_persec_bucket0316_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket0316_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket0432K
    pub fn set_successful_reads_persec_bucket0432_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket0432_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket0432K
    pub fn get_successful_reads_persec_bucket0432_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket0432_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket0564K
    pub fn set_successful_reads_persec_bucket0564_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket0564_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket0564K
    pub fn get_successful_reads_persec_bucket0564_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket0564_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket06128K
    pub fn set_successful_reads_persec_bucket06128_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket06128_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket06128K
    pub fn get_successful_reads_persec_bucket06128_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket06128_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket07256K
    pub fn set_successful_reads_persec_bucket07256_k(&mut self, value: u64) {
        self.successful_reads_persec_bucket07256_k = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket07256K
    pub fn get_successful_reads_persec_bucket07256_k(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket07256_k.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket081M
    pub fn set_successful_reads_persec_bucket081_m(&mut self, value: u64) {
        self.successful_reads_persec_bucket081_m = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket081M
    pub fn get_successful_reads_persec_bucket081_m(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket081_m.as_ref()
    }

    /// Sets the value of SuccessfulReadsPersecBucket091M
    pub fn set_successful_reads_persec_bucket091_m(&mut self, value: u64) {
        self.successful_reads_persec_bucket091_m = Some(value);
    }

    /// Gets the value of SuccessfulReadsPersecBucket091M
    pub fn get_successful_reads_persec_bucket091_m(&self) -> Option<&u64> {
        self.successful_reads_persec_bucket091_m.as_ref()
    }
}

