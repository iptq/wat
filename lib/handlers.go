package lib

import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"time"

	"github.com/iptq/wat/lib/models"
	"github.com/thanhpk/randstr"
)

type heartbeatPost struct {
	Entity   string  `json:"entity"`
	Type     string  `json:"type"`
	Category string  `json:"category"`
	Time     float64 `json:"time"`

	Project  string `json:"project"`
	Branch   string `json:"branch"`
	Language string `json:"language"`

	Lines     int  `json:"lines"`
	LineNo    int  `json:"lineno"`
	CursorPos int  `json:"cursorpos"`
	IsWrite   bool `json:"is_write"`
}

type heartbeatRsp struct {
	ID     string  `json:"id"`
	Entity string  `json:"entity"`
	Type   string  `json:"type"`
	Time   float64 `json:"time"`
}

func (app *App) HandleUserHeartbeat(w http.ResponseWriter, r *http.Request) {
	// read the payload
	payload, err := ioutil.ReadAll(r.Body)
	if err != nil {
		return
	}

	// parse the payload
	var data []heartbeatPost
	err = json.Unmarshal(payload, &data)
	if err != nil {
		log.Println(string(payload))
		log.Println("parsing error", err)
		return
	}
	if len(data) < 1 {
		return
	}

	// construct Heartbeat record
	post := data[0]
	heartbeat := models.Heartbeat{
		Entity:   post.Entity,
		Type:     post.Type,
		Category: post.Category,
		Time:     time.Unix(int64(post.Time), 0),

		Project:  post.Project,
		Branch:   post.Branch,
		Language: post.Language,
	}

	// record it in the database
	doc, err := app.engine.Insert(&heartbeat)
	log.Println(doc, heartbeat.Id)
	if err != nil {
		log.Println("db write error", err)
		return
	}

	// write the response back
	payload, _ = json.Marshal(struct {
		Data heartbeatRsp `json:"data"`
	}{Data: heartbeatRsp{
		ID: string(heartbeat.Id),
	}})
	w.WriteHeader(http.StatusCreated)
	w.Write(payload)
}

func (app *App) HandleUserRegister(w http.ResponseWriter, r *http.Request) {
	var err error

	// for now, just generate a user and return the API key
	// note: what the fuck
	key := randstr.Hex(4) + "-" + randstr.Hex(2) + "-4" + randstr.Hex(2)[1:] + "-8" + randstr.Hex(2)[1:] + "-" + randstr.Hex(6)

	// insert user into the database
	user := models.User{
		APIKey: key,
	}
	_, err = app.engine.Insert(&user)
	if err != nil {
		log.Println("failed to create user")
		return
	}

	// write the result back to the user
	payload, _ := json.Marshal(struct {
		Key string
	}{
		Key: key,
	})
	w.Write(payload)
}
