SELECT
    users.id,
    users.username,
    users.battletag,
    users.password,
    users.role,
    users.rating,
    users.wins,
    users.losses,
    users.email
FROM
    session_tokens
    INNER JOIN users ON session_tokens.for_user = users.id
WHERE
    token = ?;
