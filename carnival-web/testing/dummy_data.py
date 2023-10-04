import requests

DUMMY_USERS = [
    {
        "username": "DummyTank1",
        "role": "Tank",
        "battletag": "DummyTank1#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummyTank2",
        "role": "Tank",
        "battletag": "DummyTank2#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummyDPS1",
        "role": "DPS",
        "battletag": "DummyDPS1#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummyDPS2",
        "role": "DPS",
        "battletag": "DummyDPS2#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummyDPS3",
        "role": "DPS",
        "battletag": "DummyDPS3#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummyDPS4",
        "role": "DPS",
        "battletag": "DummyDPS4#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummySupport1",
        "role": "Support",
        "battletag": "DummySuppor#123t1",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummySupport2",
        "role": "Support",
        "battletag": "DummySupport2#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummySupport3",
        "role": "Support",
        "battletag": "DummySupport3#123",
        "password": "123",
        "password_conf": "123",
    },
    {
        "username": "DummySupport4",
        "role": "Support",
        "battletag": "DummySupport4#123",
        "password": "123",
        "password_conf": "123",
    },
]


def post_get_response(*, endpoint, json_payload, cookies=None):
    r = requests.post(
        f"http://localhost:3000/{endpoint}",
        json=json_payload,
        headers={"Content-Type": "application/json"},
        cookies=cookies
    )
    return r

def join_queue(user):
    session = requests.Session()
    r = session.post(
            "http://localhost:3000/api/login",
            json={"username": user.get("username"), "password": user.get("password")})

    print(r.status_code)
    if r.status_code != 200:
        print(f"Could not login as {user.get('username')} with password {user.get('password')}. Did you run this script once to create users?")
        print(r.text)
        return

    q = post_get_response(endpoint="api/join_queue", json_payload={"queue_id": "1"}, cookies=r.cookies.get_dict())
    print(q.text)

if __name__ == "__main__":
    should_create_users = input("Should users be created (y/n): ")
    should_join_queue = input("Should populate queue? This requires users to have previously been created (y/n): ")

    if should_create_users == 'y':
        for user in DUMMY_USERS:
            resp = post_get_response(endpoint="api/register", json_payload=user)
            if resp.status_code == 201:
                print(f"{user.get('username')} created.")

    if should_join_queue == 'y':
        print('asd')
        for user in DUMMY_USERS:
            join_queue(user)
