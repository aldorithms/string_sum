from python import Python

fn main() raises:
    let sum_string  = Python.import_module("string_sum")
    print("1 + 2 = " + sum_string.sum_as_string(1, 2)) 
    print("1 - 3 = " + sum_string.difference_as_string(1, 3))
    print("3 * 5 = " + sum_string.product_as_string(3, 5))
    print("8 / 2 = " + sum_string.quotient_as_string(8, 2))
    print("1 % 2 = " + sum_string.remainder_as_string(1, 2))