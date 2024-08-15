package routers

import (
	"github.com/gin-gonic/gin"

	"github.com/Xithrius/alacrite/api/controllers"
	"github.com/Xithrius/alacrite/api/database"
)

type AppRouter struct {
	// Router context and config
	Engine *gin.Engine
	Port   string

	// Database session
	DB *database.Database
}

func NewRoutesHandler(port string, db *database.Database) *AppRouter {
	r := gin.New()

	r.Use(gin.Logger())
	r.Use(gin.Recovery())

	trustedProxies := []string{"127.0.0.1"}
	r.SetTrustedProxies(trustedProxies)

	return &AppRouter{
		Engine: r,
		Port:   port,
		DB:     db,
	}
}

func (r *AppRouter) AddRouterGroups() {
	router_group := r.Engine.Group("/v1")

	AddHealthEndpoints(router_group)

	location_controller := controllers.NewLocationController(r.DB.Session)
	AddLocationEndpoints(router_group, location_controller)
}

func (r *AppRouter) Run() {
	r.Engine.Run(":" + r.Port)
}
