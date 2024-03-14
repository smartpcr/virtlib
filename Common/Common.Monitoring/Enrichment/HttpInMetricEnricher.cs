// -----------------------------------------------------------------------
// <copyright file="HttpInMetricEnricher.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace Common.Monitoring.Enrichment;

using System;
using Microsoft.AspNetCore.Http;
using Microsoft.R9.Service.Middleware;

public class HttpInMetricEnricher : IHttpMetricEnricher
{
    private const string DimensionRequestFailed = "req_failed";
    private const string Yes = "Yes";
    private const string No = "No";

    public void Enrich(IHttpMetricEnrichmentPropertyBag enrichmentBag, HttpContext context, Exception? exception)
    {
        enrichmentBag.Add(DimensionRequestFailed, exception is null ? No : Yes);
    }
}