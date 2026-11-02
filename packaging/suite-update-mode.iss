// The update helper verifies installed files and writes its result before restarting.
// Manual installations retain the ordinary Finish-page launcher checkbox.
function ShouldOfferLauncher: Boolean;
begin
  Result := ExpandConstant('{param:SUITEUPDATE|0}') <> '1';
end;
