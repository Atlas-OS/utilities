package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

// config
type config struct {
	TIMERRES      int16
	KILL_DWM      bool
	KILL_EXPLORER bool
	DISABLE_IDLE  bool
}

func GetConfig(params ...string) config {
	file, err := ioutil.ReadFile("config.json")
	if err != nil {
		fmt.Println("Error ReadFile:", err)
	}

	var data = config{}
	err = json.Unmarshal(file, &data)
	if err != nil {
		fmt.Println("Error Unmarshal:", err)
	}

	return data
}
