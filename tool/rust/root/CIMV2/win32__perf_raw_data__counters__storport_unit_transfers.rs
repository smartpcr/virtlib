// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.CIMV2
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// Win32_PerfRawData_Counters_StorportUnitTransfers struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Win32_PerfRawData_Counters_StorportUnitTransfers {
    #[serde(flatten)]
    pub base: Win32_PerfRawData,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket014K")]
    pub successful_transfers_persec_bucket014_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket028K")]
    pub successful_transfers_persec_bucket028_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket0316K")]
    pub successful_transfers_persec_bucket0316_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket0432K")]
    pub successful_transfers_persec_bucket0432_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket0564K")]
    pub successful_transfers_persec_bucket0564_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket06128K")]
    pub successful_transfers_persec_bucket06128_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket07256K")]
    pub successful_transfers_persec_bucket07256_k: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket081M")]
    pub successful_transfers_persec_bucket081_m: Option<u64>,

/// 
    #[serde(rename = "SuccessfulTransfersPersecBucket091M")]
    pub successful_transfers_persec_bucket091_m: Option<u64>,

/// 
    #[serde(rename = "TransferBytesAverage")]
    pub transfer_bytes_average: Option<u64>,

/// 
    #[serde(rename = "TransferBytesAverage_Base")]
    pub transfer_bytes_average__base: Option<u32>,

/// 
    #[serde(rename = "TransferBytesPersec")]
    pub transfer_bytes_persec: Option<u64>,

/// 
    #[serde(rename = "TransferLatency")]
    pub transfer_latency: Option<u64>,

/// 
    #[serde(rename = "TransferLatency_Base")]
    pub transfer_latency__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket01128us")]
    pub transfer_latency_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket01128us_Base")]
    pub transfer_latency_bucket01128us__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket02256us")]
    pub transfer_latency_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket02256us_Base")]
    pub transfer_latency_bucket02256us__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket03512us")]
    pub transfer_latency_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket03512us_Base")]
    pub transfer_latency_bucket03512us__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket041ms")]
    pub transfer_latency_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket041ms_Base")]
    pub transfer_latency_bucket041ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket054ms")]
    pub transfer_latency_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket054ms_Base")]
    pub transfer_latency_bucket054ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket0616ms")]
    pub transfer_latency_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket0616ms_Base")]
    pub transfer_latency_bucket0616ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket0764ms")]
    pub transfer_latency_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket0764ms_Base")]
    pub transfer_latency_bucket0764ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket08128ms")]
    pub transfer_latency_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket08128ms_Base")]
    pub transfer_latency_bucket08128ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket09256ms")]
    pub transfer_latency_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket09256ms_Base")]
    pub transfer_latency_bucket09256ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket10512ms")]
    pub transfer_latency_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket10512ms_Base")]
    pub transfer_latency_bucket10512ms__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket111s")]
    pub transfer_latency_bucket111s: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket111s_Base")]
    pub transfer_latency_bucket111s__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket122s")]
    pub transfer_latency_bucket122s: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket122s_Base")]
    pub transfer_latency_bucket122s__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket1310s")]
    pub transfer_latency_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket1310s_Base")]
    pub transfer_latency_bucket1310s__base: Option<u32>,

/// 
    #[serde(rename = "TransferLatencyBucket1410s")]
    pub transfer_latency_bucket1410s: Option<u64>,

/// 
    #[serde(rename = "TransferLatencyBucket1410s_Base")]
    pub transfer_latency_bucket1410s__base: Option<u32>,

