# Useful curl request to test:
- create request
```
curl -d '{"title":"First Request", "description": "First request, requested from Alice send to Bob", "price": 1231231234, "requester_id":1, "helper_id": 2}' -H "Content-Type: application/json" -X POST http://localhost:8080/v1/requests
{"id":4,"title":"First Request","description":"First request, requested from Alice send to Bob","price":1231231234,"requesterId":1,"helperId":2}
```