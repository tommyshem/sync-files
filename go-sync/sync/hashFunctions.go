package sync

import (
	"crypto/sha512"
	"fmt"
	"io"
	"log"
	"os"
)

// HashFile hash the file passed in by filename
func HashFile() string {
	f, err := os.Open("../test-files/file.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()

	h := sha512.New()
	if _, err := io.Copy(h, f); err != nil {
		log.Fatal(err)
	}
	// Print hash
	return fmt.Sprintf("%x", h.Sum(nil))
}

// HashString will hash the string passed in
func HashString(stringToHash string) string {
	h := sha512.New()
	h.Write([]byte(stringToHash))
	// Print Hash
	return fmt.Sprintf("%x", h.Sum(nil))
}
