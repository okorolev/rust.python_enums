from typing import Dict


class Value:
    name: str
    value: object

    def __new__(cls, name, value) -> Value: ...
