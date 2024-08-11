package database

import (
	"gorm.io/driver/postgres"
	"gorm.io/gorm"

	"github.com/Xithrius/alacrite/api/database/models"
)

type Database struct {
	session *gorm.DB
}

func NewDatabase() (*Database, error) {
	// TODO: Get from environment variables and/or config
	dsn := "host=localhost user=alacrite password=alacrite dbname=alacrite port=5432"
	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})

	if err != nil {
		return nil, err
	}

	return &Database{
		session: db,
	}, nil
}

func (db *Database) RunTableAutoMigrations() {
	db.session.AutoMigrate(models.User{})
}

func (db *Database) CreateNewUser() {
	user := models.User{Name: "Jinzhu", Age: 18}

	result := db.session.Create(&user)

	println(user.ID, result.RowsAffected)
}
