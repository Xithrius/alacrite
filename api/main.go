package main

import (
	"github.com/Xithrius/alacrite/api/database"
	"github.com/Xithrius/alacrite/api/routes"
)

func main() {
	db, err := database.NewDatabase()
	if err != nil {
		panic("Failed to set up connection to database")
	}
	db.RunTableAutoMigrations()

	r := routes.NewRoutesHandler("8080", db)

	r.AddRouterGroups()
	r.Run()
}
