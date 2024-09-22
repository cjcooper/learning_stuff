### to build project
```cargo build```

### expecting colima
```colima start --arch x86_64```

### start docker-compose
```UID=$UID GID=$GID docker-compose up```

### setup the database
```diesel setup```

### migrate the database
```diesel migration run```

### start the server
```cargo run```

### adding a book
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"title":"the title","primary_author":"me"}' \
  http://localhost:8080/api/books

### getting a book
curl http://localhost:8080/api/books/1
