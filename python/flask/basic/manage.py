import os
import unittest

from app.main import create_context, db
from app.main.context import Context

from flask_migrate import Migrate, MigrateCommand
from flask_script import Manager

from app.main.model.user import User

ctx = create_context('./config.yaml')

ctx.app.app_context().push()

manager = Manager(ctx.app)
migrate = Migrate(ctx.app, db)

manager.add_command('db', MigrateCommand)

@manager.command
def run():
    ctx.app.run()


@manager.command
def test():
    tests = unittest.TestLoader().discover('app/test', pattern='test*.py')
    result = unittest.TextTestRunner(verbosity=2).run(tests)
    if result.wasSuccessful():
        return 0
    return 1

if __name__ == '__main__':
    manager.run()