if exist main.exe del /f /q main.exe
go build -ldflags "-s" -gcflags "-dwarf=false" main.go
pause