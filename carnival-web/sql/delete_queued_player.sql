DELETE FROM
    queued_players
WHERE
    queue_id = ?
    AND user_id = ?;