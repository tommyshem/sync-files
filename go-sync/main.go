package main

import (
	"github.com/tommyshem/sync-files/sync"
)

func main() {
	stringToHash := "Hello World! 1234567**"
	sync.HashFile()
	sync.HashString(stringToHash)
}
