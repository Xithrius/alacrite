package config

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
