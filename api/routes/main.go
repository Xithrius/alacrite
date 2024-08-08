package routes

import (
	"github.com/gin-gonic/gin"
)

var router = gin.Default()

func Run() {
	getRoutes()
	router.Run(":8080")
}

func getRoutes() {
	router_group := router.Group("/v1")
	addUserRoutes(router_group)
	addPingRoutes(router_group)
}
