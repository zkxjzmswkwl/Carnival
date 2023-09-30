select
    queued_players.role,
    users.username
FROM
    (
        queued_players
        INNER JOIN users ON queued_players.user_id = users.id
    )
WHERE
    queue_id = ?;