SELECT
    *
FROM
    overwatch_match_players
WHERE
    match_id = ?
    AND team_id = ?;