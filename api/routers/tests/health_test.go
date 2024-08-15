package routers_test

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/stretchr/testify/assert"

	"github.com/Xithrius/alacrite/api/routers"
)

func SetupTestRouter() *gin.Engine {
	r := gin.Default()

	router_group := r.Group("/v1")

	routers.AddHealthEndpoints(router_group)

	return r
}

func TestPingRoute(t *testing.T) {
	router := SetupTestRouter()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/v1/ping", nil)
	router.ServeHTTP(w, req)

	assert.Equal(t, 200, w.Code)
	assert.Equal(t, "pong", w.Body.String())
}

func TestHealthRoute(t *testing.T) {
	router := SetupTestRouter()

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/v1/health", nil)
	router.ServeHTTP(w, req)

	assert.Equal(t, http.StatusOK, w.Code)

	expectedResponseBody := `{"state":"healthy"}`
	assert.JSONEq(t, expectedResponseBody, w.Body.String())
}
