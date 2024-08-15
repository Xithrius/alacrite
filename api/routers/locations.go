package routers

import (
	"github.com/Xithrius/alacrite/api/controllers"
	"github.com/gin-gonic/gin"
)

func AddLocationEndpoints(rg *gin.RouterGroup, location_controller *controllers.LocationController) {
	locations := rg.Group("/locations")

	locations.POST("/", location_controller.CreateLocation)
}
