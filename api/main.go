package main

import (
	"fmt"
	"log"

	"github.com/Xithrius/alacrite/api/config"
)

func main() {
	filename := "config.toml"

	// Create the config, loading from a file
	conf, err := config.NewConfig(filename)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Server Port:", conf.Server.Port)
	fmt.Println("Database Name:", conf.Database.Name)

	// Modify the config
	port := conf.Server.Port
	if port == 8080 {
		conf.Server.Port = 3000
		fmt.Println("Changing port to 3000 due to it being 8080")
	}

	// Update the config with the new values
	conf.WriteConfig(filename)

	fmt.Println("Config file updated successfully")
}
