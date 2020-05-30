from flask import request
from flask_restplus import Resource

from app.main.model.user import User
from app.main.namespaces import UserNs


@UserNs.impl.route('/')
class UserList(Resource):
    @UserNs.impl.doc('list of users')
    @UserNs.impl.marshal_list_with(UserNs.user, envelope='data')
    def get(self):
        return User.query.all()

'''
@user_ns.route('/sample')
class UserList(Resource):
    @user_ns.doc('list of samples')
    @user_ns.marshal_list_with(user_sample, envelope='data')
    def get(self):
        return User.query.all()
'''