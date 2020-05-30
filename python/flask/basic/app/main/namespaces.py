from flask_restplus import Namespace, fields


class UserNs:
    impl = Namespace('user', description='user namespace')
    user = impl.model('user', {
        'email': fields.String(required=True, description='user email address'),
        'username': fields.String(required=True, description='user username'),
        'password': fields.String(required=True, description='user password'),
        'public_id': fields.String(description='user Identifier')
    })
    '''
    sample = impl.model('sample', {
        'sample': fields.String(required=True, description='sample')
    })
    '''
