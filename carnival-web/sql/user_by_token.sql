SELECT
    users.id,
    users.username,
    users.battletag,
    users.password,
    users.role
FROM
    session_tokens
    INNER JOIN users ON session_tokens.for_user = users.id
WHERE
    token = ?;
