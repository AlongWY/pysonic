# Generated content DO NOT EDIT
class ControlChannel:
    def __init__(self, addr, passwd):
        pass
    def backup(self, action):
        """
        Backup KV + FST to /<BACKUP_{KV/FST}_PATH> See [sonic backend source code](https://github.com/valeriansaliou/sonic/blob/master/src/channel/command.rs#L808) for more information.
        """
        pass
    def consolidate(self):
        """
        Consolidate indexed search data instead of waiting for the next automated consolidation tick.
        """
        pass
    def ping(self):
        """
        Ping server.
        """
        pass
    def quit(self):
        """
        Stop connection.
        """
        pass
    def restore(self, action):
        """
        Restore KV + FST from if you already have backup with the same name.
        """
        pass

class IngestChannel:
    def __init__(self, addr, passwd):
        pass
    def count(self, collection, bucket=None, object=None):
        """
        Count indexed search data of your collection.
        """
        pass
    def flush(self, collection, bucket=None, object=None):
        """
        Flush all indexed data from collections.
        """
        pass
    def ping(self):
        """
        Ping server.
        """
        pass
    def pop(self, collection, *args, **kwargs):
        """
        Pop search data from the index. Returns removed words count as usize type.
        The function can be called in two ways:
            `pop(self, collection, bucket, object, text)`
            `pop(self, collection, object, text)`
        where `bucket` is optional.
        """
        pass
    def push(self, collection, *args, lang=None, **kwargs):
        """
        Push search data in the index.
        The function can be called in two ways:
            `push(self, collection, bucket, object, text, lang=None)`
            `push(self, collection, object, text, lang=None)`
        where `bucket` is optional.
        """
        pass
    def quit(self):
        """
        Stop connection.
        """
        pass

class SearchChannel:
    def __init__(self, addr, passwd):
        pass
    def ping(self):
        """
        Ping server.
        """
        pass
    def query(self, collection, *args, lang=None, limit=None, offset=None, **kwargs):
        """
        Query objects in database.
        The function can be called in two ways:
            `query(self, collection, bucket, terms, lang=None, limit=None, offset=None)`
            `query(self, collection, terms, lang=None, limit=None, offset=None)`
        where `bucket` is optional.
        """
        pass
    def quit(self):
        """
        Stop connection.
        """
        pass
    def suggest(self, collection, *args, limit=None, **kwargs):
        """
        Suggest auto-completes words.
        The function can be called in two ways:
            `suggest(self, collection, bucket, word, limit=None)`
            `suggest(self, collection, word, limit=None)`
        where `bucket` is optional.
        """
        pass
