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


def post_get_response(*, endpoint, json_payload):
    r = requests.post(
            f"http://localhost:3000/{endpoint}",
            json=json_payload,
            headers={"Content-Type": "application/json"})
    return Response(r.status_code, r.text)


class UserNotExist:
    def __init__(self):
        self.expected = Response(400, "User does not exist")

    def test(self) -> Response:
        payload = {
            "username": str(uuid.uuid1).replace("-", "")[0:10],
            "password": "123123123"
        }
        return post_get_response(endpoint="api/login",
                                 json_payload=payload)

    def satisfies_expectations(self, response) -> bool:
        return self.expected == response


class RegisterMismatchedPasswords:
    def __init__(self):
        self.expected = Response(400, "Passwords do not match")

    def test(self) -> Response:
        payload = {
            "username": "test_user",
            "password": "123123",
            "password_conf": "123123123"
        }
        return post_get_response(endpoint="api/register",
                                 json_payload=payload)

    def satisfies_expectations(self, response) -> bool:
        return self.expected == response


def test_users():
    # TODO: Make test base class that handles this boilerplate for us
    not_exist = UserNotExist()
    not_exist_resp = not_exist.test()

    if not not_exist.satisfies_expectations(not_exist_resp):
        print(f'''{Fore.RED}UserNotExist: Failed:
        \tExpected: {not_exist.expected.to_string()}
        \tReceived: {not_exist_resp.to_string()}{Style.RESET_ALL}''')
        return

    print(f"{Fore.GREEN}UserNotExist: Passed{Style.RESET_ALL}")

    reg_mismatched_pw = RegisterMismatchedPasswords()
    reg_mismatched_pw_resp = reg_mismatched_pw.test()

    if not reg_mismatched_pw.satisfies_expectations(reg_mismatched_pw_resp):
        print(f'''{Fore.RED}RegisterMismatchedPasswords: Failed:
        \tExpected: {reg_mismatched_pw.expected.to_string()}
        \tReceived: {reg_mismatched_pw_resp.to_string()}{Style.RESET_ALL}''')
        return

    print(f"{Fore.GREEN}RegisterMismatchedPasswords: Passed{Style.RESET_ALL}")


if __name__ == "__main__":
    test_users()
