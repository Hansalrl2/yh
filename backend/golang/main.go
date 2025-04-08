package main

import (
    "encoding/json"
    "fmt"
    "io/ioutil"
    "log"
    "net/http"
)

func toAnimeHandler(w http.ResponseWriter, r *http.Request) {
    if r.Method != http.MethodPost {
        http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
        return
    }

    var requestBody struct {
        ImageUrl string `json:"imageUrl"`
    }
    err := json.NewDecoder(r.Body).Decode(&requestBody)
    if err != nil {
        http.Error(w, "Invalid request body", http.StatusBadRequest)
        return
    }

    // Process the image (placeholder logic)
    resultImageUrl := fmt.Sprintf("https://example.com/processed/%s", requestBody.ImageUrl)

    response := map[string]string{"resultImageUrl": resultImageUrl}
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(response)
}

func main() {
    http.HandleFunc("/api/toanime", toAnimeHandler)
    log.Fatal(http.ListenAndServe(":3000", nil))
}
