package sync

import (
	"fmt"
	"io/ioutil"
	"os"
)

// CheckFileInfo returns a fileinfo structure of the filename passed in
func CheckFileInfo(filename string) os.FileInfo {
	// read files info
	destinationFileInfo, err := os.Stat(filename)
	if err != nil && !os.IsNotExist(err) {
		panic(err)
	}
	return destinationFileInfo
}

// ReadAllFiles goes through all the files
func ReadAllFiles(filename string) {
	files, err := ioutil.ReadDir(filename)
	if os.IsNotExist(err) {
		return
	}
	m := make(map[string]bool, len(files))
	for _, file := range files {
		m[file.Name()] = true

	}
	fmt.Println("map:", m)
}
