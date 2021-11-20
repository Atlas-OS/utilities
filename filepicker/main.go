package main

import (
	"fmt"

	"github.com/harry1453/go-common-file-dialog/cfd"
	"github.com/harry1453/go-common-file-dialog/cfdutil"
)

func main() {
	result, err := cfdutil.ShowOpenFileDialog(cfd.DialogConfig{
		Title:       "Open An Executable", // Title of the dialog window
		Role:        "AtlasFilePick",      // used to differentiate between different file picker dialogs
		FileFilters: []cfd.FileFilter{{DisplayName: "Executable Files (*.exe)", Pattern: "*.exe"}},
	})
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(result)
}
