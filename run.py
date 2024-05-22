#!/usr/bin/env python3

import subprocess
from os import path
from time import sleep
from typing import Optional

from playwright.sync_api import sync_playwright


class Runner:
    vm_url = "http://16.170.124.96:3001/"
    source_path = ""

    def __init__(self, source_path: str):
        if not path.exists(source_path):
            print(f"File {source_path} does not exist")
            exit(1)

        self.source_path = source_path

    def compile(self) -> Optional[str]:
        """
        Compile the program.

        Returns the compiled program as a string.
        """
        compiler = subprocess.run(
            ["cargo", "run", "compile", self.source_path], stdout=subprocess.PIPE
        )

        if compiler.returncode != 0:
            print(compiler.stdout.decode())
            return None

        return compiler.stdout.decode()

    def compile_and_run(self) -> Optional[list[str]]:
        """
        Compile and run the program on the VM.

        Returns a list containing the lines outputted by the log area in the VM.
        """
        compiler = subprocess.run(
            ["cargo", "run", "compile", self.source_path], stdout=subprocess.PIPE
        )

        if compiler.returncode != 0:
            print(compiler.stdout.decode())
            return None

        program = compiler.stdout.decode()

        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True)
            page = browser.new_page()
            page.goto(self.vm_url)
            element = page.query_selector("#pad_program")
            element.fill(program)
            element = page.query_selector("#RunBtn")
            element.click()

            sleep(0.5)

            logs_area = page.query_selector("#logsarea")

            return list(
                map(
                    lambda x: x.split("-- ")[1], logs_area.inner_text().splitlines()[1:]
                )
            )
