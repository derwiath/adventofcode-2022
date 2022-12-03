Param(
	[parameter(Mandatory=$true)]
	[ValidateRange(1, 25)]
	[int]
	$Day,

	[String]
	$SourcePackage="dayx"
)
$TargetPackage = "day$Day"
$TargetToml = "$TargetPackage/Cargo.toml"
$TargetMain= "$TargetPackage/src/main.rs"
$WorkspaceToml = "Cargo.toml"
$TempFile = ".tempFile"
$CommitMsg ="$Day`: Add Skeleton"

Copy-Item -Recurse $SourcePackage -Destination $TargetPackage

(Get-Content -Path $TargetToml ) -Replace $SourcePackage, $TargetPackage | Add-Content -Path $TempFile
Remove-Item $TargetToml
Move-Item -Force -Path $TempFile -Destination $TargetToml

(Get-Content -Path $TargetMain ) -Replace $SourcePackage, $TargetPackage | Add-Content -Path $TempFile
Move-Item -Force -Path $TempFile -Destination $TargetMain

$Search="`"$SourcePackage`""
$Replace ="`"$SourcePackage`",`r`n  `"$TargetPackage`""
(Get-Content -Path $WorkspaceToml) -Replace $Search, $Replace | Add-Content -Path $TempFile
Move-Item -Force -Path $TempFile -Destination $WorkspaceToml

git add $TargetPackage
git add $WorkspaceToml
git commit -m $CommitMsg
