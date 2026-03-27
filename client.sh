CLIENT_ID=THE_CLIENT_ID
REALM=master
ISSUER_URL=http://localhost:8080
USER=aliefhooghe
PASSWORD=1234

TOKEN=$(curl -s -X POST $ISSUER_URL/realms/$REALM/protocol/openid-connect/token \
  -d "client_id=$CLIENT_ID&grant_type=password&username=$USER&password=$PASSWORD" | jq -r ".access_token")

curl -X "GET" \
  "http://localhost:5000/auth/me" \
  -H "Authorization: Bearer $TOKEN" \
  -H "accept: application/json" | jq
