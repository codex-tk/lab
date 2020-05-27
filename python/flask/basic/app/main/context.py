from app.main.config import Config

from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt


class Context:
    def __init__(self, config_yaml_file):
        self.config = Config(config_yaml_file)
        self.app = Flask(__name__.split('.')[0])
        for key in self.config.flaskcfg:
            self.app.config[key] = self.config.flaskcfg[key]
            
