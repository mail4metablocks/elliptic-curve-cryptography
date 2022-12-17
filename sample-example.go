package main

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"fmt"
	"log"
)

func main() {
	// Create a new private key using the P-256 curve.
	privateKey, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		log.Fatal(err)
	}

	// Extract the public key from the private key.
	publicKey := privateKey.Public()

	// Sign a message using the private key.
	message := []byte("hello, world")
	r, s, err := ecdsa.Sign(rand.Reader, privateKey, message)
	if err != nil {
		log.Fatal(err)
	}

	// Verify the signature using the public key.
	valid := ecdsa.Verify(publicKey, message, r, s)
	fmt.Println("Signature is valid:", valid)
}
