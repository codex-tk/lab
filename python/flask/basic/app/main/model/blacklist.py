from . import db
import datetime

class Blacklist(db.Model):

    __tablename__ = "blacklist"

    id = db.Column(db.Integer, primary_key=True, autoincrement=True)
    token = db.Column(db.String(500), unique=True, nullable=False)
    blacklisted_on = db.Column(db.DateTime, nullable=False)

    def __init__(self, token):
        self.token = token
        self.blacklisted_on = datetime.datetime.now()

    def __repr__(self):
        return '<Blacklist token : {}>'.format(self.token)

    @classmethod
    def check_blacklist(cls, token):
        res = Blacklist.query.filter_by(token=str(token)).first()
        return True if res else False

