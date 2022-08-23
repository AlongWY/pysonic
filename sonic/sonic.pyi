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
    def count(self, collection, bucket, object):
        """
        Count indexed search data of your collection.
        """
        pass
    def flush(self, collection, bucket, object):
        """
        Flush all indexed data from collections.
        """
        pass
    def ping(self):
        """
        Ping server.
        """
        pass
    def pop(self, collection, bucket, object, text):
        """
        Pop search data from the index. Returns removed words count as usize type.
        """
        pass
    def push(self, collection, bucket, object, text, lang):
        """
        Push search data in the index.
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
    def query(self, collection, bucket, terms, lang, limit, offset):
        """
        Query objects in database.
        """
        pass
    def quit(self):
        """
        Stop connection.
        """
        pass
    def suggest(self, collection, bucket, word, limit):
        """
        Suggest auto-completes words.
        """
        pass
