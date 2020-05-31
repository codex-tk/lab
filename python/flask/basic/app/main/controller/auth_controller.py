from flask import request
from flask_restplus import Resource

from app.main.service.auth_service import login_user, logout_user
from app.main.namespaces import AuthNs

from app.main.util import make_response


@AuthNs.impl.route('/login')
class AuthLogin(Resource):

    @AuthNs.impl.doc('auth login')
    @AuthNs.impl.expect(AuthNs.auth, validate=True)
    def post(self):
        data = request.json
        return login_user(data['email'], data['password'])


@AuthNs.impl.route('/logout')
class AuthLogout(Resource):

    @AuthNs.impl.doc('auth logout')
    def post(self):
        print(request.headers)
        auth_token = request.headers.get('Authorization')
        return logout_user(auth_token)


