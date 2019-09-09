New-Item -ItemType Directory -Name temp
Copy-Item README.md temp
Copy-Item LICENSE-MIT temp
Copy-Item LICENSE-APACHE temp
Get-ChildItem target\$Env:TARGET\release
Copy-Item target\$Env:TARGET\release\$Env:PROJECT_NAME.exe temp
7z a $Env:PROJECT_NAME-$Env:TRAVIS_TAG-$Env:TARGET.zip temp\*
Get-ChildItem