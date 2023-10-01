Write-Host "ioasjdioasdjoiasjd"
#!/bin/bash
date
Write-Host "**************************************************"
Write-Host "> Creating db-carnival-dev.db and applying default schema"
sqlite3 db-carnival-dev.db ".read apply_schema.sql"
Write-Host "> Creating test user with username 'EvilToaster' and password 'orisa'"
sqlite3 db-carnival-dev.db ".read insert_test_user.sql"
Write-Host "> Creating test queue"
sqlite3 db-carnival-dev.db ".read insert_test_queue.sql"
Write-Host "**************************************************"
Write-Host "> Done"

