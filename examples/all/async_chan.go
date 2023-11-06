package main

import (
	"fmt"
	"io"
	"net/http"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	messages := make(chan string)
	eggs := [4]string{"A", "B", "C", "D"}

	for _, egg := range eggs {
		wg.Add(1)
		egg := egg
		go func() {
			messages <- fryEgg(egg)
			wg.Done()
		}()
	}

	go func() {
		wg.Wait()
		close(messages)
	}()

	for msg := range messages {
		fmt.Println(msg)
	}

}

func fryEgg(name string) string {
	resp, err := http.Get("http://127.0.0.1:3001/" + name)
	if err != nil {
		panic("oh fk")
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		panic("oh fk")
	}
	return string(body)
}
