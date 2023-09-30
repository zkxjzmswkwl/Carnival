SELECT
    users.username
FROM
    overwatch_match_players
    INNER JOIN users on overwatch_match_players.user_id = users.id
WHERE
    match_id = ?
    AND team_id = ?;