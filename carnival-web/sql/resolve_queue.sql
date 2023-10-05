select
    users.username,
    users.id,
    users.rating,
    users.wins,
    users.losses,
    users.battletag,
    queued_players.role
FROM
    (
        queued_players
        INNER JOIN users ON queued_players.user_id = users.id
    )
WHERE
    queue_id = ?;
