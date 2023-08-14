#!/usr/bin/env python3
import libownership_pyrust

list_instance = libownership_pyrust.NumberList()
print("inserting two numbers: 5 and 10")
list_instance.add(5)
list_instance.add(10)

print("printing the list")
print(list_instance.length())  # Outputs: 2

print("clearing the list")
list_instance.clear_list()
print("printing the cleared list which should be empty")
print(list_instance.length())  # Outputs: 0
