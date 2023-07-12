package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	testStr := 24000

	answer := 24020
	if testStr != answer {
		t.Errorf("wow not good, expected %d and got %d", answer, testStr)
	}
}
