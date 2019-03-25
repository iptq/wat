package wat

import (
	"net/http"

	"github.com/gorilla/mux"
	"github.com/iptq/wat/static"
)

func (app *App) createRouter() *mux.Router {
	router := mux.NewRouter()
	router.Use(app.logMiddleware)

	api := router.PathPrefix("/api").Subrouter()
	api.Use(app.jsonMiddleware)

	api.HandleFunc("/config", app.publicConfig).Methods("GET")

	users := api.PathPrefix("/users").Subrouter()
	if app.config.RegistrationEnabled {
		users.HandleFunc("/register", app.handleUserRegister).Methods("POST")
	} else {
		users.HandleFunc("/register", app.error403).Methods("POST")
	}

	protected := users.PathPrefix("/current").Subrouter()
	protected.HandleFunc("/heartbeats.bulk", app.handleUserHeartbeat).Methods("POST")
	protected.Use(app.authMiddleware)

	fs := static.StaticFS()
	router.Handle("/", http.FileServer(fs)).Methods("GET")

	// cache index.html in memory cuz it's small
	file, err := fs.Open("index.html")
	if err != nil {
		panic(err)
	}
	indexPage := make([]byte, 1024)
	_, err = file.Read(indexPage)
	if err != nil {
		panic(err)
	}

	// redirect everything else to index.html cuz SPA
	/* router.NotFoundHandler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		w.WriteHeader(http.StatusOK)
		w.Write(indexPage)
	}) */

	return router
}
