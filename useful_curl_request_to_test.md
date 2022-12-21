# Useful curl request to test:
- create request
```
curl -d '{"title":"First Request", "description": "First request, requested from Alice send to Bob", "price": 1231231234, "requester_id":1, "helper_id": 2}' -H "Content-Type: application/json" -X POST http://localhost:8080/v1/requests
{"id":4,"title":"First Request","description":"First request, requested from Alice send to Bob","price":1231231234,"requesterId":1,"helperId":2}
```
- list requests
```
curl -X GET http://localhost:8080/v1/requests
```
- binding request
```
curl -d '{"helper_id": 2}' -H "Content-Type: application/json" -X PATCH http://localhost:8080/v1/requests/2/binding
```
- listing helpers
```
curl http://localhost:8080/v1/helpers
```
- mark request as done
```
curl -H "Content-Type: application/json" -X POST http://localhost:8080/v1/requests/1/done -v
```