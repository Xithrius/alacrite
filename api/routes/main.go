package routes

import (
	"github.com/gin-gonic/gin"

	"github.com/Xithrius/alacrite/api/database"
)

type Routes struct {
	// Router context and config
	Engine *gin.Engine
	Port   string

	// Database session
	DBSession *database.Database
}

func NewRoutesHandler(port string, db *database.Database) *Routes {
	r := gin.New()

	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	trustedProxies := []string{"127.0.0.1"}
	r.SetTrustedProxies(trustedProxies)

	return &Routes{
		Engine:    r,
		Port:      port,
		DBSession: db,
	}
}

func (r *Routes) AddRouterGroups() {
	router_group := r.Engine.Group("/v1")

	r.addUserRoutes(router_group)
	r.addPingRoutes(router_group)
}

func (r *Routes) Run() {
	r.Engine.Run(":" + r.Port)
}
