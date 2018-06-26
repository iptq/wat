package lib

import (
	"github.com/gorilla/mux"
)

func (app *App) Router() *mux.Router {
	router := mux.NewRouter()
	router.Use(app.logMiddleware)

	api := router.PathPrefix("/api/v1").Subrouter()
	api.Use(app.jsonMiddleware)

	users := api.PathPrefix("/users").Subrouter()
	users.HandleFunc("/register", app.HandleUserRegister).Methods("POST")

	protected := users.PathPrefix("/current").Subrouter()
	protected.HandleFunc("/heartbeats.bulk", app.HandleUserHeartbeat).Methods("POST")
	protected.Use(app.authMiddleware)

	return router
}
