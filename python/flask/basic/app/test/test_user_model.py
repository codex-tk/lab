import unittest
import datetime

from app.main.model import db
from app.main.model.user import User
from app.test.base import BaseTestCase


class TestUserModel(BaseTestCase):

    def test_encode_auth_token(self):
        user = User(
            email='test@test.com',
            password='test',
            registered_on=datetime.datetime.utcnow()
        )
        db.session.add(user)
        db.session.commit()
        ret, auth_token = user.encode_auth_token(user.id, 'test_key')
        self.assertTrue(ret)

    def test_decode_auth_token(self):
        user = User(
            email='test0@test.com',
            password='test',
            registered_on=datetime.datetime.utcnow()
        )
        db.session.add(user)
        db.session.commit()
        ret, auth_token = user.encode_auth_token(user.id, 'test_key')
        self.assertTrue(ret)
        ret, user_id = user.decode_auth_token(auth_token, 'test_key')
        self.assertTrue(ret)
