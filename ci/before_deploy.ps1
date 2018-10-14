cargo build --release
mkdir staging
copy target\release\anyshortcut.exe staging
copy target\release\build\*\out\_anyshortcut.ps1 staging
cd staging
7z a ../%PROJECT_NAME%-%TRAVIS_TAG%-%TARGET%.zip *