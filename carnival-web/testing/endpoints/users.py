from dataclasses import dataclass
import requests
import uuid
from colorama import Fore, Style, init as colorama_init

colorama_init()


@dataclass
class Response:
    status_code: int
    response_body: str

    def to_string(self):
        return f"{self.status_code}: {self.response_body}"

    def print(self):
        print(self.to_string())


class Test:
    def __init__(self, name, expected):
        self.name = name
        self.expected = expected

    def check(self, response) -> bool:
        self.actual = response

        if self.expected.response_body is None:
            self.expected.response_body = "N/A: status_code only check"
            if self.expected.status_code == response.status_code:
                return self.success()
            return self.fail()

        if self.actual == self.expected:
            return self.success()
        return self.fail()

    def fail(self) -> bool:
        print(f'''{Fore.RED}{self.name}: Failed:
        \tExpected: {self.expected.to_string()}
        \tReceived: {self.actual.to_string()}{Style.RESET_ALL}''')
        return False

    def success(self):
        print(f'''{Fore.GREEN}{self.name}: Passed:
        \tExpected: {self.expected.to_string()}
        \tReceived: {self.actual.to_string()}{Style.RESET_ALL}''')
        return True


def post_get_response(*, endpoint, json_payload):
    r = requests.post(
            f"http://localhost:3000/{endpoint}",
            json=json_payload,
            headers={"Content-Type": "application/json"})
    return Response(r.status_code, r.text)


class UserNotExist(Test):
    def __init__(self):
        super().__init__("UserNotExist", Response(400, "User does not exist"))

    def test(self) -> Response:
        payload = {
            "username": str(uuid.uuid1).replace("-", "")[0:10],
            "password": "123123123"
        }
        return post_get_response(endpoint="api/login",
                                 json_payload=payload)


class RegisterMismatchedPasswords(Test):
    def __init__(self):
        super().__init__("RegisterMismatchedPasswords",
                         Response(400, "Passwords do not match"))

    def test(self) -> Response:
        payload = {
            "username": "1realuserwow",
            "password": "123123",
            "password_conf": "123123",
            "battletag": "riealuser#12333"
        }
        return post_get_response(endpoint="api/register",
                                 json_payload=payload)


class BattletagExists(Test):
    def __init__(self):
        super().__init__("BattletagExists",
                         Response(400, "Battletag already exists"))

    def test(self) -> Response:
        payload = {
            "username": "cartertest",
            "password": "123123",
            "password_conf": "123123",
            "battletag": "Fuey500#123"
        }
        return post_get_response(endpoint="api/register",
                                 json_payload=payload)


class Login(Test):
    def __init__(self):
        super().__init__("Login", Response(200, None))

    def test(self) -> Response:
        payload = {
            "username": "cartertest",
            "password": "123123",
        }
        return post_get_response(endpoint="api/login", json_payload=payload)


def test_users():
    login = Login()
    login_resp = login.test()
    login.check(login_resp)

    not_exist = UserNotExist()
    not_exist_resp = not_exist.test()
    not_exist.check(not_exist_resp)

    reg_mismatched_pw = RegisterMismatchedPasswords()
    reg_mismatched_pw_resp = reg_mismatched_pw.test()
    reg_mismatched_pw.check(reg_mismatched_pw_resp)

    battletag_exists = BattletagExists()
    battletag_exists_resp = battletag_exists.test()
    battletag_exists.check(battletag_exists_resp)


if __name__ == "__main__":
    test_users()
