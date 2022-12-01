Param(
	[parameter(Mandatory=$true)]
	[ValidateRange(1, 25)]
	[int]
	$Day,

	[String]
	$Filename="input.txt",

	[String]
	$CookieFilename=".cookie.txt",

	[switch]
	$Force=$false

)
$OutputDir="day$Day"
$OutputPath="$OutputDir/$Filename"

If (-Not (Test-Path -Path "$CookieFilename" -PathType Leaf)) {
	Write-Error "Failed to find $CookieFilename"
	Exit 1
}
ElseIf (-Not (Test-Path -Path "$OutputDir" -PathType Container)) {
	Write-Error "Failed to find $OutputDir."
	Exit 1
}
ElseIf ((-Not $Force) -And (Test-Path -Path "$OutputPath" -PathType Leaf)) {
	Write-Error "$OutputPath already exists. Use -Force to overwrite."
	Exit 1
}

Write-Host "curl -o $OutputPath --cookie $CookieFilename https://adventofcode.com/2022/day/$Day/input"
curl --cookie $CookieFilename https://adventofcode.com/2022/day/$Day/input | Set-Content $OutputPath

Exit $LASTEXITCODE
