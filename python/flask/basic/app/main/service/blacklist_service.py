from app.main.model import db
from app.main.model.blacklist import Blacklist

from app.main.util import make_response
import logging


def save_token(token):
    blacklist = Blacklist(token=token)
    try:
        db.session.add(blacklist)
        db.session.commit()
        return make_response(200, 'success', 'logout success')
    except Exception as e:
        logging.error(e)
        return make_response(500, 'fail', 'logout internal error')

