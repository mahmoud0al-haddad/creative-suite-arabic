; Executes the production Pascal evaluator against isolated HKCU fixtures.
; InitializeSetup always returns False: no installation or uninstall registration is created.
#ifndef HarnessOutput
  #error Define HarnessOutput as an owned test output directory.
#endif
#ifndef HarnessReport
  #error Define HarnessReport as an owned test report file.
#endif
#ifndef HarnessKey
  #error Define HarnessKey as a unique HKCU test key.
#endif
[Setup]
AppName=Creative Suite Arabic Runtime QA
AppVersion=1
DefaultDirName={tmp}\CreativeSuiteArabicRuntimeQA
CreateAppDir=no
Uninstallable=no
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
OutputDir={#HarnessOutput}
OutputBaseFilename=runtime-prerequisite-qa
Compression=none
DisableStartupPrompt=yes
[Languages]
Name: "ar"; MessagesFile: "compiler:Languages\Arabic.isl"
Name: "en"; MessagesFile: "compiler:Default.isl"
[CustomMessages]
ar.RuntimeRequired=يلزم تثبيت متطلب التشغيل x64.
en.RuntimeRequired=The x64 runtime prerequisite is required.
ar.RuntimeOffer=فتح صفحة متطلب التشغيل؟
en.RuntimeOffer=Open the runtime prerequisite page?
ar.RuntimeOpenFailed=تعذّر فتح الصفحة.
en.RuntimeOpenFailed=The page could not be opened.
[Code]
#include "..\packaging\runtime-prerequisite.iss"

var
  Checks: Integer;
  Report: String;

procedure Verify(Condition: Boolean; Name: String);
begin
  if not Condition then RaiseException('Runtime QA failed: ' + Name);
  Checks := Checks + 1;
  Report := Report + 'PASS ' + Name + #13#10;
end;

procedure CheckVersion(Text: String; Valid, Sufficient: Boolean);
var
  Parts: RuntimeVersion;
  Parsed: Boolean;
begin
  Parsed := ParseRuntimeVersion(Text, Parts);
  Verify(Parsed = Valid, 'parse ' + Text);
  if Parsed then Verify(SufficientRuntimeVersion(Parts) = Sufficient, 'minimum ' + Text);
end;

procedure CheckRegistryFixture(Name, Version: String; Installed: Cardinal; Expected: Boolean);
var
  Key: String;
begin
  Key := '{#HarnessKey}\' + Name;
  Verify(RegWriteDWordValue(HKCU, Key, 'Installed', Installed), 'write fixture ' + Name);
  Verify(RegWriteStringValue(HKCU, Key, 'Version', Version), 'write version ' + Name);
  Verify(RuntimeInViewAtKey(HKCU, Key) = Expected, 'registry evaluator ' + Name);
end;

function InitializeSetup: Boolean;
var
  NeedsRestart: Boolean;
  ActualAvailable: Boolean;
  OwnFixtures: Boolean;
begin
  Result := False;
  Checks := 0;
  Report := '';
  OwnFixtures := False;
  try
    Verify(not RegKeyExists(HKCU, '{#HarnessKey}'), 'unique isolated test root');
    OwnFixtures := True;
    CheckVersion('', False, False);
    CheckVersion('14.44', False, False);
    CheckVersion('14.44.35207', True, True);
    CheckVersion('v14.44.35207.0', True, True);
    CheckVersion('V14.44.35207.5', True, True);
    CheckVersion('14.44.35206.0', True, False);
    CheckVersion('14.43.99999.0', True, False);
    CheckVersion('14.45.1.0', True, True);
    CheckVersion('15.0.1.0', True, False);
    CheckVersion('14.-44.35207.0', False, False);
    CheckVersion('14.44.35207.', False, False);
    CheckVersion('14.44.35207.0.1', False, False);
    CheckVersion('14.44.35207x.0', False, False);
    CheckVersion('9999999999999999999999.44.35207.0', False, False);
    Verify(not RuntimeInViewAtKey(HKCU, '{#HarnessKey}\missing'), 'missing registry key');
    CheckRegistryFixture('old', 'v14.44.35206.0', 1, False);
    CheckRegistryFixture('current', 'v14.44.35207.0', 1, True);
    CheckRegistryFixture('future-v14', 'v14.45.1.0', 1, True);
    CheckRegistryFixture('not-installed', 'v14.44.35207.0', 0, False);
    CheckRegistryFixture('malformed', 'not a runtime version', 1, False);
    Verify(RuntimeRequirementError(True) = '', 'available prerequisite permits install');
    Verify(RuntimeRequirementError(False) = CustomMessage('RuntimeRequired'), 'missing prerequisite blocks install');
    Verify(Length(RuntimeRequirementError(False)) > 0, 'localized blocking reason');
    ActualAvailable := HasRequiredRuntime;
    NeedsRestart := False;
    Verify(PrepareToInstall(NeedsRestart) = RuntimeRequirementError(ActualAvailable), 'actual machine silent prerequisite path');
    Verify(not NeedsRestart, 'no automatic reboot');
    Report := Report + 'CHECKS=' + IntToStr(Checks) + #13#10;
    Verify(SaveStringToFile('{#HarnessReport}', Report, False), 'write owned report');
  finally
    if OwnFixtures and RegKeyExists(HKCU, '{#HarnessKey}') then
      if not RegDeleteKeyIncludingSubkeys(HKCU, '{#HarnessKey}') then
        RaiseException('Could not remove isolated runtime QA registry fixtures');
  end;
end;
