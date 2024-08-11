package routes

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func PingEndpoint(c *gin.Context) {
	c.JSON(http.StatusOK, "pong")
}

func (r *Routes) addPingRoutes(rg *gin.RouterGroup) {
	router := rg.Group("/ping")

	router.GET("/", PingEndpoint)
}
