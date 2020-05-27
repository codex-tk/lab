from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt

from .config import Config

db = SQLAlchemy()
flask_bcrypt = Bcrypt()


def create_app(config_yaml):
    app = Flask(__name__)




