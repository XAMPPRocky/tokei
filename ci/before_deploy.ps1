# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $pwd.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $env:TEMP
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$ZIP = "$SRC_DIR\$($env:CRATE_NAME)-$($env:APPVEYOR_REPO_TAG_NAME)-$($env:TARGET).zip"

# TODO Update this to package the right artifacts
Copy-Item "$SRC_DIR\target\$($env:TARGET)\release\$(env:CRATE_NAME).exe" '.\'

7z a "$ZIP" *

Push-AppveyorArtifact "$ZIP"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR
