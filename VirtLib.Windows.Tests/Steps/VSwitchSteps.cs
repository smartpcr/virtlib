namespace VirtLib.Windows.Tests.Steps;

using System;
using FluentAssertions;
using TechTalk.SpecFlow;
using Windows.Models;

[Binding]
public class VSwitchSteps
{
    private readonly ScenarioContext _context;

    public VSwitchSteps(ScenarioContext context)
    {
        _context = context;
    }

    [Given(@"hyper-v host running on localhost")]
    public void GivenHyperVHostRunningOnLocalhost()
    {
        var hcs = new HostComputeSystem();
        _context.Set(hcs);
    }

    [When(@"I check virtual switch with name ""(.*)""")]
    public void WhenICheckVirtualSwitchWithName(string vswitchName)
    {
        var hcs = _context.Get<HostComputeSystem>();
        var vswitch = hcs.GetVSwitch(vswitchName);
        _context.Set(vswitch);
    }

    [Then(@"the virtual switch should exist")]
    public void ThenTheVirtualSwitchShouldExist()
    {
        var vswitch = _context.Get<SwitchInfo>();
        vswitch.Should().NotBeNull();
    }

    [Then(@"the virtual switch connection type should be ""(.*)""")]
    public void ThenTheVirtualSwitchConnectionTypeShouldBe(string connectionType)
    {
        SwitchConnectionType expectedType = connectionType switch
        {
            "External" => SwitchConnectionType.External,
            "Internal" => SwitchConnectionType.Internal,
            "Private" => SwitchConnectionType.Private,
            _ => throw new ArgumentOutOfRangeException(nameof(connectionType), connectionType, null)
        };
        var vswitch = _context.Get<SwitchInfo>();
        vswitch.ConnectionType.Should().Be(expectedType);
    }

    [Then(@"the virtual switch vlan should be disabled")]
    public void ThenTheVirtualSwitchVlanShouldBeDisabled()
    {
        var vswitch = _context.Get<SwitchInfo>();
        vswitch.VLanEnabled.Should().BeFalse();
    }
}