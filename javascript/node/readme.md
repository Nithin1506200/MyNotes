```sh
openssl genrsa -out key.pem 2048
openssl req -new -key key.pem -out client.csr
openssl x509 -req -in client.csr -signkey key.pem -out cert.pem
```
