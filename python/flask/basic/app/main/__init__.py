from flask import Flask

from .config import Config
from .model import db, bcrypt


def create_app(config_yaml_file):
    usercfg = Config(config_yaml_file)
    app = Flask(__name__)

    for key in usercfg.flask:
        app.config[key] = usercfg.flask[key]

    setattr(app, 'usercfg', usercfg)

    db.init_app(app)
    bcrypt.init_app(app)
    
    return app


