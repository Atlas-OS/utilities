go build -v -tags walk_use_cgo -ldflags="-w -s" -gcflags "-dwarf=false" .
pause