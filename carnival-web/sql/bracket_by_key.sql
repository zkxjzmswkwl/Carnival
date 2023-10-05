select
    brackets.id,
    brackets.queue_id
FROM
    (
        bracket_keys
        INNER JOIN brackets ON bracket_keys.bracket_id = brackets.id
    )
WHERE
    key = ?;
