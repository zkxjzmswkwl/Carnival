#!/bin/bash
CYAN='\033[0;96m'
OFF='\033[0m'
printf "${CYAN}"
date
echo -e "**************************************************"
echo -e "> Creating db-carnival-dev.db and applying default schema"
sqlite3 db-carnival-dev.db ".read apply_schema.sql"
echo -e "> Creating test user with username \"EvilToaster\" and password \"orisa\""
sqlite3 db-carnival-dev.db ".read insert_test_user.sql"
echo -e "> Creating test queue"
sqlite3 db-carnival-dev.db ".read insert_test_queue.sql"
echo -e "**************************************************"
printf "${OFF}"
echo -e "> Done"

