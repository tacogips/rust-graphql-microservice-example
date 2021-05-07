### usage

```bash

# invoke db and init it
docker-compose up -d db
make mig-db

# invoke services along with graphql server
docker-compose up -d

```

#### graphiql
`http://localhost:5010`


### Progress

- [x] implement graphql Interface(mock)
- [x] Article service
- [x] Comment service
- [x] User service
- [x] connect the graphql to the each services.
- [ ] impl frontend
- [ ] impl mutations
- [ ] SSO compatible auth server
- [ ] Event processing with message queue(add comment)
