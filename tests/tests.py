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
    """
    runner = Runner(source=program)
    output = runner.compile_and_run()

    print(output)

    assert output == ["5", "0", "1.23"] + [str(i) for i in range(1, 6)]


if __name__ == "__main__":
    test_variable_declaration()
