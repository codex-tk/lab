import os
import unittest

from app.main import create_app
from flask_testing import TestCase
from manage import app

class TestSample(TestCase):

    def create_app(self):
        return create_app('./config.yaml')

    def test_sample(self):
        self.assertTrue(app.usercfg.version == 0.1)