from app.main.model.user import User
from app.main.model.blacklist import Blacklist
from app.main.service.blacklist_service import save_token

import logging

from manage import app
from app.main.util import make_response


def login_user(email, password):
    try:
        user = User.query.filter_by(email=email).first()
        if not user:
            return make_response(401, 'fail', "can't find user")
        if not user.check_password(password):
            return make_response(401, 'fail', "invalid password")
        token = user.encode_auth_token(user.id, app.config['SECRET_KEY'])
        return make_response(200, 'success', "login success", Authorization=token.decode())
    except Exception as e:
        logging.error(e)
        return make_response(500, 'fail', 'internal error')


def logout_user(auth_token):
    is_blacklist = Blacklist.check_blacklist(auth_token)
    if is_blacklist:
        return make_response(401, 'fail', "Token Blacklisted, Please log in again")
    ret, token = User.decode_auth_token(auth_token, app.config['SECRET_KEY'])
    if not ret:
        return make_response(401, 'fail', token)

    return save_token(token=auth_token)
