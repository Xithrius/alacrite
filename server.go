package main

import (
	"fmt"
	"log"
	"os"

	"github.com/BurntSushi/toml"
)

type Config struct {
	Server struct {
		Port int
		Host string
	}
	Database struct {
		Name     string
		User     string
		Password string
	}
}

func main() {
	filename := "config.toml"

	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatal(err)
	}

	var config Config
	if _, err := toml.Decode(string(data), &config); err != nil {
		log.Fatal(err)
	}

	fmt.Println("Server Port:", config.Server.Port)
	fmt.Println("Database Name:", config.Database.Name)

	port := config.Server.Port
	if port == 8080 {
		config.Server.Port = 3000
		fmt.Println("Changing port to 3000 due to it being 8080")
	}

    file, err := os.Create(filename)
    if err != nil {
        log.Fatal(err)
    }
    defer file.Close()

    encoder := toml.NewEncoder(file)
    if err := encoder.Encode(config); err != nil {
        log.Fatal(err)
    }

    fmt.Println("Config file updated successfully")
}
