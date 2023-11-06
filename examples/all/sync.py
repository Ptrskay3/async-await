import requests

def fry_egg(name):
    return requests.get(f'http://localhost:3001/{name}').text

for egg in ['A', 'B', 'C', 'D']:
    print(fry_egg(egg))

