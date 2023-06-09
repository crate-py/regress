from __future__ import annotations

from typing import Iterable

class Regex:
    def __init__(self, pattern: str): ...
    def find(self, text: str) -> Match | None: ...
    def find_iter(self, text: str) -> Iterable[Match] | None: ...

class Match:
    def group(self, idx: int) -> slice | None: ...
    def named_group(self, name: str) -> slice | None: ...
    def range(self) -> slice: ...
