import requests
import asyncio

def get_user(name):
    return requests.get(f'http://localhost:3001/{name}').text

async def get_user_async(name):
    return await asyncio.to_thread(get_user, name)

async def main():
    users = ['A', 'B', 'C', 'D']
    responses = await asyncio.gather(*[get_user_async(user) for user in users])
    print(responses)


asyncio.run(main())
