// Copyright 2019 (c) Microsoft Corporation.
// Licensed under the MIT license.

//
// Author:
//      Auto Generated on 12/5/2025 using wmigen
//      Source root.Microsoft.Windows.ClusterUpdate
//////////////////////////////////////////////
use crate::wmi;
use crate::cim;


/// MSFT_CAUReportHelper struct
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSFT_CAUReportHelper {

/// 
    #[serde(rename = "OrchestratorGuid")]
    pub orchestrator_guid: Option<String>,
}

impl MSFT_CAUReportHelper {
    /// Creates a new instance of the struct
    pub fn new() -> Self {
        Self {
            orchestrator_guid: None,
        }
    }


    /// Sets the value of OrchestratorGuid
    pub fn set_orchestrator_guid(&mut self, value: String) {
        self.orchestrator_guid = Some(value);
    }

    /// Gets the value of OrchestratorGuid
    pub fn get_orchestrator_guid(&self) -> Option<&String> {
        self.orchestrator_guid.as_ref()
    }

/// 

    /// * `report_id` -  (MSFT_CAURun_Report_ID[])
    /// * `return_value` -  (u32)
    pub fn get_report_ids(&self, report_id: &mut Vec<MSFT_CAURun_Report_ID>) -> Result<(), WmiError> {

        let result = self.invoke_method("GetReportIDs", &[])?;
        let report_id = result.get_value("ReportID")?;
        Ok(result.return_value)

    }


/// 

    /// * `chunk_size` -  (u32)
    /// * `report_id` -  (MSFT_CAURun_Report_ID)

    /// * `report_chunks` -  (MSFT_CAURun_Report_Chunk[])
    /// * `return_value` -  (u32)
    pub fn get_report(&self, report_id: MSFT_CAURun_Report_ID, chunk_size: u32, report_chunks: &mut Vec<MSFT_CAURun_Report_Chunk>) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReportId".to_string(), value: report_id.into() });
        args.push(MethodParameter { name: "ChunkSize".to_string(), value: chunk_size.into() });

        let result = self.invoke_method("GetReport", &args)?;
        let report_chunks = result.get_value("ReportChunks")?;
        Ok(result.return_value)

    }


/// 

    /// * `last_chunk` -  (bool)
    /// * `report_chunk` -  (MSFT_CAURun_Report_Chunk)
    /// * `report_size` -  (u64)

    /// * `return_value` -  (u32)
    pub fn put_report(&self, report_size: u64, report_chunk: MSFT_CAURun_Report_Chunk, last_chunk: bool) -> Result<(), WmiError> {
        let mut args = Vec::new();
        args.push(MethodParameter { name: "ReportSize".to_string(), value: report_size.into() });
        args.push(MethodParameter { name: "ReportChunk".to_string(), value: report_chunk.into() });
        args.push(MethodParameter { name: "LastChunk".to_string(), value: last_chunk.into() });
        self.invoke_method("PutReport", &args)

    }

}

