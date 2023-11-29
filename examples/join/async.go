// Run with: `go run async.go`.
package main

import (
	"fmt"
	"io"
	"net/http"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	eggs := [4]string{"A", "B", "C", "D"}

	for _, egg := range eggs {
		wg.Add(1)
		go func(egg string) {
			fmt.Println(fryEgg(egg))
			wg.Done()
		}(egg)
	}

	wg.Wait()
}

func fryEgg(name string) string {
	resp, err := http.Get("http://127.0.0.1:3001/" + name)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}
	return string(body)
}