/// 
    #[serde(rename = "TransfersPersec")]
    pub transfers_persec: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket01128us")]
    pub transfers_persec_bucket01128us: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket02256us")]
    pub transfers_persec_bucket02256us: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket03512us")]
    pub transfers_persec_bucket03512us: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket041ms")]
    pub transfers_persec_bucket041ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket054ms")]
    pub transfers_persec_bucket054ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket0616ms")]
    pub transfers_persec_bucket0616ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket0764ms")]
    pub transfers_persec_bucket0764ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket08128ms")]
    pub transfers_persec_bucket08128ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket09256ms")]
    pub transfers_persec_bucket09256ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket10512ms")]
    pub transfers_persec_bucket10512ms: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket111s")]
    pub transfers_persec_bucket111s: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket122s")]
    pub transfers_persec_bucket122s: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket1310s")]
    pub transfers_persec_bucket1310s: Option<u64>,

/// 
    #[serde(rename = "TransfersPersecBucket1410s")]
    pub transfers_persec_bucket1410s: Option<u64>,
}

impl Win32_PerfRawData_Counters_StorportUnitTransfers {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            base: Win32_PerfRawData::new(),
            successful_transfers_persec_bucket014_k: None,
            successful_transfers_persec_bucket028_k: None,
            successful_transfers_persec_bucket0316_k: None,
            successful_transfers_persec_bucket0432_k: None,
            successful_transfers_persec_bucket0564_k: None,
            successful_transfers_persec_bucket06128_k: None,
            successful_transfers_persec_bucket07256_k: None,
            successful_transfers_persec_bucket081_m: None,
            successful_transfers_persec_bucket091_m: None,
            transfer_bytes_average: None,
            transfer_bytes_average__base: None,
            transfer_bytes_persec: None,
            transfer_latency: None,
            transfer_latency__base: None,
            transfer_latency_bucket01128us: None,
            transfer_latency_bucket01128us__base: None,
            transfer_latency_bucket02256us: None,
            transfer_latency_bucket02256us__base: None,
            transfer_latency_bucket03512us: None,
            transfer_latency_bucket03512us__base: None,
            transfer_latency_bucket041ms: None,
            transfer_latency_bucket041ms__base: None,
            transfer_latency_bucket054ms: None,
            transfer_latency_bucket054ms__base: None,
            transfer_latency_bucket0616ms: None,
            transfer_latency_bucket0616ms__base: None,
            transfer_latency_bucket0764ms: None,
            transfer_latency_bucket0764ms__base: None,
            transfer_latency_bucket08128ms: None,
            transfer_latency_bucket08128ms__base: None,
            transfer_latency_bucket09256ms: None,
            transfer_latency_bucket09256ms__base: None,
            transfer_latency_bucket10512ms: None,
            transfer_latency_bucket10512ms__base: None,
            transfer_latency_bucket111s: None,
            transfer_latency_bucket111s__base: None,
            transfer_latency_bucket122s: None,
            transfer_latency_bucket122s__base: None,
            transfer_latency_bucket1310s: None,
            transfer_latency_bucket1310s__base: None,
            transfer_latency_bucket1410s: None,
            transfer_latency_bucket1410s__base: None,
            transfers_persec: None,
            transfers_persec_bucket01128us: None,
            transfers_persec_bucket02256us: None,
            transfers_persec_bucket03512us: None,
            transfers_persec_bucket041ms: None,
            transfers_persec_bucket054ms: None,
            transfers_persec_bucket0616ms: None,
            transfers_persec_bucket0764ms: None,
            transfers_persec_bucket08128ms: None,
            transfers_persec_bucket09256ms: None,
            transfers_persec_bucket10512ms: None,
            transfers_persec_bucket111s: None,
            transfers_persec_bucket122s: None,
            transfers_persec_bucket1310s: None,
            transfers_persec_bucket1410s: None,
        }
    }


    /// Sets the value of SuccessfulTransfersPersecBucket014K
    pub fn set_successful_transfers_persec_bucket014_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket014_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket014K
    pub fn get_successful_transfers_persec_bucket014_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket014_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket028K
    pub fn set_successful_transfers_persec_bucket028_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket028_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket028K
    pub fn get_successful_transfers_persec_bucket028_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket028_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket0316K
    pub fn set_successful_transfers_persec_bucket0316_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket0316_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket0316K
    pub fn get_successful_transfers_persec_bucket0316_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket0316_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket0432K
    pub fn set_successful_transfers_persec_bucket0432_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket0432_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket0432K
    pub fn get_successful_transfers_persec_bucket0432_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket0432_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket0564K
    pub fn set_successful_transfers_persec_bucket0564_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket0564_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket0564K
    pub fn get_successful_transfers_persec_bucket0564_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket0564_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket06128K
    pub fn set_successful_transfers_persec_bucket06128_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket06128_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket06128K
    pub fn get_successful_transfers_persec_bucket06128_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket06128_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket07256K
    pub fn set_successful_transfers_persec_bucket07256_k(&mut self, value: u64) {
        self.successful_transfers_persec_bucket07256_k = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket07256K
    pub fn get_successful_transfers_persec_bucket07256_k(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket07256_k.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket081M
    pub fn set_successful_transfers_persec_bucket081_m(&mut self, value: u64) {
        self.successful_transfers_persec_bucket081_m = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket081M
    pub fn get_successful_transfers_persec_bucket081_m(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket081_m.as_ref()
    }

    /// Sets the value of SuccessfulTransfersPersecBucket091M
    pub fn set_successful_transfers_persec_bucket091_m(&mut self, value: u64) {
        self.successful_transfers_persec_bucket091_m = Some(value);
    }

    /// Gets the value of SuccessfulTransfersPersecBucket091M
    pub fn get_successful_transfers_persec_bucket091_m(&self) -> Option<&u64> {
        self.successful_transfers_persec_bucket091_m.as_ref()
    }

    /// Sets the value of TransferBytesAverage
    pub fn set_transfer_bytes_average(&mut self, value: u64) {
        self.transfer_bytes_average = Some(value);
    }

    /// Gets the value of TransferBytesAverage
    pub fn get_transfer_bytes_average(&self) -> Option<&u64> {
        self.transfer_bytes_average.as_ref()
    }

    /// Sets the value of TransferBytesAverage_Base
    pub fn set_transfer_bytes_average__base(&mut self, value: u32) {
        self.transfer_bytes_average__base = Some(value);
    }

    /// Gets the value of TransferBytesAverage_Base
    pub fn get_transfer_bytes_average__base(&self) -> Option<&u32> {
        self.transfer_bytes_average__base.as_ref()
    }

    /// Sets the value of TransferBytesPersec
    pub fn set_transfer_bytes_persec(&mut self, value: u64) {
        self.transfer_bytes_persec = Some(value);
    }

    /// Gets the value of TransferBytesPersec
    pub fn get_transfer_bytes_persec(&self) -> Option<&u64> {
        self.transfer_bytes_persec.as_ref()
    }

    /// Sets the value of TransferLatency
    pub fn set_transfer_latency(&mut self, value: u64) {
        self.transfer_latency = Some(value);
    }

    /// Gets the value of TransferLatency
    pub fn get_transfer_latency(&self) -> Option<&u64> {
        self.transfer_latency.as_ref()
    }

    /// Sets the value of TransferLatency_Base
    pub fn set_transfer_latency__base(&mut self, value: u32) {
        self.transfer_latency__base = Some(value);
    }

    /// Gets the value of TransferLatency_Base
    pub fn get_transfer_latency__base(&self) -> Option<&u32> {
        self.transfer_latency__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket01128us
    pub fn set_transfer_latency_bucket01128us(&mut self, value: u64) {
        self.transfer_latency_bucket01128us = Some(value);
    }

    /// Gets the value of TransferLatencyBucket01128us
    pub fn get_transfer_latency_bucket01128us(&self) -> Option<&u64> {
        self.transfer_latency_bucket01128us.as_ref()
    }

    /// Sets the value of TransferLatencyBucket01128us_Base
    pub fn set_transfer_latency_bucket01128us__base(&mut self, value: u32) {
        self.transfer_latency_bucket01128us__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket01128us_Base
    pub fn get_transfer_latency_bucket01128us__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket01128us__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket02256us
    pub fn set_transfer_latency_bucket02256us(&mut self, value: u64) {
        self.transfer_latency_bucket02256us = Some(value);
    }

    /// Gets the value of TransferLatencyBucket02256us
    pub fn get_transfer_latency_bucket02256us(&self) -> Option<&u64> {
        self.transfer_latency_bucket02256us.as_ref()
    }

    /// Sets the value of TransferLatencyBucket02256us_Base
    pub fn set_transfer_latency_bucket02256us__base(&mut self, value: u32) {
        self.transfer_latency_bucket02256us__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket02256us_Base
    pub fn get_transfer_latency_bucket02256us__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket02256us__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket03512us
    pub fn set_transfer_latency_bucket03512us(&mut self, value: u64) {
        self.transfer_latency_bucket03512us = Some(value);
    }

    /// Gets the value of TransferLatencyBucket03512us
    pub fn get_transfer_latency_bucket03512us(&self) -> Option<&u64> {
        self.transfer_latency_bucket03512us.as_ref()
    }

    /// Sets the value of TransferLatencyBucket03512us_Base
    pub fn set_transfer_latency_bucket03512us__base(&mut self, value: u32) {
        self.transfer_latency_bucket03512us__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket03512us_Base
    pub fn get_transfer_latency_bucket03512us__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket03512us__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket041ms
    pub fn set_transfer_latency_bucket041ms(&mut self, value: u64) {
        self.transfer_latency_bucket041ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket041ms
    pub fn get_transfer_latency_bucket041ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket041ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket041ms_Base
    pub fn set_transfer_latency_bucket041ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket041ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket041ms_Base
    pub fn get_transfer_latency_bucket041ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket041ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket054ms
    pub fn set_transfer_latency_bucket054ms(&mut self, value: u64) {
        self.transfer_latency_bucket054ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket054ms
    pub fn get_transfer_latency_bucket054ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket054ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket054ms_Base
    pub fn set_transfer_latency_bucket054ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket054ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket054ms_Base
    pub fn get_transfer_latency_bucket054ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket054ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket0616ms
    pub fn set_transfer_latency_bucket0616ms(&mut self, value: u64) {
        self.transfer_latency_bucket0616ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket0616ms
    pub fn get_transfer_latency_bucket0616ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket0616ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket0616ms_Base
    pub fn set_transfer_latency_bucket0616ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket0616ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket0616ms_Base
    pub fn get_transfer_latency_bucket0616ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket0616ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket0764ms
    pub fn set_transfer_latency_bucket0764ms(&mut self, value: u64) {
        self.transfer_latency_bucket0764ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket0764ms
    pub fn get_transfer_latency_bucket0764ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket0764ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket0764ms_Base
    pub fn set_transfer_latency_bucket0764ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket0764ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket0764ms_Base
    pub fn get_transfer_latency_bucket0764ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket0764ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket08128ms
    pub fn set_transfer_latency_bucket08128ms(&mut self, value: u64) {
        self.transfer_latency_bucket08128ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket08128ms
    pub fn get_transfer_latency_bucket08128ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket08128ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket08128ms_Base
    pub fn set_transfer_latency_bucket08128ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket08128ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket08128ms_Base
    pub fn get_transfer_latency_bucket08128ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket08128ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket09256ms
    pub fn set_transfer_latency_bucket09256ms(&mut self, value: u64) {
        self.transfer_latency_bucket09256ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket09256ms
    pub fn get_transfer_latency_bucket09256ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket09256ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket09256ms_Base
    pub fn set_transfer_latency_bucket09256ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket09256ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket09256ms_Base
    pub fn get_transfer_latency_bucket09256ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket09256ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket10512ms
    pub fn set_transfer_latency_bucket10512ms(&mut self, value: u64) {
        self.transfer_latency_bucket10512ms = Some(value);
    }

    /// Gets the value of TransferLatencyBucket10512ms
    pub fn get_transfer_latency_bucket10512ms(&self) -> Option<&u64> {
        self.transfer_latency_bucket10512ms.as_ref()
    }

    /// Sets the value of TransferLatencyBucket10512ms_Base
    pub fn set_transfer_latency_bucket10512ms__base(&mut self, value: u32) {
        self.transfer_latency_bucket10512ms__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket10512ms_Base
    pub fn get_transfer_latency_bucket10512ms__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket10512ms__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket111s
    pub fn set_transfer_latency_bucket111s(&mut self, value: u64) {
        self.transfer_latency_bucket111s = Some(value);
    }

    /// Gets the value of TransferLatencyBucket111s
    pub fn get_transfer_latency_bucket111s(&self) -> Option<&u64> {
        self.transfer_latency_bucket111s.as_ref()
    }

    /// Sets the value of TransferLatencyBucket111s_Base
    pub fn set_transfer_latency_bucket111s__base(&mut self, value: u32) {
        self.transfer_latency_bucket111s__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket111s_Base
    pub fn get_transfer_latency_bucket111s__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket111s__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket122s
    pub fn set_transfer_latency_bucket122s(&mut self, value: u64) {
        self.transfer_latency_bucket122s = Some(value);
    }

    /// Gets the value of TransferLatencyBucket122s
    pub fn get_transfer_latency_bucket122s(&self) -> Option<&u64> {
        self.transfer_latency_bucket122s.as_ref()
    }

    /// Sets the value of TransferLatencyBucket122s_Base
    pub fn set_transfer_latency_bucket122s__base(&mut self, value: u32) {
        self.transfer_latency_bucket122s__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket122s_Base
    pub fn get_transfer_latency_bucket122s__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket122s__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket1310s
    pub fn set_transfer_latency_bucket1310s(&mut self, value: u64) {
        self.transfer_latency_bucket1310s = Some(value);
    }

    /// Gets the value of TransferLatencyBucket1310s
    pub fn get_transfer_latency_bucket1310s(&self) -> Option<&u64> {
        self.transfer_latency_bucket1310s.as_ref()
    }

    /// Sets the value of TransferLatencyBucket1310s_Base
    pub fn set_transfer_latency_bucket1310s__base(&mut self, value: u32) {
        self.transfer_latency_bucket1310s__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket1310s_Base
    pub fn get_transfer_latency_bucket1310s__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket1310s__base.as_ref()
    }

    /// Sets the value of TransferLatencyBucket1410s
    pub fn set_transfer_latency_bucket1410s(&mut self, value: u64) {
        self.transfer_latency_bucket1410s = Some(value);
    }

    /// Gets the value of TransferLatencyBucket1410s
    pub fn get_transfer_latency_bucket1410s(&self) -> Option<&u64> {
        self.transfer_latency_bucket1410s.as_ref()
    }

    /// Sets the value of TransferLatencyBucket1410s_Base
    pub fn set_transfer_latency_bucket1410s__base(&mut self, value: u32) {
        self.transfer_latency_bucket1410s__base = Some(value);
    }

    /// Gets the value of TransferLatencyBucket1410s_Base
    pub fn get_transfer_latency_bucket1410s__base(&self) -> Option<&u32> {
        self.transfer_latency_bucket1410s__base.as_ref()
    }

    /// Sets the value of TransfersPersec
    pub fn set_transfers_persec(&mut self, value: u64) {
        self.transfers_persec = Some(value);
    }

    /// Gets the value of TransfersPersec
    pub fn get_transfers_persec(&self) -> Option<&u64> {
        self.transfers_persec.as_ref()
    }

    /// Sets the value of TransfersPersecBucket01128us
    pub fn set_transfers_persec_bucket01128us(&mut self, value: u64) {
        self.transfers_persec_bucket01128us = Some(value);
    }

    /// Gets the value of TransfersPersecBucket01128us
    pub fn get_transfers_persec_bucket01128us(&self) -> Option<&u64> {
        self.transfers_persec_bucket01128us.as_ref()
    }

    /// Sets the value of TransfersPersecBucket02256us
    pub fn set_transfers_persec_bucket02256us(&mut self, value: u64) {
        self.transfers_persec_bucket02256us = Some(value);
    }

    /// Gets the value of TransfersPersecBucket02256us
    pub fn get_transfers_persec_bucket02256us(&self) -> Option<&u64> {
        self.transfers_persec_bucket02256us.as_ref()
    }

    /// Sets the value of TransfersPersecBucket03512us
    pub fn set_transfers_persec_bucket03512us(&mut self, value: u64) {
        self.transfers_persec_bucket03512us = Some(value);
    }

    /// Gets the value of TransfersPersecBucket03512us
    pub fn get_transfers_persec_bucket03512us(&self) -> Option<&u64> {
        self.transfers_persec_bucket03512us.as_ref()
    }

    /// Sets the value of TransfersPersecBucket041ms
    pub fn set_transfers_persec_bucket041ms(&mut self, value: u64) {
        self.transfers_persec_bucket041ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket041ms
    pub fn get_transfers_persec_bucket041ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket041ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket054ms
    pub fn set_transfers_persec_bucket054ms(&mut self, value: u64) {
        self.transfers_persec_bucket054ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket054ms
    pub fn get_transfers_persec_bucket054ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket054ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket0616ms
    pub fn set_transfers_persec_bucket0616ms(&mut self, value: u64) {
        self.transfers_persec_bucket0616ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket0616ms
    pub fn get_transfers_persec_bucket0616ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket0616ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket0764ms
    pub fn set_transfers_persec_bucket0764ms(&mut self, value: u64) {
        self.transfers_persec_bucket0764ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket0764ms
    pub fn get_transfers_persec_bucket0764ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket0764ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket08128ms
    pub fn set_transfers_persec_bucket08128ms(&mut self, value: u64) {
        self.transfers_persec_bucket08128ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket08128ms
    pub fn get_transfers_persec_bucket08128ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket08128ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket09256ms
    pub fn set_transfers_persec_bucket09256ms(&mut self, value: u64) {
        self.transfers_persec_bucket09256ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket09256ms
    pub fn get_transfers_persec_bucket09256ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket09256ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket10512ms
    pub fn set_transfers_persec_bucket10512ms(&mut self, value: u64) {
        self.transfers_persec_bucket10512ms = Some(value);
    }

    /// Gets the value of TransfersPersecBucket10512ms
    pub fn get_transfers_persec_bucket10512ms(&self) -> Option<&u64> {
        self.transfers_persec_bucket10512ms.as_ref()
    }

    /// Sets the value of TransfersPersecBucket111s
    pub fn set_transfers_persec_bucket111s(&mut self, value: u64) {
        self.transfers_persec_bucket111s = Some(value);
    }

    /// Gets the value of TransfersPersecBucket111s
    pub fn get_transfers_persec_bucket111s(&self) -> Option<&u64> {
        self.transfers_persec_bucket111s.as_ref()
    }

    /// Sets the value of TransfersPersecBucket122s
    pub fn set_transfers_persec_bucket122s(&mut self, value: u64) {
        self.transfers_persec_bucket122s = Some(value);
    }

    /// Gets the value of TransfersPersecBucket122s
    pub fn get_transfers_persec_bucket122s(&self) -> Option<&u64> {
        self.transfers_persec_bucket122s.as_ref()
    }

    /// Sets the value of TransfersPersecBucket1310s
    pub fn set_transfers_persec_bucket1310s(&mut self, value: u64) {
        self.transfers_persec_bucket1310s = Some(value);
    }

    /// Gets the value of TransfersPersecBucket1310s
    pub fn get_transfers_persec_bucket1310s(&self) -> Option<&u64> {
        self.transfers_persec_bucket1310s.as_ref()
    }

    /// Sets the value of TransfersPersecBucket1410s
    pub fn set_transfers_persec_bucket1410s(&mut self, value: u64) {
        self.transfers_persec_bucket1410s = Some(value);
    }

    /// Gets the value of TransfersPersecBucket1410s
    pub fn get_transfers_persec_bucket1410s(&self) -> Option<&u64> {
        self.transfers_persec_bucket1410s.as_ref()
    }
}

