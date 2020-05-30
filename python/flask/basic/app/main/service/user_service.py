import uuid
import datetime

from app.main.model import db
from app.main.model.user import User


def save_changes(data):
    db.session.add(data)
    db.session.commit()


def get_all_users():
    return User.query.all()


def get_a_user(public_id):
    return User.query.filter_by(public_id=public_id).first()


def save_new_user(data):
    user = User.query.filter_by(
        email=data['email']
    ).first()
    if not user:
        nuser = User(
            public_id=str(uuid.uuid4()),
            email=data['email'],
            username=data['username'],
            password=data['password'],
            registered_on=datetime.datetime.utcnow()
        )
        save_changes(nuser)
        return True
    return False
