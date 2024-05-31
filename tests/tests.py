#!/usr/bin/env python3

from runner import Runner


def test_variable_declaration():
    print("Testing variable declaration...")
    program = """

    let a: int = 5;
    let b: colour = #000000;
    let c: float  = 1.23;

    let d: int[5] = [1, 2, 3, 4, 5];

    __print a;
    __print b;
    __print c;
    __print d;

    {
        let a: int = 1;
        __print a;
    }

    __print a;
    """

    runner = Runner(source=program)

    parir, result = runner.compile_and_run()

    # print(runner.parse())

    # print(parir)
    # print(result)

    assert result == ["5", "0", "1.23"] + ["1", "2", "3", "4", "5"] + ["1"] + ["5"]

    print("Variable declaration tests passed!")


def test_variable_assignment():
    print("Testing variable assignment...")
    program = """
    let a: int = 5;
    let b: colour = #000000;
    let c: float  = 1.23;
    let d: int[5] = [1, 2, 3, 4, 5];

    __print a;
    __print b;
    __print c;
    __print d[1];

    a = 2;
    b = #FFFFFF;
    c = 3.45;
    d[1] = 1;

    __print a;
    __print b;
    __print c;
    __print d[0];

    """

    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    # print(parir)
    # print(result)

    assert result == ["5", "0", "1.23", "2", "2", "16777215", "3.45", "1"]

    print("Variable assignment tests passed!")


def test_control_flows():
    program = """
    let loop_max: int = 5;

    for (let i: int = 0; i < loop_max; i = i + 1) {
        __print i;
    }

    if (1 < 2) {
        __print 1;
    } else {
        __print 0;
    }

    if (1 > 2) {
        __print 1;
    } else {
        __print 0;
    }


    let i: int = 0;
    while (i < loop_max) {
        __print i;
        i = i + 1;
    }
    """
    print("Testing control flows...")
    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    # print(parir)
    # print(result)

    assert result == list(map(str, range(5))) + ["1", "0"] + list(map(str, range(5)))
    print("Control flow tests passed!")


def test_functions():
    program = """
    fun add(a: int, b: int) -> int {
        return a + b;
    }

    let result: int = add(5, 10);

    __print result;

    fun some_func(a: int[2]) -> int[2] {
        let b: int[2] = [1,3];
        return b;
    }

    let a: int[2] = [1, 2];

    __print some_func(a);


    fun mixed_parameters(a: int, b: float, c: colour) -> bool {
        __print a;
        __print b;
        __print c;

        return true;
    }

    __print mixed_parameters(5, 1.23, #000000);

    fun array_parameters(a: int[5]) -> int {

        let sum: int = 0;

        for (let i: int = 0; i < 5; i = i + 1) {
            __print a[i];
            sum = sum + a[i];
        }

        return sum;
    }

    let arr: int[5] = [5, 1, 2, 3, 4];

    __print array_parameters(arr);
    """
    print("Testing functions...")
    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    # print(parir)
    print(result)

    expected = ["15"]
    expected += ["1", "3"]
    expected += ["5", "1.23", "0", "1"]
    expected += ["5", "1", "2", "3", "4"] + ["15"]

    assert result == expected
    print("Function tests passed!")


def test_expressions(debug=False):
    program = """
    let a: int = 5;
    let b: int = 10;

    __print (a + b) == 20;
    __print ((((a + b) * 2) / 3) - 1) == 9;
    __print (a + b) <= 20 and a - b <= 0;
    __print ((12 + 5) * (8 - 3) + (18 / 2)) - ((25 - 5) / (4 + 1));
    """

    print("Testing expressions...")
    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    expected = ["0", "1", "1", "90"]

    if debug:
        print(parir)
        print(result)

    assert result == expected


def test_pad_writing():
    program = """

    let c1: colour = #213B5F;
    let c2: colour = #FFA400;

    let x1: int = 5;
    let y1: int = 10;

    let x2: int = 20;
    let y2: int = 30;

    __write x1, y1, c1;

    __print __read x1, y1;

    __write_box x1, y1, x2, y2, c2;

    __print __read x1 + 3, y1 + 3;
    """

    print("Testing pad writing...")
    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    c1 = 0x213B5F
    assert result[0] == str(c1)

    c2 = 0xFFA400
    assert result[1] == str(c2)
    print("Pad writing tests passed!")


def if_statement_test():
    program = """

    let b: bool[2] = [true, false];

    for (let i: int = 0; i < 2; i = i + 1) {

        if (b[i]) {
            __print 111;
        } else {
            __print 1001;
        }

    }

    """

    print("Testing if statements...")
    runner = Runner(source=program)
    parir, result = runner.compile_and_run()

    expected = ["111", "1001"]
    assert result == expected
    print("If statement tests passed!")


def test_max_in_array():
    path = "samples/max_in_array.parl"
    runner = Runner(source_path=path)

    print("Testing max in array...")
    parir, result = runner.compile_and_run()

    assert result == ["120"]

    print("Max in array tests passed!")


if __name__ == "__main__":
    test_max_in_array()
    test_variable_declaration()
    test_variable_assignment()
    test_control_flows()
    test_functions()
    test_expressions()
    test_pad_writing()
    if_statement_test()
