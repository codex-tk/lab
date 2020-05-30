# main/__init__.py

from flask_restplus import Api
from flask import Blueprint
from .namespaces import UserNs
from .controller import user_controller


blueprint = Blueprint('api', __name__)
api = Api(blueprint,
          title='flask restplus boilerpalte',
          version='0.1',
          description='flask restplus')
api.add_namespace(UserNs.impl, path='/user')

