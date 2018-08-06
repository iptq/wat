// +build bindata

package static

import "net/http"

func StaticFS() http.Handler {
	return http.FileServer(HTTP)
}
