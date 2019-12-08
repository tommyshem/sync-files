package main

import (
	"crypto/sha512"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
stringToHash := "Hello World!\n"	
hashFile()
hashString(stringToHash)
}

func hashFile(){
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
fmt.Printf("%x \n", h.Sum(nil))
}
// hashString will hash the string passed in
func hashString(stringToHash string){
	h := sha512.New()
	h.Write([]byte(stringToHash))
	// Print Hash
	fmt.Printf("%x \n", h.Sum(nil))

}