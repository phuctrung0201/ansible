import os
import json
from typing import Optional


def greet(name: str, greeting: Optional[str] = "Hello") -> str:
    return f"{greeting}, {name}!"


class Person:
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age

    def introduce(self) -> str:
        return f"I'm {self.name}, {self.age} years old."


person = Person("Alice", 30)
print(greet(person.name))
print(person.introduce())
