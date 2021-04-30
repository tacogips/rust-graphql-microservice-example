#this docker file based on
#https://github.com/MartinKaburu/docker-postgresql-multiple-databases/blob/master/create-multiple-postgresql-databases.sh

FROM library/postgres:13-alpine

COPY create-multiple-dbs.sh /docker-entrypoint-initdb.d/

RUN chmod +x /docker-entrypoint-initdb.d/create-multiple-dbs.sh

EXPOSE 5432
