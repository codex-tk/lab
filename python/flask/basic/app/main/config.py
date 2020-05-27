import os
import yaml


class Config(object):

    def __init__(self, yaml_file_name):
        with open(yaml_file_name) as f:
            rootcfg = yaml.load(f, Loader=yaml.FullLoader)
            appcfg = rootcfg['application']
            profiles = rootcfg['profiles']
            targetcfg = profiles[profiles['active']]

            self.flaskcfg = appcfg['flask']
            for key in appcfg:
                if key != 'flask':
                    setattr(self, key, appcfg[key])

            for key in targetcfg['flask']:
                self.flaskcfg[key] = targetcfg['flask'][key]


if __name__ == '__main__':
    cfg = Config('./../../config.yaml')