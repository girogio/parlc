from runner import Runner


def test_variable_declaration():
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

    print(parir)
    print(result)

    # assert result == ["5", "0", "1.23"] + ["1", "2", "3", "4", "5"]


if __name__ == "__main__":
    test_variable_declaration()
