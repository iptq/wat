package handlers

import (
	"encoding/json"
	"net/http"
)

func PostHeartbeat(w http.ResponseWriter, r *http.Request) {
	payload, _ := json.Marshal("shiet")
	w.Write(payload)
}
