docker run -it --rm --name sql-cookbook \
-e POSTGRES_DB=sql-cookbook \
-e POSTGRES_PASSWORD=sql-cookbook \
-e POSTGRES_USER=sql-cookbook \
-p 5432:5432 \
-v "$(realpath import)":/tmp/import \
postgres:11.2
