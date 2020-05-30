# app/__init__.py

from flask import Flask
from .main.config import Config
from .main.model import db, bcrypt

import os


def create_app(config_yaml_file):
    usercfg = Config(config_yaml_file)
    app = Flask(__name__)

    for key in usercfg.flask:
        app.config[key] = usercfg.flask[key]

    if usercfg.sqlite_uri_use_basedir and app.config['SQLALCHEMY_DATABASE_URI'].startswith('sqlite'):
        db_uri = app.config['SQLALCHEMY_DATABASE_URI']
        real_target = os.path.join(os.path.abspath(os.path.dirname(__file__)),
                                   '..',
                                   db_uri[db_uri.rindex('/') + 1:]).replace('\\', '/')
        app.config['SQLALCHEMY_DATABASE_URI'] = 'sqlite:///' + real_target

    setattr(app, 'usercfg', usercfg)

    db.init_app(app)
    bcrypt.init_app(app)

    return app
