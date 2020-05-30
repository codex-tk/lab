from app.main.model import db, bcrypt
from app.main.model.blacklist import Blacklist

import jwt
import datetime

from app.main.util import make_response


class User(db.Model):
    """ User Model for storing user related details """
    __tablename__ = "user"

    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    email = db.Column(db.String(255), unique=True, nullable=False)
    registered_on = db.Column(db.DateTime, nullable=False)
    admin = db.Column(db.Boolean, nullable=False, default=False)
    public_id = db.Column(db.String(100), unique=True)
    username = db.Column(db.String(50), unique=True)
    password_hash = db.Column(db.String(100))

    @property
    def password(self):
        raise AttributeError('password: write only field')

    @password.setter
    def password(self, password):
        self.password_hash = bcrypt.generate_password_hash(password).decode('utf-8')

    def check_password(self, password):
        return bcrypt.check_password_hash(self.password_hash, password)

    def __repr__(self):
        return "<User '{}'>".format(self.username)

    @classmethod
    def encode_auth_token(cls, user_id, key):
        try:
            payload = {
                'exp': datetime.datetime.utcnow() + datetime.timedelta(days=1, seconds=5),
                'iat': datetime.datetime.utcnow(),
                'sub': user_id
            }
            return True, jwt.encode(
                payload,
                key,
                algorithm='HS256'
            )
        except Exception as e:
            return False, e

    @classmethod
    def decode_auth_token(cls, token, key):
        try:
            payload = jwt.decode(token, key)
            return True, payload['sub']
        except jwt.ExpiredSignatureError:
            return False, 'Signature expired. Please log in again.'
        except jwt.InvalidTokenError:
            return False, 'Invalid token. Please log in again.'
