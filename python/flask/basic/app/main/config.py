import os
import yaml


class Config(object):

    def __init__(self, yaml_file_name):
        with open(yaml_file_name) as f:
            rootcfg = yaml.load(f, Loader=yaml.FullLoader)

            profiles = rootcfg['profiles']
            commoncfg = profiles['common']

            for key in commoncfg:
                setattr(self, key, commoncfg[key])

            targetcfg = profiles[profiles['active']]

            for key in targetcfg:
                if isinstance(targetcfg[key], dict):
                    if hasattr(self, key):
                        targetdict = getattr(self, key)
                        for subkey in targetcfg[key]:
                            targetdict[subkey] = targetcfg[key][subkey]
                    else:
                        setattr(self, key, targetcfg[key])
                else:
                    setattr(self, key, targetcfg[key])


if __name__ == '__main__':
    cfg = Config('./config.yaml')
    #print(cfg.__dict__)
    