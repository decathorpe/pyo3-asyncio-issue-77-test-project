import asyncio
import random

import invalid_state_error


async def run(delay: float):
    coro = asyncio.wrap_future(invalid_state_error.sleep(0.1))
    await asyncio.sleep(delay)
    coro.cancel()


def main():
    while True:
        delay = random.uniform(0.099, 0.101)
        print(delay)
        asyncio.run(run(delay), debug=True)


if __name__ == "__main__":
    main()
