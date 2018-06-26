package lib

import "github.com/gorilla/mux"

func Router() *mux.Router {
	router := mux.NewRouter()
	protected := router.PathPrefix("/users").Subrouter()
	protected.Use(authMiddleware)

	return router
}
