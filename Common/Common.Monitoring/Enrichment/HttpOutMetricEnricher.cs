// -----------------------------------------------------------------------
// <copyright file="HttpOutMetricEnricher.cs" company="Microsoft Corp.">
//     Copyright (c) Microsoft Corp. All rights reserved.
// </copyright>
// -----------------------------------------------------------------------

namespace Common.Monitoring.Enrichment;

using System;
using System.Net.Http;
using Microsoft.AspNetCore.Http;
using Microsoft.R9.Extensions.HttpClient.Metering;

internal class HttpOutMetricEnricher : IHttpClientMetricEnricher
{
    private const string DimensionRequestFailed = "req_failed";
    private const string DimensionInWebContext = "req_in_web_context";
    private const string Yes = "Yes";
    private const string No = "No";

    private readonly IHttpContextAccessor _httpContextAccessor;

    public HttpOutMetricEnricher(IHttpContextAccessor httpContextAccessor)
    {
        _httpContextAccessor = httpContextAccessor;
    }

    public void Enrich(IHttpClientMetricEnrichmentPropertyBag enrichmentBag, HttpRequestMessage request, HttpResponseMessage response)
    {
        enrichmentBag.Add(DimensionRequestFailed, No);
        AddDimensionInWebContext(enrichmentBag);
    }

    public void Enrich(IHttpClientMetricEnrichmentPropertyBag enrichmentBag, HttpRequestMessage request, Exception exception)
    {
        enrichmentBag.Add(DimensionRequestFailed, Yes);
        AddDimensionInWebContext(enrichmentBag);
    }

    private void AddDimensionInWebContext(IHttpClientMetricEnrichmentPropertyBag enrichmentBag)
    {
        enrichmentBag.Add(DimensionInWebContext, _httpContextAccessor.HttpContext is null ? No : Yes);
    }
}