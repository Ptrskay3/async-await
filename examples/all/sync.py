import requests

def get_user(name):
    return requests.get(f'http://localhost:3001/{name}').text


for user in ['A', 'B', 'C', 'D']:
    print(get_user(user))

