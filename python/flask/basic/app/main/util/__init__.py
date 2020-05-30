# util/__init__.py


def make_response(status_code, status, message, **kwargs):
    res = {'status': status, 'message': message}
    for k, v in kwargs.items():
        res[k] = v
    return res, status_code
