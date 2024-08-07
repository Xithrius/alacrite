package config

import (
	"log"
	"os"

	"github.com/BurntSushi/toml"
)

func NewConfig(f string) (*Config, error) {
	data, err := os.ReadFile(f)
	if err != nil {
		return nil, err
	}

	var config Config
	var errDecode error

	_, errDecode = toml.Decode(string(data), &config)
	if errDecode != nil {
		return nil, errDecode
	}

	return &config, nil
}

func (c *Config) WriteConfig(f string) {
	file, err := os.Create(f)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	encoder := toml.NewEncoder(file)
	if err := encoder.Encode(c); err != nil {
		log.Fatal(err)
	}
}
