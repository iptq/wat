package wat

import (
	"encoding/base64"
	"log"
	"net/http"
	"regexp"
	"strings"

	"github.com/iptq/wat/lib/models"
)

var apiKeyPattern = regexp.MustCompile(`[0-9a-f]{12}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{8}`)

func validateApiKey(key string) bool {
	return apiKeyPattern.Match([]byte(key))
}

func (app *App) authMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var user models.User
		var err error

		auth := r.Header.Get("Authorization")
		parts := strings.Split(auth, " ")
		if len(parts) < 2 {
			log.Println("parts:", parts)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		decode, _ := base64.StdEncoding.DecodeString(parts[1])
		apiKey := string(decode)
		if !validateApiKey(apiKey) {
			// TODO: try backup, ?api_key= query string
			return
		}

		has, err := app.engine.Where("api_key = ?", apiKey).Get(&user)
		if err != nil {
			log.Println("error:", err)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		if !has {
			log.Println("api key not found:", apiKey)
			w.WriteHeader(http.StatusForbidden)
			w.Write([]byte(""))
			return
		}
		log.Println("auth ok")
		next.ServeHTTP(w, r)
	})
}

func (app *App) jsonMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Header().Set("Access-Control-Allow-Origin", "*")
		next.ServeHTTP(w, r)
	})
}

func (app *App) logMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Println(r)
		next.ServeHTTP(w, r)
	})
}
