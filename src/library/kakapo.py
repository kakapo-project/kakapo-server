

import sqlalchemy.engine
import sqlalchemy.create_engine

import pandas as pd

class Entity:
    pass

class Script(Entity):
    def __init__(self):
        self.name = None

class ScriptList:
    @property
    def me(self):
        return

    def insert(self, name, on_duplicate='update'):
        raise NotImplementedError('management features not currently available')

    def __getitem__(self, name):
        return

    def __iter__(self):
        return self

    def __next__(self):
        return

class Query(Entity):
    def __init__(self):
        self.name = None

class QueryList:
    def insert(self, name, on_duplicate='update'):
        raise NotImplementedError('management features not currently available')

    def __getitem__(self, name):
        return

    def __iter__(self):
        return self

    def __next__(self):
        return


class TableData(Entity):
    def __init__(self):
        self.name = None

    """ Returns the pandas dataframe
    """
    def df():
        pass

class TableDataList:
    def insert(self, name):
        raise NotImplementedError('management features not currently available')

    def __getitem__(self, name):
        return

    def __iter__(self):
        return self

    def __next__(self):
        return


class DatabaseInfo:
    def __init__(self, username, password, host, port, database, driver='postgresql'):
        self.username = username
        self.password = password
        self.host = host
        self.port = port
        self.database = database

        url = engine.url.URL(driver, username, password, host, port, database)
        self.engine = create_engine(url)

class Environment:

    def __init__(self, database_info, script_name):
        if isinstance(database_info, DatabaseInfo):
            self.database_info = database_info
        else:
            self.database_info = DatabaseInfo(
                database_info['username'],
                database_info['password'],
                database_info['host'],
                database_info['port'],
                database_info['database'],
            )
        self.script_name = script_name

    """
    Get sqlalchemy connection

    Usage::

        with env.connect() as conn:
            rs = conn.execute("SELECT * FROM stuff")
            for row in rs:
                print(row)

    """
    @property
    def connect(self):
        return self.database_info.engine.connect()

    """ Get all accessible scripts """
    @property
    def scripts(self):
        return []

    """ Get all accessible tables """
    @property
    def tables(self):
        return []

    """ Get all accessible scripts """
    @property
    def scripts(self):
        return []



def runner(script, json_config):
    handler = script.handler
    config = json.loads(json_config)

    env = Environment(config, config['script_name'])
    handler(env)

""" playground:

current_script = env.scripts.me
current_script(message1='hi') # calls recursively

for script in scripts: # can iterate
    print(script.name)

table = tables['my_special_table'] # can index
data = table.get() # equivalent to calling {{url}}/api/tables/my_special_table

"""