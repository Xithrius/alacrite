package controllers

import (
	"net/http"

	"github.com/Xithrius/alacrite/api/database/models"
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type LocationController struct {
	DBSession *gorm.DB
}

func NewLocationController(DBSession *gorm.DB) *LocationController {
	return &LocationController{DBSession}
}

func (ctl *LocationController) CreateLocation(ctx *gin.Context) {
	operating_systems := []models.OperatingSystem{
		{
			Name:         "Arch linux",
			Abbreviation: "Linux",
		},
	}

	location := &models.Location{
		Name:                      "asdf",
		SupportedOperatingSystems: operating_systems,
	}

	result := ctl.DBSession.Create(&location)

	if result.Error != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{"status": "error", "message": "Could not insert user into database"})
	}

	ctx.JSON(http.StatusCreated, gin.H{"status": "created"})
}
