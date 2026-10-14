// Include inside the public installer's [Code] section. No download or installation runs automatically.
const
  RuntimeKey = 'SOFTWARE\Microsoft\VisualStudio\14.0\VC\Runtimes\x64';
  RuntimeDownloadPage = 'https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist';

type
  RuntimeVersion = array[0..3] of Cardinal;

function ParseRuntimeVersion(Text: String; var Parts: RuntimeVersion): Boolean;
var
  I, N, AtPart: Integer;
  Token: String;
begin
  Result := False;
  for I := 0 to 3 do Parts[I] := 0;
  if Length(Text) = 0 then Exit;
  if (Text[1] = 'v') or (Text[1] = 'V') then Delete(Text, 1, 1);
  AtPart := 0;
  Token := '';
  for I := 1 to Length(Text) + 1 do begin
    if (I > Length(Text)) or (Copy(Text, I, 1) = '.') then begin
      if (Token = '') or (AtPart > 3) then Exit;
      N := StrToIntDef(Token, -1);
      if N < 0 then Exit;
      Parts[AtPart] := N;
      AtPart := AtPart + 1;
      Token := '';
    end else begin
      if (Text[I] < '0') or (Text[I] > '9') then Exit;
      Token := Token + Text[I];
    end;
  end;
  Result := (AtPart = 3) or (AtPart = 4);
end;

function SufficientRuntimeVersion(Parts: RuntimeVersion): Boolean;
begin
  // MSVC used for the reviewed GUI builds: 14.44.35207. Newer v14 minor/builds are accepted.
  Result := (Parts[0] = 14) and
    ((Parts[1] > 44) or ((Parts[1] = 44) and (Parts[2] >= 35207)));
end;

function RuntimeInViewAtKey(Root: Integer; Key: String): Boolean;
var
  Installed: Cardinal;
  Parts: RuntimeVersion;
  VersionText: String;
begin
  Result := False;
  if not RegQueryDWordValue(Root, Key, 'Installed', Installed) then Exit;
  if Installed <> 1 then Exit;
  if RegQueryStringValue(Root, Key, 'Version', VersionText) then begin
    if ParseRuntimeVersion(VersionText, Parts) then begin
      Result := SufficientRuntimeVersion(Parts);
      Exit;
    end;
  end;
  if not RegQueryDWordValue(Root, Key, 'Major', Parts[0]) then Exit;
  if not RegQueryDWordValue(Root, Key, 'Minor', Parts[1]) then Exit;
  if not RegQueryDWordValue(Root, Key, 'Bld', Parts[2]) then Exit;
  if not RegQueryDWordValue(Root, Key, 'Rbld', Parts[3]) then Parts[3] := 0;
  Result := SufficientRuntimeVersion(Parts);
end;

function RuntimeInView(Root: Integer): Boolean;
begin
  Result := RuntimeInViewAtKey(Root, RuntimeKey);
end;

function HasRequiredRuntime: Boolean;
begin
  Result := False;
  if not IsWin64 then Exit;
  // Query the x64 runtime key in both machine registry views; the x86 key is never accepted.
  Result := RuntimeInView(HKLM64) or RuntimeInView(HKLM32);
end;

function RuntimeRequirementError(Available: Boolean): String;
begin
  if Available then Result := '' else Result := CustomMessage('RuntimeRequired');
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ErrorCode: Integer;
begin
  Result := RuntimeRequirementError(HasRequiredRuntime);
  if Result = '' then Exit;
  if WizardSilent then Exit;
  if MsgBox(CustomMessage('RuntimeOffer'), mbConfirmation, MB_YESNO) = IDYES then
    if not ShellExec('open', RuntimeDownloadPage, '', '', SW_SHOWNORMAL, ewNoWait, ErrorCode) then
      MsgBox(CustomMessage('RuntimeOpenFailed') + #13#10 + RuntimeDownloadPage, mbError, MB_OK);
  // Setup remains blocked until the user installs Microsoft's x64 runtime and presses Retry.
end;
