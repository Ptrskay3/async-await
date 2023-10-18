package main

import (
	"fmt"
	"io"
	"net/http"
	// "time"
)

func main() {
	channel := make(chan string)
	users := [4]string{"A", "B", "C", "D"}

	for _, user := range users {
		u := user
		go func() {
			channel <- getUser(u)
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

func getUser(name string) string {
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
