namespace VirtLib.Windows.Tests.Steps;

using System;
using System.Management;
using Windows.Models;
using FluentAssertions;
using Queries;
using TechTalk.SpecFlow;
using TechTalk.SpecFlow.Assist;
using Xunit;

[Binding]
public class ResourceCapabilities
{
    private readonly ScenarioContext _context;

    public ResourceCapabilities(ScenarioContext context)
    {
        _context = context;
    }

    [When(@"I get resource capability for ""(.*)""")]
    public void WhenIGetResourceCapabilityFor(string resourceSubtypeName)
    {
        if (!Enum.TryParse<ResourceSubtype>(resourceSubtypeName, out var resourceSubtype))
        {
            Assert.Fail($"Invalid resource subtype: {resourceSubtypeName}");
        }

        _context.Set(resourceSubtype, "ResourceSubtype");
        var hcs = _context.Get<HostComputeSystem>();
        using var resourceCapabilities = ResourceQueries.CreateDefaultResource(resourceSubtype, hcs.VirtualizationScope);
        resourceCapabilities.Should().NotBeNull();
        _context.Set(resourceCapabilities, "ResourceCapability");
    }

    [Then(@"the default setting path should be ""(.*)""")]
    public void ThenTheDefaultSettingPathShouldBe(string resourceName, Table table)
    {
        var resourceCapability = _context.Get<ManagementObject>("ResourceCapability");
        var caption = resourceCapability["Caption"].ToString();
        caption.Should().Be(resourceName);

        foreach (var row in table.Rows)
        {
            var propertyName = row["Name"];
            var propertyValue = resourceCapability[propertyName];
            propertyValue.Should().NotBeNull();
            var expectedValue = row["Value"];
            propertyValue.ToString().Should().Be(expectedValue);
        }
    }
}