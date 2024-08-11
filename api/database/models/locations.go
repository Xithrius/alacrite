package models

import (
	"time"

	"gorm.io/gorm"
)

type Location struct {
	gorm.Model

	Name string

	SupportedOperatingSystems []OperatingSystem

	CreatedAt time.Time
	UpdatedAt time.Time
}

type OperatingSystem struct {
	gorm.Model

	Name         string
	Abbreviation string

	CreatedAt time.Time
	UpdatedAt time.Time

	LocationID uint
}
