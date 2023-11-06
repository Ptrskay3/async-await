package main

import (
	"fmt"
	"io"
	"net/http"
	// "time"
)

func main() {
	channel := make(chan string)
	eggs := [4]string{"A", "B", "C", "D"}

	for _, egg := range eggs {
		egg := egg
		go func() {
			channel <- fryEgg(egg)
		}()
	}

	select {
	case response := <-channel:
		fmt.Println(response)
		// case <-time.After(time.Second):
		// 	fmt.Println("Timed out")
		// 	return
	}

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
