package main

import (
	"fmt"
	"os"

	"github.com/harry1453/go-common-file-dialog/cfd"
	"github.com/harry1453/go-common-file-dialog/cfdutil"
)

func main() {
	// Check for extension arg
	if len(os.Args) != 2 {
		fmt.Println("Usage: filepicker <ext>")
		os.Exit(1)
	}
	var ext = os.Args[1]
	result, err := cfdutil.ShowOpenFileDialog(cfd.DialogConfig{
		Title:       "Select a File", // Title of the dialog window
		Role:        "AtlasFilePick", // used to differentiate between different file picker dialogs
		FileFilters: []cfd.FileFilter{{DisplayName: ext + " Files", Pattern: "*." + ext}},
	})
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Println(result)
}
