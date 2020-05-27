from flask import Flask
from flask_sqlalchemy import SQLAlchemy
from flask_bcrypt import Bcrypt
from flask_sqlalchemy import SQLAlchemy

from .context import Context

db = SQLAlchemy()
bcrypt = Bcrypt()


def create_context(config_yaml_file):
    ctx = Context(config_yaml_file)
    db.init_app(ctx.app)
    bcrypt.init_app(ctx.app)
    return ctx




