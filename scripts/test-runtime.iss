; Isolated read-only self-test: exits before installation and never edits registry keys.
[Setup]
AppId=CreativeSuiteArabicRuntimeSelfTest
AppName=Runtime prerequisite self-test
AppVersion=1
DefaultDirName={tmp}\RuntimePrerequisiteSelfTest
PrivilegesRequired=lowest
Uninstallable=no
CreateAppDir=no
OutputDir=..\.build\verification
OutputBaseFilename=runtime-prerequisite-self-test
Compression=none

[CustomMessages]
RuntimeRequired=Runtime prerequisite missing.
RuntimeOffer=Open the official Microsoft page?
RuntimeOpenFailed=Could not open browser.

[Code]
#include "..\packaging\runtime-prerequisite.iss"

var
  TestCount: Integer;
  TestOkay: Boolean;
  TestDetails: String;

procedure CheckVersion(Text: String; ExpectParsed, ExpectSufficient: Boolean);
var
  Parts: RuntimeVersion;
  Parsed, Sufficient, Okay: Boolean;
begin
  Parsed := ParseRuntimeVersion(Text, Parts);
  Sufficient := Parsed and SufficientRuntimeVersion(Parts);
  Okay := (Parsed = ExpectParsed) and (Sufficient = ExpectSufficient);
  TestCount := TestCount + 1;
  TestOkay := TestOkay and Okay;
  TestDetails := TestDetails + Text + ': ' + IntToStr(Ord(Okay)) + #13#10;
end;

function InitializeSetup: Boolean;
var
  Missing32, Missing64, Actual32, Actual64: Boolean;
  ReportPath: String;
begin
  TestCount := 0;
  TestOkay := True;
  TestDetails := '';
  CheckVersion('', False, False);
  CheckVersion('garbage', False, False);
  CheckVersion('v14.44.invalid.0', False, False);
  CheckVersion('14..44.35207', False, False);
  CheckVersion('14.44.35207.0.1', False, False);
  CheckVersion('14.43.99999.0', True, False);
  CheckVersion('14.44.35206.999', True, False);
  CheckVersion('14.44.35207', True, True);
  CheckVersion('v14.44.35207.0', True, True);
  CheckVersion('14.44.35211.0', True, True);
  CheckVersion('14.51.36247.0', True, True);
  CheckVersion('15.0.1.0', True, False);
  Missing32 := RuntimeInViewAtKey(HKLM32, 'SOFTWARE\CreativeSuiteArabic\SyntheticMissingRuntime\37A7A2B8');
  Missing64 := False;
  if IsWin64 then Missing64 := RuntimeInViewAtKey(HKLM64, 'SOFTWARE\CreativeSuiteArabic\SyntheticMissingRuntime\37A7A2B8');
  TestOkay := TestOkay and not Missing32 and not Missing64;
  TestCount := TestCount + 2;
  TestOkay := TestOkay and (RuntimeRequirementError(False) <> '') and (RuntimeRequirementError(True) = '');
  TestCount := TestCount + 2;
  Actual32 := RuntimeInView(HKLM32);
  Actual64 := False;
  if IsWin64 then Actual64 := RuntimeInView(HKLM64);
  ReportPath := ExpandConstant('{param:Report|{tmp}\CreativeSuiteArabicRuntimeSelfTest.txt}');
  SaveStringToFile(ReportPath, 'passed=' + IntToStr(Ord(TestOkay)) + #13#10 +
    'tests=' + IntToStr(TestCount) + #13#10 + 'actual32=' + IntToStr(Ord(Actual32)) + #13#10 +
    'actual64=' + IntToStr(Ord(Actual64)) + #13#10 + TestDetails, False);
  // Abort intentionally before installation, regardless of self-test outcome.
  Result := False;
end;
