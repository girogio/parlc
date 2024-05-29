#!/usr/bin/env python3

from runner import Runner
from sys import stdin

program = ""
for line in stdin:
    program += line
runner = Runner(source=program)
parir, results = runner.compile_and_run()
print(results)
