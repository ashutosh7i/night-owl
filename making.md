create rust image
define to install sqlite in it
copy code, 
create volume
in code main, define logic to createa talbe,
creates volume in main uses its to store the db.
write code to compose
push

deploy and run on local.

docker volume create sqlite_volume
docker run -it -v sqlite_volume:/sqlite rust /bin/bash
apt update && apt upgrade -y
apt install sqlite3 -y
sqlite3 --version
sqlite3 /sqlite/nightowl_db.db

dev server-
npx nodemon --exec "cargo run" ./src/main.rs
