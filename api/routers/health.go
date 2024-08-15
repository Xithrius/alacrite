package routers

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func PingEndpoint(c *gin.Context) {
	c.String(http.StatusOK, "pong")
}

func HealthEndpoint(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{"state": "healthy"})
}

func AddHealthEndpoints(rg *gin.RouterGroup) {
	router := rg.Group("/")

	router.GET("/ping", PingEndpoint)
	router.GET("/health", HealthEndpoint)
}
