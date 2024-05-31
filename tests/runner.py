#!/usr/bin/env python3

import os
import subprocess
from time import sleep

from playwright.sync_api import sync_playwright


class Runner:
    vm_url = "http://16.170.124.96:3001/"
    source_path: str
    source: str

    def __init__(self, source: str = "", source_path: str = ""):
        self.source_path = source_path
        self.source = source.replace("\n", " ")

    def parse(self) -> str:
        """Gets the pretty printed version of the AST of the program."""

        if not self.source and not self.source_path:
            raise ValueError("Either source or source_path must be provided.")

        if self.source and not self.source_path:
            self.source_path = "/tmp/program.parl"

            with open(self.source_path, "w") as f:
                f.write(self.source)

        sleep(1)

        compiler = subprocess.run(
            ["cargo", "run", "parse", self.source_path],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        sleep(1)

        if self.source and os.path.exists(self.source_path):
            os.remove(self.source_path)
            self.source_path = ""

        if compiler.returncode != 0:
            print(compiler.stderr.decode())
            # raise ValueError("Compilation failed.")

        return compiler.stdout.decode()

    def compile(self) -> str:
        """
        Compile the program.

        Returns the compiled program as a string.
        """

        if not self.source and not self.source_path:
            raise ValueError("Either source or source_path must be provided.")

        if self.source and not self.source_path:
            self.source_path = "/tmp/program.parl"

            with open(self.source_path, "w") as f:
                f.write(self.source)

            compiler = subprocess.run(
                ["cargo", "run", "compile", self.source_path],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )

            if os.path.exists(self.source_path):
                os.remove(self.source_path)

        elif self.source_path and not self.source:
            compiler = subprocess.run(
                ["cargo", "run", "compile", self.source_path],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
        else:
            raise ValueError("Only one of source or source_path must be provided.")

        if compiler.returncode != 0:
            print(compiler.stderr.decode())

        return compiler.stdout.decode()

    def compile_and_run(self) -> tuple[str, list[str]]:
        """
        Compile and run the program on the VM.

        Returns a list containing the lines outputted by the log area in the VM.
        """
        if not self.source and not self.source_path:
            raise ValueError("Either source or source_path must be provided.")

        parir = self.compile()

        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True)
            page = browser.new_page()
            page.goto(self.vm_url)
            element = page.query_selector("#pad_program")
            element.fill(parir)
            element = page.query_selector("#RunBtn")
            element.click()

            sleep(0.5)

            logs_area = page.query_selector("#logsarea")

            return parir, list(
                map(
                    lambda x: x.split("-- ")[1], logs_area.inner_text().splitlines()[1:]
                )
            )
