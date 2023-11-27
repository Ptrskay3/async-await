# Run with: `python3 -m async.py`
import requests
import asyncio

def fry_egg(name):
    return requests.get(f'http://localhost:3001/{name}').text

async def fry_egg_async(name):
    return await asyncio.to_thread(fry_egg, name)

async def main():
    eggs = ['A', 'B', 'C', 'D']
    responses = await asyncio.gather(*[fry_egg_async(egg) for egg in eggs])
    print(responses)


asyncio.run(main())
