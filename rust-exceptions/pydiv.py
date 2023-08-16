#!/usr/bin/env python3
import librust_exceptions

print(librust_exceptions.divide(10, 2))  # Output: 5.0

try:
    print(librust_exceptions.divide(10, 0))
except ZeroDivisionError as e:
    print(f"Triggered Exception from Rust: {e}") # Output: division by zero
