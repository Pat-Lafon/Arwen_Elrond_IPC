#!/usr/bin/env python3
# https://github.com/paulgb/interactive_process/blob/main/examples/echo_stream.py
def main():
    while True:
        line = input()
        if line == 'exit':
            break
        print(f"echo: {line}", flush=True)

if __name__ == '__main__':
    main()