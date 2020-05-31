from flask_testing import TestCase
from app.main.model import db
from manage import app
from app.main.model.user import User
from app.main.model.blacklist import Blacklist


class BaseTestCase(TestCase):

    def create_app(self):
        return app

    def setUp(self) -> None:
        pass
        #db.create_all()
        #db.session.commit()

    def tearDown(self) -> None:
        User.query.delete()
        Blacklist.query.delete()
    '''
    def setUp(self) -> None:
        db.create_all()
        db.session.commit()

    def tearDown(self) -> None:
        db.session.remove()
        db.drop_all()
    '''

