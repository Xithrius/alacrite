package controllers

import (
	"net/http"

	"github.com/Xithrius/alacrite/api/database/models"
	"github.com/gin-gonic/gin"
	"gorm.io/gorm"
)

type LocationController struct {
	DB *gorm.DB
}

func NewLocationController(DB *gorm.DB) *LocationController {
	return &LocationController{DB}
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

	result := ctl.DB.Create(&location)

	if result.Error != nil {
		error_msg := gin.H{"status": "error", "message": "Could not insert user into database"}
		ctx.JSON(http.StatusInternalServerError, error_msg)
		return
	}

	result_data := gin.H{"id": location.ID}
	response := gin.H{"status": "created", "affected": result.RowsAffected, "data": result_data}

	ctx.JSON(http.StatusCreated, response)
}

func (ctl *LocationController) GetLocation(ctx *gin.Context) {
	location_id := ctx.Param("location_id")

	var location models.Location
	result := ctl.DB.First(&location, "id = ?", location_id)

	if result.Error != nil {
		error_msg := gin.H{"status": "fail", "message": "No location could be found"}
		ctx.JSON(http.StatusNotFound, error_msg)
		return
	}

	ctx.JSON(http.StatusOK, gin.H{"status": "success", "data": location})
}
