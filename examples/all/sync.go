package main

import (
	"fmt"
	"io"
	"net/http"
)

func main() {
	users := [4]string{"A", "B", "C", "D"}
	for _, user := range users {
		fmt.Println(getUser(user))
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
