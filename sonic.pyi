from typing import List

__version__: str


class SearchChannel(object):
    @staticmethod
    def __new__(self, addr: str, password: str):
        pass

    def __init__(self, addr: str, password: str):
        pass

    def ping(self) -> bool:
        pass

    def quit(self) -> bool:
        pass

    def query(self, collection: str, bucket: str, terms: str) -> List[str]:
        pass

    def query_with_limit(self, collection: str, bucket: str, terms: str, limit: int) -> List[str]:
        pass

    def query_with_limit_and_offset(self, collection: str, bucket: str, terms: str, limit: int, offset: int) -> List[
        str]:
        pass

    def suggest(self, collection: str, bucket: str, terms: str) -> List[str]:
        pass

    def suggest_with_limit(self, collection: str, bucket: str, terms: str, limit: int) -> List[str]:
        pass


class IngestChannel(object):
    @staticmethod
    def __new__(self, addr: str, password: str):
        pass

    def __init__(self, addr: str, password: str):
        pass

    def ping(self) -> bool:
        pass

    def quit(self) -> bool:
        pass

    def bucket_count(self, collection: str) -> int:
        pass

    def object_count(self, collection: str, bucket: str) -> int:
        pass

    def word_count(self, collection: str, bucket: str, object: str) -> int:
        pass

    def flushc(self, collection: str) -> int:
        pass

    def flushb(self, collection: str, bucket: str) -> int:
        pass

    def flusho(self, collection: str, bucket: str, object: str) -> int:
        pass

    def push(self, collection: str, bucket: str, object: str, text: str) -> bool:
        pass

    def push_with_locale(self, collection: str, bucket: str, object: str, text: str, local: str) -> bool:
        pass

    def pop(self, collection: str, bucket: str, object: str, text: str, ) -> int:
        pass


class ControlChannel(object):
    @staticmethod
    def __new__(self, addr: str, password: str):
        pass

    def __init__(self, addr: str, password: str):
        pass

    def ping(self) -> bool:
        pass

    def quit(self) -> bool:
        pass

    def consolidate(self) -> bool:
        pass

    def backup(self, action: str) -> bool:
        pass

    def restore(self, action: str) -> bool:
        pass
