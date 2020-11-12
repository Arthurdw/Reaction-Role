# -*- coding: utf-8 -*-
"""
MIT License

Copyright (c) 2019-2020 Arthur

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
from datetime import datetime

from utilsx.console import Prettier, Colors

d = Colors.default.value

r = Colors.red.value
lr = Colors.light_red.value

b = Colors.blue.value
lb = Colors.light_blue.value

y = Colors.yellow.value
ly = Colors.light_yellow.value


# Handles most console messages.
class PrintHandler:
    def __init__(self, prettier: Prettier, log: bool):
        self.log = log
        self.prettier = prettier
        self.info_prefix = f"\b{b}[{lb}INFO{b}]{d} "
        self.warning_prefix = f"\b{y}[{ly}WARN{y}]{d} "
        self.fatal_prefix = f"\b{r}[{lr}FATAL{r}]{d} "

    def printf(self, message: str) -> None:
        """
        Format prints a message to the console.
        (date + message)

        :param message: The message that must be printed.
        """
        self.prettier.print(message + d, datetime.now())

    def info(self, message: str) -> None:
        """
        Sends a message with the INFO prefix.

        :param message: The message that must be printed.
        """
        if self.log:
            self.printf(self.info_prefix + message)

    def warn(self, message: str) -> None:
        """
        Sends a message with the WARN prefix.

        :param message: The message that must be printed.
        """
        if self.log:
            self.printf(self.warning_prefix + message)

    def fatal(self, message: str) -> None:
        """
        Sends a message with the FATAL prefix.

        :param message: The message that must be printed.
        """
        if self.log:
            self.printf(self.fatal_prefix + message)
