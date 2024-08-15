package database

import (
	"gorm.io/driver/postgres"
	"gorm.io/gorm"

	"github.com/Xithrius/alacrite/api/database/models"
)

type Database struct {
	Session *gorm.DB
}

func NewDatabase() (*Database, error) {
	// TODO: Get from environment variables and/or config
	dsn := "host=localhost user=alacrite password=alacrite dbname=alacrite port=5432"
	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if err != nil {
		return nil, err
	}

	return &Database{
		Session: db,
	}, nil
}

func (db *Database) RunTableAutoMigrations() {
	db.Session.AutoMigrate(models.OperatingSystem{}, models.Location{})
}
