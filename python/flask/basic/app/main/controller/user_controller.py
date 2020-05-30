from flask import request
from flask_restplus import Resource, fields

from app.main.model.user import User
from app.main.namespaces import UserNs
from app.main.service.user_service import save_new_user, get_all_users, get_a_user

from app.main.util import make_response


@UserNs.impl.route('/')
class UserList(Resource):
    res_users = UserNs.impl.model('user_list', {
        'status': fields.String(required=True, description='status'),
        'message': fields.String(required=True, description='status'),
        'data': fields.List(fields.Nested(UserNs.user))
    })

    @UserNs.impl.doc('list of users')
    @UserNs.impl.marshal_list_with(res_users)
    def get(self):
        return {'status': 'success', 'message': 'list', 'data': get_all_users()}

    @UserNs.impl.response(201, 'User successfully created')
    @UserNs.impl.doc('create new user')
    @UserNs.impl.expect(UserNs.user, validate=True)
    def post(self):
        data = request.json
        print(data)
        if save_new_user(data):
            return make_response(201, 'success', 'Successfully registered.')
        return make_response(409, 'fail', 'User already exists. Please Log in.')


@UserNs.impl.route('/<public_id>')
@UserNs.impl.param('public_id', 'User Identifier')
class User(Resource):
    res_userinfo = UserNs.impl.model('user_list', {
        'status': fields.String(required=True, description='status'),
        'message': fields.String(required=True, description='status'),
        'data': fields.Nested(UserNs.userinfo)
    })

    @UserNs.impl.doc('get a user')
    @UserNs.impl.marshal_with(res_userinfo)
    def get(self, public_id):
        user = get_a_user(public_id)
        if not user:
            UserNs.impl.abort(404, 'cant find user', status='fail')
        else:
            return {'status': 'success', 'message': 'list', 'data': user}
