import http from "k6/http";

export default function() {
    const url = 'http://localhost:8080';
    http.get(url);
}