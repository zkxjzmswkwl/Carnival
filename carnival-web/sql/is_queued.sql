select
    users.username
from
    queued_players
inner join users on queued_players.user_id = users.id
where
    users.username = ?;