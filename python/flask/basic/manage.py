import os
import unittest
import logging
import logging.config
import yaml

from flask_migrate import Migrate, MigrateCommand
from flask_script import Manager

from app.main.model.user import User
from app.main.model.blacklist import Blacklist

from app import create_app
from app.main.model import db
from app.main import blueprint

app = create_app('./config.yaml')
app.register_blueprint(blueprint)
app.app_context().push()

manager = Manager(app)
migrate = Migrate(app, db)

manager.add_command('db', MigrateCommand)


def init_logging(yaml_file_name):
    with open(yaml_file_name) as f:
        loggingcfg = yaml.load(f, Loader=yaml.FullLoader)
        logging.config.dictConfig(loggingcfg)
    '''
    logger = logging.getLogger('lab')
    logger.setLevel(logging.DEBUG)

    formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
    stream_handler = logging.StreamHandler()
    stream_handler.setFormatter(formatter)
    logger.addHandler(stream_handler)
    '''
    logging.info("logger init!")

@manager.command
def run():
    init_logging('./logging.yaml')
    logging.info(os.getcwd())
    app.run()


@manager.command
def test():
    tests = unittest.TestLoader().discover('app/test', pattern='test*.py')
    result = unittest.TextTestRunner(verbosity=2).run(tests)
    if result.wasSuccessful():
        return 0
    return 1


if __name__ == '__main__':
    manager.run()